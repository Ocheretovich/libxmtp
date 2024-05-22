pub mod group_membership;
pub mod group_metadata;
pub mod group_mutable_metadata;
pub mod group_permissions;
mod intents;
mod members;
mod message_history;
mod subscriptions;
mod sync;
pub mod validated_commit;
#[allow(dead_code)]
mod validated_commit_v2;

use intents::SendMessageIntentData;
use openmls::{
    credentials::{BasicCredential, CredentialType},
    error::LibraryError,
    extensions::{
        Extension, ExtensionType, Extensions, Metadata, RequiredCapabilitiesExtension,
        UnknownExtension,
    },
    group::{CreateGroupContextExtProposalError, MlsGroupCreateConfig, MlsGroupJoinConfig},
    messages::proposals::ProposalType,
    prelude::{
        BasicCredentialError, Capabilities, CredentialWithKey, CryptoConfig,
        Error as TlsCodecError, GroupId, MlsGroup as OpenMlsGroup, StagedWelcome,
        Welcome as MlsWelcome, WireFormatPolicy,
    },
};
use openmls_traits::OpenMlsProvider;
use prost::Message;
use thiserror::Error;

use xmtp_cryptography::signature::{
    is_valid_ed25519_public_key, sanitize_evm_addresses, AddressValidationError,
};
use xmtp_proto::xmtp::mls::{
    api::v1::{
        group_message::{Version as GroupMessageVersion, V1 as GroupMessageV1},
        GroupMessage,
    },
    message_contents::{
        plaintext_envelope::{Content, V1},
        PlaintextEnvelope,
    },
};

use std::sync::Arc;

pub use self::group_permissions::PreconfiguredPolicies;
pub use self::intents::{AddressesOrInstallationIds, IntentError};
use self::{
    group_membership::GroupMembership,
    group_metadata::extract_group_metadata,
    group_mutable_metadata::{
        extract_group_mutable_metadata, GroupMutableMetadata, GroupMutableMetadataError,
        MetadataField,
    },
    group_permissions::{
        extract_group_permissions, GroupMutablePermissions, GroupMutablePermissionsError,
    },
    intents::{AdminListActionType, UpdateAdminListIntentData, UpdateMetadataIntentData},
};
use self::{
    group_metadata::{ConversationType, GroupMetadata, GroupMetadataError},
    group_permissions::PolicySet,
    intents::{AddMembersIntentData, RemoveMembersIntentData},
    message_history::MessageHistoryError,
    validated_commit::CommitValidationError,
};

use crate::{
    client::{deserialize_welcome, ClientError, MessageProcessingError, XmtpMlsLocalContext},
    configuration::{
        CIPHERSUITE, GROUP_MEMBERSHIP_EXTENSION_ID, GROUP_PERMISSIONS_EXTENSION_ID, MAX_GROUP_SIZE,
        MUTABLE_METADATA_EXTENSION_ID,
    },
    hpke::{decrypt_welcome, HpkeError},
    identity::v3::{Identity, IdentityError},
    retry::RetryableError,
    retryable,
    storage::{
        group::{GroupMembershipState, Purpose, StoredGroup},
        group_intent::{IntentKind, NewGroupIntent},
        group_message::{DeliveryStatus, GroupMessageKind, StoredGroupMessage},
        StorageError,
    },
    utils::{id::calculate_message_id, time::now_ns},
    xmtp_openmls_provider::XmtpOpenMlsProvider,
    Client, Store, XmtpApi,
};

#[derive(Debug, Error)]
pub enum GroupError {
    #[error("group not found")]
    GroupNotFound,
    #[error("Max user limit exceeded.")]
    UserLimitExceeded,
    #[error("api error: {0}")]
    Api(#[from] xmtp_proto::api_client::Error),
    #[error("storage error: {0}")]
    Storage(#[from] crate::storage::StorageError),
    #[error("intent error: {0}")]
    Intent(#[from] IntentError),
    #[error("create message: {0}")]
    CreateMessage(#[from] openmls::prelude::CreateMessageError),
    #[error("TLS Codec error: {0}")]
    TlsError(#[from] TlsCodecError),
    #[error("add members: {0}")]
    AddMembers(#[from] openmls::prelude::AddMembersError<StorageError>),
    #[error("remove members: {0}")]
    RemoveMembers(#[from] openmls::prelude::RemoveMembersError<StorageError>),
    #[error("group create: {0}")]
    GroupCreate(#[from] openmls::prelude::NewGroupError<StorageError>),
    #[error("self update: {0}")]
    SelfUpdate(#[from] openmls::group::SelfUpdateError<StorageError>),
    #[error("welcome error: {0}")]
    WelcomeError(#[from] openmls::prelude::WelcomeError<StorageError>),
    #[error("Invalid extension {0}")]
    InvalidExtension(#[from] openmls::prelude::InvalidExtensionError),
    #[error("Invalid signature: {0}")]
    Signature(#[from] openmls::prelude::SignatureError),
    #[error("client: {0}")]
    Client(#[from] ClientError),
    #[error("receive error: {0}")]
    ReceiveError(#[from] MessageProcessingError),
    #[error("Receive errors: {0:?}")]
    ReceiveErrors(Vec<MessageProcessingError>),
    #[error("generic: {0}")]
    Generic(String),
    #[error("diesel error {0}")]
    Diesel(#[from] diesel::result::Error),
    #[error(transparent)]
    AddressValidation(#[from] AddressValidationError),
    #[error("Public Keys {0:?} are not valid ed25519 public keys")]
    InvalidPublicKeys(Vec<Vec<u8>>),
    #[error("Commit validation error {0}")]
    CommitValidation(#[from] CommitValidationError),
    #[error("Metadata error {0}")]
    GroupMetadata(#[from] GroupMetadataError),
    #[error("Mutable Metadata error {0}")]
    GroupMutableMetadata(#[from] GroupMutableMetadataError),
    #[error("Mutable Permissions error {0}")]
    GroupMutablePermissions(#[from] GroupMutablePermissionsError),
    #[error("Errors occurred during sync {0:?}")]
    Sync(Vec<GroupError>),
    #[error("Hpke error: {0}")]
    Hpke(#[from] HpkeError),
    #[error("identity error: {0}")]
    Identity(#[from] IdentityError),
    #[error("serialization error: {0}")]
    EncodeError(#[from] prost::EncodeError),
    #[error("create group context proposal error: {0}")]
    CreateGroupContextExtProposalError(#[from] CreateGroupContextExtProposalError<StorageError>),
    #[error("Credential error")]
    CredentialError(#[from] BasicCredentialError),
    #[error("LeafNode error")]
    LeafNodeError(#[from] LibraryError),
    #[error("Message History error: {0}")]
    MessageHistory(#[from] MessageHistoryError),
}

impl RetryableError for GroupError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Diesel(diesel) => retryable!(diesel),
            Self::Storage(storage) => retryable!(storage),
            Self::ReceiveError(msg) => retryable!(msg),
            Self::AddMembers(members) => retryable!(members),
            Self::RemoveMembers(members) => retryable!(members),
            Self::GroupCreate(group) => retryable!(group),
            Self::SelfUpdate(update) => retryable!(update),
            Self::WelcomeError(welcome) => retryable!(welcome),
            _ => false,
        }
    }
}

pub struct MlsGroup {
    pub group_id: Vec<u8>,
    pub created_at_ns: i64,
    context: Arc<XmtpMlsLocalContext>,
}

impl Clone for MlsGroup {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            group_id: self.group_id.clone(),
            created_at_ns: self.created_at_ns,
        }
    }
}

impl MlsGroup {
    // Creates a new group instance. Does not validate that the group exists in the DB
    pub fn new(context: Arc<XmtpMlsLocalContext>, group_id: Vec<u8>, created_at_ns: i64) -> Self {
        Self {
            context,
            group_id,
            created_at_ns,
        }
    }

    // Load the stored MLS group from the OpenMLS provider's keystore
    fn load_mls_group(&self, provider: impl OpenMlsProvider) -> Result<OpenMlsGroup, GroupError> {
        let mls_group =
            OpenMlsGroup::load(&GroupId::from_slice(&self.group_id), provider.key_store())
                .ok_or(GroupError::GroupNotFound)?;

        Ok(mls_group)
    }

    // Create a new group and save it to the DB
    pub fn create_and_insert(
        context: Arc<XmtpMlsLocalContext>,
        membership_state: GroupMembershipState,
        permissions: Option<PreconfiguredPolicies>,
        added_by_address: String,
    ) -> Result<Self, GroupError> {
        let conn = context.store.conn()?;
        let provider = XmtpOpenMlsProvider::new(conn);
        let protected_metadata =
            build_protected_metadata_extension(&context.identity, Purpose::Conversation)?;
        let mutable_metadata = build_mutable_metadata_extension_default(&context.identity)?;
        let group_membership = build_starting_group_membership_extension(
            context.inbox_id(),
            context.inbox_latest_sequence_id(),
        );
        let mutable_permissions =
            build_mutable_permissions_extension(permissions.unwrap_or_default().to_policy_set())?;
        let group_config = build_group_config(
            protected_metadata,
            mutable_metadata,
            group_membership,
            mutable_permissions,
        )?;

        let mut mls_group = OpenMlsGroup::new(
            &provider,
            &context.identity.installation_keys,
            &group_config,
            CredentialWithKey {
                credential: context.identity.credential()?,
                signature_key: context.identity.installation_keys.to_public_vec().into(),
            },
        )?;
        mls_group.save(provider.key_store())?;

        let group_id = mls_group.group_id().to_vec();
        let stored_group = StoredGroup::new(
            group_id.clone(),
            now_ns(),
            membership_state,
            added_by_address.clone(),
        );

        stored_group.store(&provider.conn())?;
        Ok(Self::new(
            context.clone(),
            group_id,
            stored_group.created_at_ns,
        ))
    }

    // Create a group from a decrypted and decoded welcome message
    // If the group already exists in the store, overwrite the MLS state and do not update the group entry
    fn create_from_welcome(
        context: Arc<XmtpMlsLocalContext>,
        provider: &XmtpOpenMlsProvider,
        welcome: MlsWelcome,
        added_by_address: String,
    ) -> Result<Self, GroupError> {
        let mls_welcome =
            StagedWelcome::new_from_welcome(provider, &build_group_join_config(), welcome, None)?;

        let mut mls_group = mls_welcome.into_group(provider)?;
        mls_group.save(provider.key_store())?;
        let group_id = mls_group.group_id().to_vec();
        let metadata = extract_group_metadata(&mls_group)?;
        let group_type = metadata.conversation_type;

        let to_store = match group_type {
            ConversationType::Group | ConversationType::Dm => StoredGroup::new(
                group_id.clone(),
                now_ns(),
                GroupMembershipState::Pending,
                added_by_address.clone(),
            ),
            ConversationType::Sync => StoredGroup::new_sync_group(
                group_id.clone(),
                now_ns(),
                GroupMembershipState::Allowed,
            ),
        };

        let stored_group = provider.conn().insert_or_ignore_group(to_store)?;

        Ok(Self::new(
            context,
            stored_group.id,
            stored_group.created_at_ns,
        ))
    }

    // Decrypt a welcome message using HPKE and then create and save a group from the stored message
    pub fn create_from_encrypted_welcome(
        context: Arc<XmtpMlsLocalContext>,
        provider: &XmtpOpenMlsProvider,
        hpke_public_key: &[u8],
        encrypted_welcome_bytes: Vec<u8>,
    ) -> Result<Self, GroupError> {
        let welcome_bytes = decrypt_welcome(provider, hpke_public_key, &encrypted_welcome_bytes)?;

        let welcome = deserialize_welcome(&welcome_bytes)?;

        let join_config = build_group_join_config();
        let staged_welcome =
            StagedWelcome::new_from_welcome(provider, &join_config, welcome.clone(), None)?;

        let added_by_node = staged_welcome.welcome_sender()?;

        let added_by_credential = BasicCredential::try_from(added_by_node.credential())?;
        let pub_key_bytes = added_by_node.signature_key().as_slice();
        let account_address =
            Identity::get_validated_account_address(added_by_credential.identity(), pub_key_bytes)?;

        Self::create_from_welcome(context, provider, welcome, account_address)
    }

    pub(crate) fn create_and_insert_sync_group(
        context: Arc<XmtpMlsLocalContext>,
    ) -> Result<MlsGroup, GroupError> {
        let conn = context.store.conn()?;
        let provider = XmtpOpenMlsProvider::new(conn);
        let protected_metadata =
            build_protected_metadata_extension(&context.identity, Purpose::Sync)?;
        let mutable_metadata = build_mutable_metadata_extension_default(&context.identity)?;
        let group_membership = build_starting_group_membership_extension(
            context.inbox_id(),
            context.inbox_latest_sequence_id(),
        );
        let mutable_permissions =
            build_mutable_permissions_extension(PreconfiguredPolicies::default().to_policy_set())?;
        let group_config = build_group_config(
            protected_metadata,
            mutable_metadata,
            group_membership,
            mutable_permissions,
        )?;
        let mut mls_group = OpenMlsGroup::new(
            &provider,
            &context.identity.installation_keys,
            &group_config,
            CredentialWithKey {
                credential: context.identity.credential()?,
                signature_key: context.identity.installation_keys.to_public_vec().into(),
            },
        )?;
        mls_group.save(provider.key_store())?;

        let group_id = mls_group.group_id().to_vec();
        let stored_group =
            StoredGroup::new_sync_group(group_id.clone(), now_ns(), GroupMembershipState::Allowed);

        stored_group.store(&provider.conn())?;
        Ok(Self::new(
            context.clone(),
            stored_group.id,
            stored_group.created_at_ns,
        ))
    }

    pub async fn send_message<ApiClient>(
        &self,
        message: &[u8],
        client: &Client<ApiClient>,
    ) -> Result<Vec<u8>, GroupError>
    where
        ApiClient: XmtpApi,
    {
        let conn = self.context.store.conn()?;

        let update_interval = Some(5_000_000); // 5 seconds in nanoseconds
        self.maybe_update_installations(conn.clone(), update_interval, client)
            .await?;

        let now = now_ns();
        let plain_envelope = Self::into_envelope(message, &now.to_string());
        let mut encoded_envelope = vec![];
        plain_envelope
            .encode(&mut encoded_envelope)
            .map_err(GroupError::EncodeError)?;

        let intent_data: Vec<u8> = SendMessageIntentData::new(encoded_envelope).into();
        let intent =
            NewGroupIntent::new(IntentKind::SendMessage, self.group_id.clone(), intent_data);
        intent.store(&conn)?;

        // store this unpublished message locally before sending
        let message_id = calculate_message_id(
            &self.group_id,
            message,
            &self.context.account_address(),
            &now.to_string(),
        );
        let group_message = StoredGroupMessage {
            id: message_id.clone(),
            group_id: self.group_id.clone(),
            decrypted_message_bytes: message.to_vec(),
            sent_at_ns: now,
            kind: GroupMessageKind::Application,
            sender_installation_id: self.context.installation_public_key(),
            sender_account_address: self.context.account_address(),
            delivery_status: DeliveryStatus::Unpublished,
        };
        group_message.store(&conn)?;

        // Skipping a full sync here and instead just firing and forgetting
        if let Err(err) = self.publish_intents(conn, client).await {
            println!("error publishing intents: {:?}", err);
        }
        Ok(message_id)
    }

    fn into_envelope(encoded_msg: &[u8], idempotency_key: &str) -> PlaintextEnvelope {
        PlaintextEnvelope {
            content: Some(Content::V1(V1 {
                content: encoded_msg.to_vec(),
                idempotency_key: idempotency_key.into(),
            })),
        }
    }

    // Query the database for stored messages. Optionally filtered by time, kind, delivery_status
    // and limit
    pub fn find_messages(
        &self,
        kind: Option<GroupMessageKind>,
        sent_before_ns: Option<i64>,
        sent_after_ns: Option<i64>,
        delivery_status: Option<DeliveryStatus>,
        limit: Option<i64>,
    ) -> Result<Vec<StoredGroupMessage>, GroupError> {
        let conn = self.context.store.conn()?;
        let messages = conn.get_group_messages(
            &self.group_id,
            sent_after_ns,
            sent_before_ns,
            kind,
            delivery_status,
            limit,
        )?;

        Ok(messages)
    }

    pub async fn add_members<ApiClient>(
        &self,
        account_addresses_to_add: Vec<String>,
        client: &Client<ApiClient>,
    ) -> Result<(), GroupError>
    where
        ApiClient: XmtpApi,
    {
        let account_addresses = sanitize_evm_addresses(account_addresses_to_add)?;
        // get current number of users in group
        let member_count = self.members()?.len();
        if member_count + account_addresses.len() > MAX_GROUP_SIZE as usize {
            return Err(GroupError::UserLimitExceeded);
        }

        let conn = self.context.store.conn()?;
        let intent_data: Vec<u8> =
            AddMembersIntentData::new(account_addresses.into()).try_into()?;
        let intent = conn.insert_group_intent(NewGroupIntent::new(
            IntentKind::AddMembers,
            self.group_id.clone(),
            intent_data,
        ))?;

        self.sync_until_intent_resolved(conn, intent.id, client)
            .await
    }

    pub async fn add_members_by_installation_id<ApiClient>(
        &self,
        installation_ids: Vec<Vec<u8>>,
        client: &Client<ApiClient>,
    ) -> Result<(), GroupError>
    where
        ApiClient: XmtpApi,
    {
        validate_ed25519_keys(&installation_ids)?;
        let conn = self.context.store.conn()?;
        let intent_data: Vec<u8> = AddMembersIntentData::new(installation_ids.into()).try_into()?;
        let intent = conn.insert_group_intent(NewGroupIntent::new(
            IntentKind::AddMembers,
            self.group_id.clone(),
            intent_data,
        ))?;

        self.sync_until_intent_resolved(conn, intent.id, client)
            .await
    }

    pub async fn remove_members<ApiClient>(
        &self,
        account_addresses_to_remove: Vec<String>,
        client: &Client<ApiClient>,
    ) -> Result<(), GroupError>
    where
        ApiClient: XmtpApi,
    {
        let account_addresses = sanitize_evm_addresses(account_addresses_to_remove)?;
        let conn = self.context.store.conn()?;
        let intent_data: Vec<u8> = RemoveMembersIntentData::new(account_addresses.into()).into();
        let intent = conn.insert_group_intent(NewGroupIntent::new(
            IntentKind::RemoveMembers,
            self.group_id.clone(),
            intent_data,
        ))?;

        self.sync_until_intent_resolved(conn, intent.id, client)
            .await
    }

    pub async fn update_group_name<ApiClient>(
        &self,
        group_name: String,
        client: &Client<ApiClient>,
    ) -> Result<(), GroupError>
    where
        ApiClient: XmtpApi,
    {
        let conn = self.context.store.conn()?;
        let intent_data: Vec<u8> =
            UpdateMetadataIntentData::new_update_group_name(group_name).into();
        let intent = conn.insert_group_intent(NewGroupIntent::new(
            IntentKind::MetadataUpdate,
            self.group_id.clone(),
            intent_data,
        ))?;

        self.sync_until_intent_resolved(conn, intent.id, client)
            .await
    }

    pub fn group_name(&self) -> Result<String, GroupError> {
        let mutable_metadata = self.mutable_metadata()?;
        match mutable_metadata
            .attributes
            .get(&MetadataField::GroupName.to_string())
        {
            Some(group_name) => Ok(group_name.clone()),
            None => Err(GroupError::GroupMutableMetadata(
                GroupMutableMetadataError::MissingExtension,
            )),
        }
    }

    pub fn admin_list(&self) -> Result<Vec<String>, GroupError> {
        let mutable_metadata = self.mutable_metadata()?;
        Ok(mutable_metadata.admin_list)
    }

    pub fn super_admin_list(&self) -> Result<Vec<String>, GroupError> {
        let mutable_metadata = self.mutable_metadata()?;
        Ok(mutable_metadata.super_admin_list)
    }

    pub async fn add_admin<ApiClient>(
        &self,
        admin_address: String,
        client: &Client<ApiClient>,
    ) -> Result<(), GroupError>
    where
        ApiClient: XmtpApi,
    {
        let conn = self.context.store.conn()?;
        let intent_data: Vec<u8> =
            UpdateAdminListIntentData::new(intents::AdminListActionType::AddAdmin, admin_address)
                .into();
        let intent = conn.insert_group_intent(NewGroupIntent::new(
            IntentKind::AdminListUpdate,
            self.group_id.clone(),
            intent_data,
        ))?;

        self.sync_until_intent_resolved(conn, intent.id, client)
            .await
    }

    pub async fn remove_admin<ApiClient>(
        &self,
        admin_address: String,
        client: &Client<ApiClient>,
    ) -> Result<(), GroupError>
    where
        ApiClient: XmtpApi,
    {
        let conn = self.context.store.conn()?;
        let intent_data: Vec<u8> =
            UpdateAdminListIntentData::new(intents::AdminListActionType::RemoveAdmin, admin_address)
                .into();
        let intent = conn.insert_group_intent(NewGroupIntent::new(
            IntentKind::AdminListUpdate,
            self.group_id.clone(),
            intent_data,
        ))?;

        self.sync_until_intent_resolved(conn, intent.id, client)
            .await
    }

    // Find the wallet address of the group member who added the member to the group
    pub fn added_by_address(&self) -> Result<String, GroupError> {
        let conn = self.context.store.conn()?;
        conn.find_group(self.group_id.clone())
            .map_err(GroupError::from)
            .and_then(|fetch_result| {
                fetch_result
                    .map(|group| group.added_by_address.clone())
                    .ok_or_else(|| GroupError::GroupNotFound)
            })
    }

    // Used in tests
    #[allow(dead_code)]
    pub(crate) async fn remove_members_by_installation_id<ApiClient>(
        &self,
        installation_ids: Vec<Vec<u8>>,
        client: &Client<ApiClient>,
    ) -> Result<(), GroupError>
    where
        ApiClient: XmtpApi,
    {
        validate_ed25519_keys(&installation_ids)?;
        let conn = self.context.store.conn()?;
        let intent_data: Vec<u8> = RemoveMembersIntentData::new(installation_ids.into()).into();
        let intent = conn.insert_group_intent(NewGroupIntent::new(
            IntentKind::RemoveMembers,
            self.group_id.clone(),
            intent_data,
        ))?;

        self.sync_until_intent_resolved(conn, intent.id, client)
            .await
    }

    // Update this installation's leaf key in the group by creating a key update commit
    pub async fn key_update<ApiClient>(&self, client: &Client<ApiClient>) -> Result<(), GroupError>
    where
        ApiClient: XmtpApi,
    {
        let conn = self.context.store.conn()?;
        let intent = NewGroupIntent::new(IntentKind::KeyUpdate, self.group_id.clone(), vec![]);
        intent.store(&conn)?;

        self.sync_with_conn(conn, client).await
    }

    pub fn is_active(&self) -> Result<bool, GroupError> {
        let conn = self.context.store.conn()?;
        let provider = XmtpOpenMlsProvider::new(conn);
        let mls_group = self.load_mls_group(&provider)?;

        Ok(mls_group.is_active())
    }

    pub fn metadata(&self) -> Result<GroupMetadata, GroupError> {
        let conn = self.context.store.conn()?;
        let provider = XmtpOpenMlsProvider::new(conn);
        let mls_group = self.load_mls_group(&provider)?;

        Ok(extract_group_metadata(&mls_group)?)
    }

    pub fn mutable_metadata(&self) -> Result<GroupMutableMetadata, GroupError> {
        let conn = self.context.store.conn()?;
        let provider = XmtpOpenMlsProvider::new(conn);
        let mls_group = self.load_mls_group(&provider)?;

        Ok(extract_group_mutable_metadata(&mls_group)?)
    }

    pub fn permissions(&self) -> Result<GroupMutablePermissions, GroupError> {
        let conn = self.context.store.conn()?;
        let provider = XmtpOpenMlsProvider::new(conn);
        let mls_group = self.load_mls_group(&provider)?;

        Ok(extract_group_permissions(&mls_group)?)
    }
}

fn extract_message_v1(message: GroupMessage) -> Result<GroupMessageV1, MessageProcessingError> {
    match message.version {
        Some(GroupMessageVersion::V1(value)) => Ok(value),
        _ => Err(MessageProcessingError::InvalidPayload),
    }
}

pub fn extract_group_id(message: &GroupMessage) -> Result<Vec<u8>, MessageProcessingError> {
    match &message.version {
        Some(GroupMessageVersion::V1(value)) => Ok(value.group_id.clone()),
        _ => Err(MessageProcessingError::InvalidPayload),
    }
}

fn validate_ed25519_keys(keys: &[Vec<u8>]) -> Result<(), GroupError> {
    let mut invalid = keys
        .iter()
        .filter(|a| !is_valid_ed25519_public_key(a))
        .peekable();

    if invalid.peek().is_some() {
        return Err(GroupError::InvalidPublicKeys(
            invalid.map(Clone::clone).collect::<Vec<_>>(),
        ));
    }

    Ok(())
}

fn build_protected_metadata_extension(
    identity: &Identity,
    group_purpose: Purpose,
) -> Result<Extension, GroupError> {
    let group_type = match group_purpose {
        Purpose::Conversation => ConversationType::Group,
        Purpose::Sync => ConversationType::Sync,
    };
    let metadata = GroupMetadata::new(
        group_type,
        identity.account_address.clone(),
        // TODO: Remove me
        "inbox_id".to_string(),
    );
    let protected_metadata = Metadata::new(metadata.try_into()?);

    Ok(Extension::ImmutableMetadata(protected_metadata))
}

fn build_mutable_permissions_extension(policies: PolicySet) -> Result<Extension, GroupError> {
    let permissions: Vec<u8> = GroupMutablePermissions::new(policies).try_into()?;
    let unknown_gc_extension = UnknownExtension(permissions);

    Ok(Extension::Unknown(
        GROUP_PERMISSIONS_EXTENSION_ID,
        unknown_gc_extension,
    ))
}

pub fn build_mutable_metadata_extension_default(
    identity: &Identity,
) -> Result<Extension, GroupError> {
    let mutable_metadata: Vec<u8> =
        GroupMutableMetadata::new_default(&identity.account_address).try_into()?;
    let unknown_gc_extension = UnknownExtension(mutable_metadata);

    Ok(Extension::Unknown(
        MUTABLE_METADATA_EXTENSION_ID,
        unknown_gc_extension,
    ))
}

pub fn build_mutable_metadata_extensions(
    group: &OpenMlsGroup,
    field_name: String,
    field_value: String,
) -> Result<Extensions, GroupError> {
    let existing_metadata = extract_group_mutable_metadata(group)?;
    let mut attributes = existing_metadata.attributes.clone();
    attributes.insert(field_name, field_value);
    let new_mutable_metadata: Vec<u8> = GroupMutableMetadata::new(
        attributes,
        existing_metadata.admin_list.clone(),
        existing_metadata.super_admin_list.clone(),
    )
    .try_into()?;
    let unknown_gc_extension = UnknownExtension(new_mutable_metadata);
    let extension = Extension::Unknown(MUTABLE_METADATA_EXTENSION_ID, unknown_gc_extension);
    let mut extensions = group.extensions().clone();
    extensions.add_or_replace(extension);
    Ok(extensions)
}

pub fn build_mutable_metadata_extensions_for_admin_lists_update(
    group: &OpenMlsGroup,
    admin_lists_update: UpdateAdminListIntentData,
) -> Result<Extensions, GroupError> {
    let existing_metadata = extract_group_mutable_metadata(group)?;
    let attributes = existing_metadata.attributes.clone();
    let mut admin_list = existing_metadata.admin_list;
    let mut super_admin_list = existing_metadata.super_admin_list;
    match admin_lists_update.action_type {
        AdminListActionType::AddAdmin => {
            if !admin_list.contains(&admin_lists_update.inbox_id) {
                admin_list.push(admin_lists_update.inbox_id);
            }
        }
        AdminListActionType::RemoveAdmin => {
            admin_list.retain(|x| x != &admin_lists_update.inbox_id)
        }
        AdminListActionType::AddSuperAdmin => {
            if !super_admin_list.contains(&admin_lists_update.inbox_id) {
                super_admin_list.push(admin_lists_update.inbox_id);
            }
        }
        AdminListActionType::RemoveSuperAdmin => {
            admin_list.retain(|x| x != &admin_lists_update.inbox_id)
        }
    }
    let new_mutable_metadata: Vec<u8> =
        GroupMutableMetadata::new(attributes, admin_list, super_admin_list).try_into()?;
    let unknown_gc_extension = UnknownExtension(new_mutable_metadata);
    let extension = Extension::Unknown(MUTABLE_METADATA_EXTENSION_ID, unknown_gc_extension);
    let mut extensions = group.extensions().clone();
    extensions.add_or_replace(extension);
    Ok(extensions)
}

pub fn build_starting_group_membership_extension(inbox_id: String, sequence_id: u64) -> Extension {
    let mut group_membership = GroupMembership::new();
    group_membership.add(inbox_id, sequence_id);
    let unknown_gc_extension = UnknownExtension(group_membership.into());

    Extension::Unknown(GROUP_MEMBERSHIP_EXTENSION_ID, unknown_gc_extension)
}

fn build_group_config(
    protected_metadata_extension: Extension,
    mutable_metadata_extension: Extension,
    group_membership_extension: Extension,
    mutable_permission_extension: Extension,
) -> Result<MlsGroupCreateConfig, GroupError> {
    let required_extension_types = &[
        ExtensionType::Unknown(GROUP_MEMBERSHIP_EXTENSION_ID),
        ExtensionType::Unknown(MUTABLE_METADATA_EXTENSION_ID),
        ExtensionType::Unknown(GROUP_PERMISSIONS_EXTENSION_ID),
        ExtensionType::ImmutableMetadata,
        ExtensionType::LastResort,
        ExtensionType::ApplicationId,
    ];

    let required_proposal_types = &[ProposalType::GroupContextExtensions];

    let capabilities = Capabilities::new(
        None,
        None,
        Some(required_extension_types),
        Some(required_proposal_types),
        None,
    );
    let credentials = &[CredentialType::Basic];

    let required_capabilities =
        Extension::RequiredCapabilities(RequiredCapabilitiesExtension::new(
            required_extension_types,
            required_proposal_types,
            credentials,
        ));

    let extensions = Extensions::from_vec(vec![
        protected_metadata_extension,
        mutable_metadata_extension,
        group_membership_extension,
        mutable_permission_extension,
        required_capabilities,
    ])?;

    Ok(MlsGroupCreateConfig::builder()
        .with_group_context_extensions(extensions)?
        .capabilities(capabilities)
        .crypto_config(CryptoConfig::with_default_version(CIPHERSUITE))
        .wire_format_policy(WireFormatPolicy::default())
        .max_past_epochs(3) // Trying with 3 max past epochs for now
        .use_ratchet_tree_extension(true)
        .build())
}

fn build_group_join_config() -> MlsGroupJoinConfig {
    MlsGroupJoinConfig::builder()
        .wire_format_policy(WireFormatPolicy::default())
        .max_past_epochs(3) // Trying with 3 max past epochs for now
        .use_ratchet_tree_extension(true)
        .build()
}

#[cfg(test)]
mod tests {
    use openmls::prelude::Member;
    use prost::Message;
    use xmtp_cryptography::utils::generate_local_wallet;
    use xmtp_proto::xmtp::mls::message_contents::EncodedContent;

    use crate::{
        builder::ClientBuilder,
        codecs::{membership_change::GroupMembershipChangeCodec, ContentCodec},
        groups::{group_mutable_metadata::MetadataField, PreconfiguredPolicies},
        storage::{
            group_intent::IntentState,
            group_message::{GroupMessageKind, StoredGroupMessage},
        },
        Client, InboxOwner, XmtpApi,
    };

    use super::MlsGroup;

    async fn receive_group_invite<ApiClient>(client: &Client<ApiClient>) -> MlsGroup
    where
        ApiClient: XmtpApi,
    {
        client.sync_welcomes().await.unwrap();
        let mut groups = client.find_groups(None, None, None, None).unwrap();

        groups.remove(0)
    }

    async fn get_latest_message<ApiClient>(
        group: &MlsGroup,
        client: &Client<ApiClient>,
    ) -> StoredGroupMessage
    where
        ApiClient: XmtpApi,
    {
        group.sync(client).await.unwrap();
        let mut messages = group.find_messages(None, None, None, None, None).unwrap();

        messages.pop().unwrap()
    }

    #[tokio::test]
    async fn test_send_message() {
        let wallet = generate_local_wallet();
        let client = ClientBuilder::new_test_client(&wallet).await;
        let group = client.create_group(None).expect("create group");
        group
            .send_message(b"hello", &client)
            .await
            .expect("send message");

        let messages = client
            .api_client
            .query_group_messages(group.group_id, None)
            .await
            .expect("read topic");

        assert_eq!(messages.len(), 1)
    }

    #[tokio::test]
    async fn test_receive_self_message() {
        let wallet = generate_local_wallet();
        let client = ClientBuilder::new_test_client(&wallet).await;
        let group = client.create_group(None).expect("create group");
        let msg = b"hello";
        group
            .send_message(msg, &client)
            .await
            .expect("send message");

        group
            .receive(&client.store().conn().unwrap(), &client)
            .await
            .unwrap();
        // Check for messages
        // println!("HERE: {:#?}", messages);
        let messages = group.find_messages(None, None, None, None, None).unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages.first().unwrap().decrypted_message_bytes, msg);
    }

    // Amal and Bola will both try and add Charlie from the same epoch.
    // The group should resolve to a consistent state
    #[tokio::test]
    async fn test_add_member_conflict() {
        let amal = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let bola = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let charlie = ClientBuilder::new_test_client(&generate_local_wallet()).await;

        let amal_group = amal.create_group(None).unwrap();
        // Add bola
        amal_group
            .add_members_by_installation_id(vec![bola.installation_public_key()], &amal)
            .await
            .unwrap();

        // Get bola's version of the same group
        let bola_groups = bola.sync_welcomes().await.unwrap();
        let bola_group = bola_groups.first().unwrap();

        // Have amal and bola both invite charlie.
        amal_group
            .add_members_by_installation_id(vec![charlie.installation_public_key()], &amal)
            .await
            .expect("failed to add charlie");
        bola_group
            .add_members_by_installation_id(vec![charlie.installation_public_key()], &bola)
            .await
            .expect_err("expected err");

        amal_group
            .receive(&amal.store().conn().unwrap(), &amal)
            .await
            .expect_err("expected error");

        // Check Amal's MLS group state.
        let amal_db = amal.context.store.conn().unwrap();
        let amal_mls_group = amal_group
            .load_mls_group(&amal.mls_provider(amal_db.clone()))
            .unwrap();
        let amal_members: Vec<Member> = amal_mls_group.members().collect();
        assert_eq!(amal_members.len(), 3);

        // Check Bola's MLS group state.
        let bola_db = bola.context.store.conn().unwrap();
        let bola_mls_group = bola_group
            .load_mls_group(&bola.mls_provider(bola_db.clone()))
            .unwrap();
        let bola_members: Vec<Member> = bola_mls_group.members().collect();
        assert_eq!(bola_members.len(), 3);

        let amal_uncommitted_intents = amal_db
            .find_group_intents(
                amal_group.group_id.clone(),
                Some(vec![IntentState::ToPublish, IntentState::Published]),
                None,
            )
            .unwrap();
        assert_eq!(amal_uncommitted_intents.len(), 0);

        let bola_uncommitted_intents = bola_db
            .find_group_intents(
                bola_group.group_id.clone(),
                Some(vec![IntentState::ToPublish, IntentState::Published]),
                None,
            )
            .unwrap();
        // Bola should have one uncommitted intent for the failed attempt at adding Charlie, who is already in the group
        assert_eq!(bola_uncommitted_intents.len(), 1);
    }

    #[tokio::test]
    async fn test_add_installation() {
        let client = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let client_2 = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let group = client.create_group(None).expect("create group");

        group
            .add_members_by_installation_id(vec![client_2.installation_public_key()], &client)
            .await
            .unwrap();

        let group_id = group.group_id;

        let messages = client
            .api_client
            .query_group_messages(group_id, None)
            .await
            .unwrap();

        assert_eq!(messages.len(), 1);
    }

    #[tokio::test]
    async fn test_add_invalid_member() {
        let client = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let group = client.create_group(None).expect("create group");

        let result = group
            .add_members_by_installation_id(vec![b"1234".to_vec()], &client)
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_unregistered_member() {
        let amal = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let unconnected_wallet_address = generate_local_wallet().get_address();
        let group = amal.create_group(None).unwrap();
        let result = group
            .add_members(vec![unconnected_wallet_address], &amal)
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_installation() {
        let client_1 = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        // Add another client onto the network
        let client_2 = ClientBuilder::new_test_client(&generate_local_wallet()).await;

        let group = client_1.create_group(None).expect("create group");
        group
            .add_members_by_installation_id(vec![client_2.installation_public_key()], &client_1)
            .await
            .expect("group create failure");

        let messages_with_add = group.find_messages(None, None, None, None, None).unwrap();
        assert_eq!(messages_with_add.len(), 1);

        // Try and add another member without merging the pending commit
        group
            .remove_members_by_installation_id(vec![client_2.installation_public_key()], &client_1)
            .await
            .expect("group create failure");

        let messages_with_remove = group.find_messages(None, None, None, None, None).unwrap();
        assert_eq!(messages_with_remove.len(), 2);

        // We are expecting 1 message on the group topic, not 2, because the second one should have
        // failed
        let group_id = group.group_id;
        let messages = client_1
            .api_client
            .query_group_messages(group_id, None)
            .await
            .expect("read topic");

        assert_eq!(messages.len(), 2);
    }

    #[tokio::test]
    async fn test_key_update() {
        let client = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let bola_client = ClientBuilder::new_test_client(&generate_local_wallet()).await;

        let group = client.create_group(None).expect("create group");
        group
            .add_members(vec![bola_client.account_address()], &client)
            .await
            .unwrap();

        group.key_update(&client).await.unwrap();

        let messages = client
            .api_client
            .query_group_messages(group.group_id.clone(), None)
            .await
            .unwrap();
        assert_eq!(messages.len(), 2);

        let conn = &client.context.store.conn().unwrap();
        let provider = super::XmtpOpenMlsProvider::new(conn.clone());
        let mls_group = group.load_mls_group(&provider).unwrap();
        let pending_commit = mls_group.pending_commit();
        assert!(pending_commit.is_none());

        group
            .send_message(b"hello", &client)
            .await
            .expect("send message");

        bola_client.sync_welcomes().await.unwrap();
        let bola_groups = bola_client.find_groups(None, None, None, None).unwrap();
        let bola_group = bola_groups.first().unwrap();
        bola_group.sync(&bola_client).await.unwrap();
        let bola_messages = bola_group
            .find_messages(None, None, None, None, None)
            .unwrap();
        assert_eq!(bola_messages.len(), 1);
    }

    #[tokio::test]
    async fn test_post_commit() {
        let client = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let client_2 = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let group = client.create_group(None).expect("create group");

        group
            .add_members_by_installation_id(vec![client_2.installation_public_key()], &client)
            .await
            .unwrap();

        // Check if the welcome was actually sent
        let welcome_messages = client
            .api_client
            .query_welcome_messages(client_2.installation_public_key(), None)
            .await
            .unwrap();

        assert_eq!(welcome_messages.len(), 1);
    }

    #[tokio::test]
    async fn test_remove_by_account_address() {
        let amal = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let bola = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let charlie = ClientBuilder::new_test_client(&generate_local_wallet()).await;

        let group = amal.create_group(None).unwrap();
        group
            .add_members(
                vec![bola.account_address(), charlie.account_address()],
                &amal,
            )
            .await
            .unwrap();
        assert_eq!(group.members().unwrap().len(), 3);
        let messages = group.find_messages(None, None, None, None, None).unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].kind, GroupMessageKind::MembershipChange);
        let encoded_content =
            EncodedContent::decode(messages[0].decrypted_message_bytes.as_slice()).unwrap();
        let members_changed_codec = GroupMembershipChangeCodec::decode(encoded_content).unwrap();
        assert_eq!(members_changed_codec.members_added.len(), 2);
        assert_eq!(members_changed_codec.members_removed.len(), 0);
        assert_eq!(members_changed_codec.installations_added.len(), 0);
        assert_eq!(members_changed_codec.installations_removed.len(), 0);

        group
            .remove_members(vec![bola.account_address()], &amal)
            .await
            .unwrap();
        assert_eq!(group.members().unwrap().len(), 2);
        let messages = group.find_messages(None, None, None, None, None).unwrap();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[1].kind, GroupMessageKind::MembershipChange);
        let encoded_content =
            EncodedContent::decode(messages[1].decrypted_message_bytes.as_slice()).unwrap();
        let members_changed_codec = GroupMembershipChangeCodec::decode(encoded_content).unwrap();
        assert_eq!(members_changed_codec.members_added.len(), 0);
        assert_eq!(members_changed_codec.members_removed.len(), 1);
        assert_eq!(members_changed_codec.installations_added.len(), 0);
        assert_eq!(members_changed_codec.installations_removed.len(), 0);

        let bola_group = receive_group_invite(&bola).await;
        bola_group.sync(&bola).await.unwrap();
        assert!(!bola_group.is_active().unwrap())
    }

    #[tokio::test]
    async fn test_get_missing_members() {
        // Setup for test
        let amal_wallet = generate_local_wallet();
        let amal = ClientBuilder::new_test_client(&amal_wallet).await;
        let bola = ClientBuilder::new_test_client(&generate_local_wallet()).await;

        let group = amal.create_group(None).unwrap();
        group
            .add_members(vec![bola.account_address()], &amal)
            .await
            .unwrap();
        assert_eq!(group.members().unwrap().len(), 2);

        let conn = &amal.context.store.conn().unwrap();
        let provider = super::XmtpOpenMlsProvider::new(conn.clone());
        // Finished with setup

        let (noone_to_add, _placeholder) =
            group.get_missing_members(&provider, &amal).await.unwrap();
        assert_eq!(noone_to_add.len(), 0);
        assert_eq!(_placeholder.len(), 0);

        // Add a second installation for amal using the same wallet
        let _amal_2nd = ClientBuilder::new_test_client(&amal_wallet).await;

        // Here we should find a new installation
        let (missing_members, _placeholder) =
            group.get_missing_members(&provider, &amal).await.unwrap();
        assert_eq!(missing_members.len(), 1);
        assert_eq!(_placeholder.len(), 0);

        let _result = group
            .add_members_by_installation_id(missing_members, &amal)
            .await;

        // After we added the new installation the list should again be empty
        let (missing_members, _placeholder) =
            group.get_missing_members(&provider, &amal).await.unwrap();
        assert_eq!(missing_members.len(), 0);
        assert_eq!(_placeholder.len(), 0);
    }

    #[tokio::test]
    async fn test_add_missing_installations() {
        // Setup for test
        let amal_wallet = generate_local_wallet();
        let amal = ClientBuilder::new_test_client(&amal_wallet).await;
        let bola = ClientBuilder::new_test_client(&generate_local_wallet()).await;

        let group = amal.create_group(None).unwrap();
        group
            .add_members(vec![bola.account_address()], &amal)
            .await
            .unwrap();
        assert_eq!(group.members().unwrap().len(), 2);

        let conn = &amal.context.store.conn().unwrap();
        let provider = super::XmtpOpenMlsProvider::new(conn.clone());
        // Finished with setup

        // add a second installation for amal using the same wallet
        let _amal_2nd = ClientBuilder::new_test_client(&amal_wallet).await;

        // test if adding the new installation(s) worked
        let new_installations_were_added = group.add_missing_installations(provider, &amal).await;
        assert!(new_installations_were_added.is_ok());
    }

    #[tokio::test]
    async fn test_self_resolve_epoch_mismatch() {
        let amal = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let bola = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let charlie = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let dave = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let amal_group = amal.create_group(None).unwrap();
        // Add bola to the group
        amal_group
            .add_members(vec![bola.account_address()], &amal)
            .await
            .unwrap();

        let bola_group = receive_group_invite(&bola).await;
        bola_group.sync(&bola).await.unwrap();
        // Both Amal and Bola are up to date on the group state. Now each of them want to add someone else
        amal_group
            .add_members(vec![charlie.account_address()], &amal)
            .await
            .unwrap();

        bola_group
            .add_members(vec![dave.account_address()], &bola)
            .await
            .unwrap();

        // Send a message to the group, now that everyone is invited
        amal_group.sync(&amal).await.unwrap();
        amal_group.send_message(b"hello", &amal).await.unwrap();

        let charlie_group = receive_group_invite(&charlie).await;
        let dave_group = receive_group_invite(&dave).await;

        let (amal_latest_message, bola_latest_message, charlie_latest_message, dave_latest_message) = tokio::join!(
            get_latest_message(&amal_group, &amal),
            get_latest_message(&bola_group, &bola),
            get_latest_message(&charlie_group, &charlie),
            get_latest_message(&dave_group, &dave)
        );

        let expected_latest_message = b"hello".to_vec();
        assert!(expected_latest_message.eq(&amal_latest_message.decrypted_message_bytes));
        assert!(expected_latest_message.eq(&bola_latest_message.decrypted_message_bytes));
        assert!(expected_latest_message.eq(&charlie_latest_message.decrypted_message_bytes));
        assert!(expected_latest_message.eq(&dave_latest_message.decrypted_message_bytes));
    }

    #[tokio::test]
    async fn test_group_permissions() {
        let amal = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let bola = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let charlie = ClientBuilder::new_test_client(&generate_local_wallet()).await;

        let amal_group = amal
            .create_group(Some(PreconfiguredPolicies::AdminsOnly))
            .unwrap();
        // Add bola to the group
        amal_group
            .add_members(vec![bola.account_address()], &amal)
            .await
            .unwrap();

        let bola_group = receive_group_invite(&bola).await;
        bola_group.sync(&bola).await.unwrap();
        assert!(bola_group
            .add_members(vec![charlie.account_address()], &bola)
            .await
            .is_err(),);
    }

    #[tokio::test]
    async fn test_max_limit_add() {
        let amal = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let amal_group = amal
            .create_group(Some(PreconfiguredPolicies::AdminsOnly))
            .unwrap();
        let mut clients = Vec::new();
        for _ in 0..249 {
            let client: Client<_> = ClientBuilder::new_test_client(&generate_local_wallet()).await;
            clients.push(client.account_address());
        }
        amal_group.add_members(clients, &amal).await.unwrap();
        let bola = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        assert!(amal_group
            .add_members(vec![bola.account_address()], &amal)
            .await
            .is_err(),);
    }

    #[tokio::test]
    async fn test_group_mutable_data() {
        let amal = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let bola = ClientBuilder::new_test_client(&generate_local_wallet()).await;

        // Create a group and verify it has the default group name
        let policies = Some(PreconfiguredPolicies::AdminsOnly);
        let amal_group: MlsGroup = amal.create_group(policies).unwrap();
        amal_group.sync(&amal).await.unwrap();

        let group_mutable_metadata = amal_group.mutable_metadata().unwrap();
        assert!(group_mutable_metadata.attributes.len().eq(&2));
        assert!(group_mutable_metadata
            .attributes
            .get(&MetadataField::GroupName.to_string())
            .unwrap()
            .eq("New Group"));

        // Add bola to the group
        amal_group
            .add_members(vec![bola.account_address()], &amal)
            .await
            .unwrap();
        bola.sync_welcomes().await.unwrap();
        let bola_groups = bola.find_groups(None, None, None, None).unwrap();
        assert_eq!(bola_groups.len(), 1);
        let bola_group = bola_groups.first().unwrap();
        bola_group.sync(&bola).await.unwrap();
        let group_mutable_metadata = bola_group.mutable_metadata().unwrap();
        assert!(group_mutable_metadata
            .attributes
            .get(&MetadataField::GroupName.to_string())
            .unwrap()
            .eq("New Group"));

        // Update group name
        amal_group
            .update_group_name("New Group Name 1".to_string(), &amal)
            .await
            .unwrap();

        // Verify amal group sees update
        amal_group.sync(&amal).await.unwrap();
        let binding = amal_group.mutable_metadata().expect("msg");
        let amal_group_name: &String = binding
            .attributes
            .get(&MetadataField::GroupName.to_string())
            .unwrap();
        assert_eq!(amal_group_name, "New Group Name 1");

        // Verify bola group sees update
        bola_group.sync(&bola).await.unwrap();
        let binding = bola_group.mutable_metadata().expect("msg");
        let bola_group_name: &String = binding
            .attributes
            .get(&MetadataField::GroupName.to_string())
            .unwrap();
        assert_eq!(bola_group_name, "New Group Name 1");

        // Verify that bola can not update the group name since they are not the creator
        bola_group
            .update_group_name("New Group Name 2".to_string(), &bola)
            .await
            .expect_err("expected err");

        // Verify bola group does not see an update
        bola_group.sync(&bola).await.unwrap();
        let binding = bola_group.mutable_metadata().expect("msg");
        let bola_group_name: &String = binding
            .attributes
            .get(&MetadataField::GroupName.to_string())
            .unwrap();
        assert_eq!(bola_group_name, "New Group Name 1");
    }

    #[tokio::test]
    async fn test_group_mutable_data_group_permissions() {
        let amal = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let bola = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let caro = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let dude = ClientBuilder::new_test_client(&generate_local_wallet()).await;

        // Create a group and verify admin list and super admin list both contain creator
        let policies = Some(PreconfiguredPolicies::AdminsOnly);
        let amal_group: MlsGroup = amal.create_group(policies).unwrap();
        amal_group.sync(&amal).await.unwrap();

        let group_mutable_metadata = amal_group.mutable_metadata().unwrap();
        assert!(group_mutable_metadata.admin_list.len() == 1);
        assert!(group_mutable_metadata
            .admin_list
            .contains(&amal.account_address()));
        assert!(group_mutable_metadata.super_admin_list.len() == 1);
        assert!(group_mutable_metadata
            .super_admin_list
            .contains(&amal.account_address()));

        // Add bola to the group, assert they can read admin list
        amal_group
            .add_members(vec![bola.account_address()], &amal)
            .await
            .unwrap();
        bola.sync_welcomes().await.unwrap();
        let bola_groups = bola.find_groups(None, None, None, None).unwrap();
        assert_eq!(bola_groups.len(), 1);
        let bola_group: &MlsGroup = bola_groups.first().unwrap();
        bola_group.sync(&bola).await.unwrap();
        let bola_mutable_metadata = bola_group.mutable_metadata().unwrap();
        assert!(bola_mutable_metadata.admin_list.len() == 1);
        assert!(bola_mutable_metadata
            .admin_list
            .contains(&amal.account_address()));
        assert!(bola_mutable_metadata.super_admin_list.len() == 1);
        assert!(bola_mutable_metadata
            .super_admin_list
            .contains(&amal.account_address()));

        // Verify that bola can not add a member because group is admin only
        bola_group
            .add_members(vec![caro.account_address()], &bola)
            .await
            .expect_err("expected err");
        bola_group.sync(&bola).await.unwrap();
        amal_group.sync(&bola).await.unwrap();
        assert!(amal_group.members().unwrap().len() == 2);
        assert!(bola_group.members().unwrap().len() == 2);

        // Verify bola can not add themselves as admin
        bola_group
            .add_admin(bola.account_address(), &bola)
            .await
            .expect_err("expected error");
        bola_group.sync(&bola).await.unwrap();
        let bola_mutable_metadata = bola_group.mutable_metadata().unwrap();
        assert!(!bola_mutable_metadata
            .admin_list
            .contains(&bola.account_address()));

        // Add bola as an admin
        amal_group
            .add_admin(bola.account_address(), &amal)
            .await
            .expect("error adding admin");
        bola_group.sync(&bola).await.unwrap();
        let bola_mutable_metadata = bola_group.mutable_metadata().unwrap();
        assert!(bola_mutable_metadata
            .admin_list
            .contains(&bola.account_address()));

        // Verify that bola can now add members, now that they are an admin
        bola_group
            .add_members(vec![caro.account_address()], &bola)
            .await
            .expect("admin should be able to add members");
        bola_group.sync(&bola).await.unwrap();
        amal_group.sync(&amal).await.unwrap();
        assert!(amal_group.members().unwrap().len() == 3);
        assert!(bola_group.members().unwrap().len() == 3);

        // Verify that bola can now update group name, now that they are an admin
        bola_group
            .update_group_name("New group name".to_string(), &bola)
            .await
            .expect("admin should be able to update metadata group name");
        bola_group.sync(&bola).await.unwrap();
        amal_group.sync(&amal).await.unwrap();
        assert!(amal_group.group_name().unwrap().eq("New group name"));
        assert!(bola_group.group_name().unwrap().eq("New group name"));

        // Remove bola as an admin
        amal_group
            .remove_admin(bola.account_address(), &amal)
            .await
            .expect("error removing admin");
        bola_group.sync(&bola).await.unwrap();
        let bola_mutable_metadata = bola_group.mutable_metadata().unwrap();
        assert!(!bola_mutable_metadata
            .admin_list
            .contains(&bola.account_address()));

        // Verify that bola can not add a member because group is admin only
        bola_group
            .add_members(vec![dude.account_address()], &bola)
            .await
            .expect_err("expected err");
        bola_group.sync(&bola).await.unwrap();
        amal_group.sync(&bola).await.unwrap();
        assert!(amal_group.members().unwrap().len() == 3);
        assert!(bola_group.members().unwrap().len() == 3);
    }

    #[tokio::test]
    async fn test_staged_welcome() {
        // Create Clients
        let amal = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let bola = ClientBuilder::new_test_client(&generate_local_wallet()).await;

        // Amal creates a group
        let amal_group = amal.create_group(None).unwrap();

        // Amal adds Bola to the group
        amal_group
            .add_members_by_installation_id(vec![bola.installation_public_key()], &amal)
            .await
            .unwrap();

        // Bola syncs groups - this will decrypt the Welcome, identify who added Bola
        // and then store that value on the group and insert into the database
        let bola_groups = bola.sync_welcomes().await.unwrap();

        // Bola gets the group id. This will be needed to fetch the group from
        // the database.
        let bola_group = bola_groups.first().unwrap();
        let bola_group_id = bola_group.group_id.clone();

        // Bola fetches group from the database
        let bola_fetched_group = bola.group(bola_group_id).unwrap();

        // Check Bola's group for the added_by_address of the inviter
        let added_by_address = bola_fetched_group.added_by_address().unwrap();

        // Verify the welcome host_credential is equal to Amal's
        assert_eq!(
            amal.account_address(),
            added_by_address,
            "The Inviter and added_by_address do not match!"
        );
    }
}
