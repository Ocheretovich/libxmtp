use std::{collections::HashMap, mem::Discriminant, sync::Arc};

use openmls::{
    credentials::errors::BasicCredentialError,
    framing::{MlsMessageBodyIn, MlsMessageIn},
    group::GroupEpoch,
    messages::Welcome,
    prelude::tls_codec::{Deserialize, Error as TlsCodecError, Serialize},
};
use openmls_traits::OpenMlsProvider;
use prost::EncodeError;
use thiserror::Error;

use xmtp_cryptography::signature::{sanitize_evm_addresses, AddressValidationError};
use xmtp_id::{
    associations::{
        builder::{SignatureRequest, SignatureRequestError},
        AssociationError,
    },
    InboxId,
};

use xmtp_proto::xmtp::mls::api::v1::{
    welcome_message::{Version as WelcomeMessageVersion, V1 as WelcomeMessageV1},
    GroupMessage, WelcomeMessage,
};

use crate::{
    api::ApiClientWrapper,
    groups::{
        validated_commit::CommitValidationError, IntentError, MlsGroup, PreconfiguredPolicies,
    },
    identity::{parse_credential, Identity, IdentityError},
    identity_updates::IdentityUpdateError,
    storage::{
        db_connection::DbConnection,
        group::{GroupMembershipState, StoredGroup},
        refresh_state::EntityKind,
        sql_key_store, EncryptedMessageStore, StorageError,
    },
    verified_key_package_v2::{KeyPackageVerificationError, VerifiedKeyPackageV2},
    xmtp_openmls_provider::XmtpOpenMlsProvider,
    Fetch, XmtpApi,
};

/// Which network the Client is connected to
#[derive(Clone, Copy, Default, Debug)]
pub enum Network {
    Local(&'static str),
    #[default]
    Dev,
    Prod,
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error(transparent)]
    AddressValidation(#[from] AddressValidationError),
    #[error("could not publish: {0}")]
    PublishError(String),
    #[error("storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("dieselError: {0}")]
    Diesel(#[from] diesel::result::Error),
    #[error("Query failed: {0}")]
    QueryError(#[from] xmtp_proto::api_client::Error),
    #[error("API error: {0}")]
    Api(#[from] crate::api::WrappedApiError),
    #[error("identity error: {0}")]
    Identity(#[from] crate::identity::IdentityError),
    #[error("TLS Codec error: {0}")]
    TlsError(#[from] TlsCodecError),
    #[error("key package verification: {0}")]
    KeyPackageVerification(#[from] KeyPackageVerificationError),
    #[error("syncing errors: {0:?}")]
    SyncingError(Vec<MessageProcessingError>),
    #[error("Stream inconsistency error: {0}")]
    StreamInconsistency(String),
    #[error("Association error: {0}")]
    Association(#[from] AssociationError),
    #[error(transparent)]
    IdentityUpdate(#[from] IdentityUpdateError),
    #[error(transparent)]
    SignatureRequest(#[from] SignatureRequestError),
    #[error("generic:{0}")]
    Generic(String),
}

/// An enum of errors that can occur when reading and processing a message off the network
#[derive(Debug, Error)]
pub enum MessageProcessingError {
    #[error("[{0}] already processed")]
    AlreadyProcessed(u64),
    #[error("diesel error: {0}")]
    Diesel(#[from] diesel::result::Error),
    #[error("[{message_time_ns:?}] invalid sender with credential: {credential:?}")]
    InvalidSender {
        message_time_ns: u64,
        credential: Vec<u8>,
    },
    #[error("invalid payload")]
    InvalidPayload,
    #[error(transparent)]
    Identity(#[from] IdentityError),
    #[error("openmls process message error: {0}")]
    OpenMlsProcessMessage(
        #[from] openmls::prelude::ProcessMessageError<sql_key_store::MemoryStorageError>,
    ),
    #[error("merge pending commit: {0}")]
    MergePendingCommit(#[from] openmls::group::MergePendingCommitError<StorageError>),
    #[error("merge staged commit: {0}")]
    MergeStagedCommit(#[from] openmls::group::MergeCommitError<sql_key_store::MemoryStorageError>),
    #[error(
        "no pending commit to merge. group epoch is {group_epoch:?} and got {message_epoch:?}"
    )]
    NoPendingCommit {
        message_epoch: GroupEpoch,
        group_epoch: GroupEpoch,
    },
    #[error("intent error: {0}")]
    Intent(#[from] IntentError),
    #[error("storage error: {0}")]
    Storage(#[from] crate::storage::StorageError),
    #[error("TLS Codec error: {0}")]
    TlsError(#[from] TlsCodecError),
    #[error("unsupported message type: {0:?}")]
    UnsupportedMessageType(Discriminant<MlsMessageBodyIn>),
    #[error("commit validation")]
    CommitValidation(#[from] CommitValidationError),
    #[error("codec")]
    Codec(#[from] crate::codecs::CodecError),
    #[error("encode proto: {0}")]
    EncodeProto(#[from] EncodeError),
    #[error("epoch increment not allowed")]
    EpochIncrementNotAllowed,
    #[error("Welcome processing error: {0}")]
    WelcomeProcessing(String),
    #[error("wrong credential type")]
    WrongCredentialType(#[from] BasicCredentialError),
    #[error("proto decode error: {0}")]
    DecodeError(#[from] prost::DecodeError),
    #[error("generic:{0}")]
    Generic(String),
}

impl crate::retry::RetryableError for MessageProcessingError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::Storage(s) => s.is_retryable(),
            _ => false,
        }
    }
}

impl From<String> for ClientError {
    fn from(value: String) -> Self {
        Self::Generic(value)
    }
}

impl From<&str> for ClientError {
    fn from(value: &str) -> Self {
        Self::Generic(value.to_string())
    }
}

/// Clients manage access to the network, identity, and data store
#[derive(Debug)]
pub struct Client<ApiClient> {
    pub(crate) api_client: ApiClientWrapper<ApiClient>,
    pub(crate) context: Arc<XmtpMlsLocalContext>,
}

/// The local context a XMTP MLS needs to function:
/// - Sqlite Database
/// - Identity for the User
///
#[derive(Debug)]
pub struct XmtpMlsLocalContext {
    /// XMTP Identity
    pub(crate) identity: Identity,
    /// XMTP Local Storage
    pub(crate) store: EncryptedMessageStore,
}

impl XmtpMlsLocalContext {
    /// The installation public key is the primary identifier for an installation
    pub fn installation_public_key(&self) -> Vec<u8> {
        self.identity.installation_keys.to_public_vec()
    }

    /// Get the account address of the blockchain account associated with this client
    pub fn inbox_id(&self) -> InboxId {
        self.identity.inbox_id().clone()
    }

    /// Get sequence id, may not be consistent with the backend
    pub fn inbox_sequence_id(&self, conn: &DbConnection) -> Result<i64, StorageError> {
        self.identity.sequence_id(conn)
    }

    /// Integrators should always check the `signature_request` return value of this function before calling [`register_identity`](Self::register_identity).
    /// If `signature_request` returns `None`, then the wallet signature is not required and [`register_identity`](Self::register_identity) can be called with None as an argument.
    pub fn signature_request(&self) -> Option<SignatureRequest> {
        self.identity.signature_request()
    }

    pub(crate) fn mls_provider(&self, conn: DbConnection) -> XmtpOpenMlsProvider {
        XmtpOpenMlsProvider::new(conn)
    }
}

impl<ApiClient> Client<ApiClient>
where
    ApiClient: XmtpApi,
{
    /// Create a new client with the given network, identity, and store.
    /// It is expected that most users will use the [`ClientBuilder`](crate::builder::ClientBuilder) instead of instantiating
    /// a client directly.
    pub fn new(
        api_client: ApiClientWrapper<ApiClient>,
        identity: Identity,
        store: EncryptedMessageStore,
    ) -> Self {
        let context = XmtpMlsLocalContext { identity, store };
        Self {
            api_client,
            context: Arc::new(context),
        }
    }

    pub fn installation_public_key(&self) -> Vec<u8> {
        self.context.installation_public_key()
    }

    pub fn inbox_id(&self) -> String {
        self.context.inbox_id()
    }

    /// Get sequence id, may not be consistent with the backend
    pub fn inbox_sequence_id(&self, conn: &DbConnection) -> Result<i64, StorageError> {
        self.context.inbox_sequence_id(conn)
    }

    pub fn store(&self) -> &EncryptedMessageStore {
        &self.context.store
    }

    pub fn identity(&self) -> &Identity {
        &self.context.identity
    }

    pub(crate) fn mls_provider(&self, conn: DbConnection) -> XmtpOpenMlsProvider {
        XmtpOpenMlsProvider::new(conn)
    }

    pub fn context(&self) -> &Arc<XmtpMlsLocalContext> {
        &self.context
    }

    /// Create a new group with the default settings
    pub fn create_group(
        &self,
        permissions: Option<PreconfiguredPolicies>,
    ) -> Result<MlsGroup, ClientError> {
        log::info!("creating group");

        let group = MlsGroup::create_and_insert(
            self.context.clone(),
            GroupMembershipState::Allowed,
            permissions,
        )
        .map_err(|e| {
            ClientError::Storage(StorageError::Store(format!("group create error {}", e)))
        })?;

        Ok(group)
    }

    pub(crate) fn create_sync_group(&self) -> Result<MlsGroup, StorageError> {
        log::info!("creating sync group");
        let sync_group = MlsGroup::create_and_insert_sync_group(self.context.clone())
            .map_err(|e| StorageError::Store(format!("sync group create error {}", e)))?;

        Ok(sync_group)
    }

    /// Look up a group by its ID
    /// Returns a [`MlsGroup`] if the group exists, or an error if it does not
    pub fn group(&self, group_id: Vec<u8>) -> Result<MlsGroup, ClientError> {
        let conn = &mut self.store().conn()?;
        let stored_group: Option<StoredGroup> = conn.fetch(&group_id)?;
        match stored_group {
            Some(group) => Ok(MlsGroup::new(
                self.context.clone(),
                group.id,
                group.created_at_ns,
            )),
            None => Err(ClientError::Storage(StorageError::NotFound)),
        }
    }

    /// Query for groups with optional filters
    ///
    /// Filters:
    /// - allowed_states: only return groups with the given membership states
    /// - created_after_ns: only return groups created after the given timestamp (in nanoseconds)
    /// - created_before_ns: only return groups created before the given timestamp (in nanoseconds)
    /// - limit: only return the first `limit` groups
    pub fn find_groups(
        &self,
        allowed_states: Option<Vec<GroupMembershipState>>,
        created_after_ns: Option<i64>,
        created_before_ns: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<MlsGroup>, ClientError> {
        Ok(self
            .store()
            .conn()?
            .find_groups(allowed_states, created_after_ns, created_before_ns, limit)?
            .into_iter()
            .map(|stored_group| {
                MlsGroup::new(
                    self.context.clone(),
                    stored_group.id,
                    stored_group.created_at_ns,
                )
            })
            .collect())
    }

    /// Register the identity with the network
    /// Callers should always check the result of [`text_to_sign`](Self::text_to_sign) before invoking this function.
    ///
    /// If `text_to_sign` returns `None`, then the wallet signature is not required and this function can be called with `None`.
    ///
    /// If `text_to_sign` returns `Some`, then the caller should sign the text with their wallet and pass the signature to this function.
    pub async fn register_identity(
        &self,
        signature_request: SignatureRequest,
    ) -> Result<(), ClientError> {
        log::info!("registering identity");
        self.apply_signature_request(signature_request).await?;
        let connection = self.store().conn()?;
        let provider = self.mls_provider(connection);
        self.identity()
            .register(&provider, &self.api_client)
            .await?;

        Ok(())
    }

    /// Upload a new key package to the network replacing an existing key package
    /// This is expected to be run any time the client receives new Welcome messages
    pub async fn rotate_key_package(&self) -> Result<(), ClientError> {
        let connection = self.store().conn()?;
        let kp = self
            .identity()
            .new_key_package(&self.mls_provider(connection))?;
        let kp_bytes = kp.tls_serialize_detached()?;
        self.api_client.upload_key_package(kp_bytes, false).await?;

        Ok(())
    }

    pub(crate) async fn query_group_messages(
        &self,
        group_id: &Vec<u8>,
        conn: &DbConnection,
    ) -> Result<Vec<GroupMessage>, ClientError> {
        let id_cursor = conn.get_last_cursor_for_id(group_id, EntityKind::Group)?;

        let welcomes = self
            .api_client
            .query_group_messages(group_id.clone(), Some(id_cursor as u64))
            .await?;

        Ok(welcomes)
    }

    pub(crate) async fn query_welcome_messages(
        &self,
        conn: &DbConnection,
    ) -> Result<Vec<WelcomeMessage>, ClientError> {
        let installation_id = self.installation_public_key();
        let id_cursor = conn.get_last_cursor_for_id(&installation_id, EntityKind::Welcome)?;

        let welcomes = self
            .api_client
            .query_welcome_messages(installation_id, Some(id_cursor as u64))
            .await?;

        Ok(welcomes)
    }

    pub(crate) fn process_for_id<ProcessingFn, ReturnValue>(
        &self,
        entity_id: &Vec<u8>,
        entity_kind: EntityKind,
        cursor: u64,
        process_envelope: ProcessingFn,
    ) -> Result<ReturnValue, MessageProcessingError>
    where
        ProcessingFn: FnOnce(&XmtpOpenMlsProvider) -> Result<ReturnValue, MessageProcessingError>,
    {
        self.store().transaction(|provider| {
            let is_updated =
                provider
                    .conn()
                    .update_cursor(entity_id, entity_kind, cursor as i64)?;
            if !is_updated {
                return Err(MessageProcessingError::AlreadyProcessed(cursor));
            }
            process_envelope(provider)
        })
    }

    pub(crate) async fn get_key_packages_for_installation_ids(
        &self,
        installation_ids: Vec<Vec<u8>>,
    ) -> Result<Vec<VerifiedKeyPackageV2>, ClientError> {
        let key_package_results = self.api_client.fetch_key_packages(installation_ids).await?;

        let conn = self.store().conn()?;
        let mls_provider = self.mls_provider(conn);
        Ok(key_package_results
            .values()
            .map(|bytes| VerifiedKeyPackageV2::from_bytes(mls_provider.crypto(), bytes.as_slice()))
            .collect::<Result<_, _>>()?)
    }

    /// Download all unread welcome messages and convert to groups.
    /// Returns any new groups created in the operation
    pub async fn sync_welcomes(&self) -> Result<Vec<MlsGroup>, ClientError> {
        let envelopes = self.query_welcome_messages(&self.store().conn()?).await?;
        let id = self.installation_public_key();
        let groups: Vec<MlsGroup> = envelopes
            .into_iter()
            .filter_map(|envelope: WelcomeMessage| {
                let welcome_v1 = match extract_welcome_message(envelope) {
                    Ok(inner) => inner,
                    Err(err) => {
                        log::error!("failed to extract welcome message: {}", err);
                        return None;
                    }
                };

                self.process_for_id(&id, EntityKind::Welcome, welcome_v1.id, |provider| {
                    // TODO: Abort if error is retryable
                    match MlsGroup::create_from_encrypted_welcome(
                        self.context.clone(),
                        provider,
                        welcome_v1.hpke_public_key.as_slice(),
                        welcome_v1.data,
                    ) {
                        Ok(mls_group) => Ok(Some(mls_group)),
                        Err(err) => {
                            log::error!("failed to create group from welcome: {}", err);
                            Err(MessageProcessingError::WelcomeProcessing(err.to_string()))
                        }
                    }
                })
                .ok()
                .flatten()
            })
            .collect();

        Ok(groups)
    }

    /**
     * Validates a credential against the given installation public key
     *
     * This will go to the network and get the latest association state for the inbox.
     * It ensures that the installation_pub_key is in that association state
     */
    pub async fn validate_credential_against_network(
        &self,
        conn: &DbConnection,
        credential: &[u8],
        installation_pub_key: Vec<u8>,
    ) -> Result<InboxId, ClientError> {
        let inbox_id = parse_credential(credential)?;
        let association_state = self.get_latest_association_state(conn, &inbox_id).await?;

        match association_state.get(&installation_pub_key.clone().into()) {
            Some(_) => Ok(inbox_id),
            None => Err(IdentityError::InstallationIdNotFound(inbox_id).into()),
        }
    }

    /// Check whether an account_address has a key package registered on the network
    ///
    /// Arguments:
    /// - account_addresses: a list of account addresses to check
    ///
    /// Returns:
    /// A Vec of booleans indicating whether each account address has a key package registered on the network
    pub async fn can_message(
        &self,
        account_addresses: Vec<String>,
    ) -> Result<HashMap<String, bool>, ClientError> {
        let account_addresses = sanitize_evm_addresses(account_addresses)?;
        let inbox_id_map = self
            .api_client
            .get_inbox_ids(account_addresses.clone())
            .await?;

        let results = account_addresses
            .iter()
            .map(|address| {
                let result = inbox_id_map.get(address).map(|_| true).unwrap_or(false);
                (address.clone(), result)
            })
            .collect::<HashMap<String, bool>>();

        Ok(results)
    }
}

pub(crate) fn extract_welcome_message(
    welcome: WelcomeMessage,
) -> Result<WelcomeMessageV1, ClientError> {
    match welcome.version {
        Some(WelcomeMessageVersion::V1(welcome)) => Ok(welcome),
        _ => Err(ClientError::Generic(
            "unexpected message type in welcome".to_string(),
        )),
    }
}

pub fn deserialize_welcome(welcome_bytes: &Vec<u8>) -> Result<Welcome, ClientError> {
    // let welcome_proto = WelcomeMessageProto::decode(&mut welcome_bytes.as_slice())?;
    let welcome = MlsMessageIn::tls_deserialize(&mut welcome_bytes.as_slice())?;
    match welcome.extract() {
        MlsMessageBodyIn::Welcome(welcome) => Ok(welcome),
        _ => Err(ClientError::Generic(
            "unexpected message type in welcome".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use xmtp_cryptography::utils::generate_local_wallet;

    use crate::{
        builder::ClientBuilder,
        hpke::{decrypt_welcome, encrypt_welcome},
    };

    #[tokio::test]
    async fn test_mls_error() {
        let client = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let result = client
            .api_client
            .register_installation(vec![1, 2, 3], false)
            .await;

        assert!(result.is_err());
        let error_string = result.err().unwrap().to_string();
        assert!(error_string.contains("invalid identity"));
    }

    #[tokio::test]
    async fn test_register_installation() {
        let wallet = generate_local_wallet();
        let client = ClientBuilder::new_test_client(&wallet).await;
        let client_2 = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        // Make sure the installation is actually on the network
        let association_state = client_2
            .get_latest_association_state(&client_2.store().conn().unwrap(), client.inbox_id())
            .await
            .unwrap();

        assert_eq!(association_state.installation_ids().len(), 1);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_rotate_key_package() {
        let wallet = generate_local_wallet();
        let client = ClientBuilder::new_test_client(&wallet).await;

        // Get original KeyPackage.
        let kp1 = client
            .get_key_packages_for_installation_ids(vec![client.installation_public_key()])
            .await
            .unwrap();
        assert_eq!(kp1.len(), 1);
        let init1 = kp1[0].inner.hpke_init_key();

        // Rotate and fetch again.
        client.rotate_key_package().await.unwrap();

        let kp2 = client
            .get_key_packages_for_installation_ids(vec![client.installation_public_key()])
            .await
            .unwrap();
        assert_eq!(kp2.len(), 1);
        let init2 = kp2[0].inner.hpke_init_key();

        assert_ne!(init1, init2);
    }

    #[tokio::test]
    async fn test_find_groups() {
        let client = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let group_1 = client.create_group(None).unwrap();
        let group_2 = client.create_group(None).unwrap();

        let groups = client.find_groups(None, None, None, None).unwrap();
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].group_id, group_1.group_id);
        assert_eq!(groups[1].group_id, group_2.group_id);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_sync_welcomes() {
        let alice = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let bob = ClientBuilder::new_test_client(&generate_local_wallet()).await;

        let alice_bob_group = alice.create_group(None).unwrap();
        alice_bob_group
            .add_members_by_inbox_id(&alice, vec![bob.inbox_id()])
            .await
            .unwrap();

        let bob_received_groups = bob.sync_welcomes().await.unwrap();
        assert_eq!(bob_received_groups.len(), 1);
        assert_eq!(
            bob_received_groups.first().unwrap().group_id,
            alice_bob_group.group_id
        );

        let duplicate_received_groups = bob.sync_welcomes().await.unwrap();
        assert_eq!(duplicate_received_groups.len(), 0);
    }

    #[tokio::test]
    async fn test_can_message() {
        // let amal = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        // let bola = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        // let charlie_address = generate_local_wallet().get_address();

        // let can_message_result = amal
        //     .can_message(vec![
        //         amal.account_address(),
        //         bola.account_address(),
        //         charlie_address.clone(),
        //     ])
        //     .await
        //     .unwrap();
        // assert_eq!(
        //     can_message_result.get(&amal.account_address().to_string()),
        //     Some(&true),
        //     "Amal's messaging capability should be true"
        // );
        // assert_eq!(
        //     can_message_result.get(&bola.account_address().to_string()),
        //     Some(&true),
        //     "Bola's messaging capability should be true"
        // );
        // assert_eq!(
        //     can_message_result.get(&charlie_address),
        //     Some(&false),
        //     "Charlie's messaging capability should be false"
        // );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_welcome_encryption() {
        let client = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let conn = client.store().conn().unwrap();
        let provider = client.mls_provider(conn);

        let kp = client.identity().new_key_package(&provider).unwrap();
        let hpke_public_key = kp.hpke_init_key().as_slice();
        let to_encrypt = vec![1, 2, 3];

        // Encryption doesn't require any details about the sender, so we can test using one client
        let encrypted = encrypt_welcome(to_encrypt.as_slice(), hpke_public_key).unwrap();

        let decrypted = decrypt_welcome(&provider, hpke_public_key, encrypted.as_slice()).unwrap();

        assert_eq!(decrypted, to_encrypt);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_add_remove_then_add_again() {
        let amal = ClientBuilder::new_test_client(&generate_local_wallet()).await;
        let bola = ClientBuilder::new_test_client(&generate_local_wallet()).await;

        // Create a group and invite bola
        let amal_group = amal.create_group(None).unwrap();
        amal_group
            .add_members_by_inbox_id(&amal, vec![bola.inbox_id()])
            .await
            .unwrap();
        assert_eq!(amal_group.members().unwrap().len(), 2);

        // Now remove bola
        amal_group
            .remove_members_by_inbox_id(&amal, vec![bola.inbox_id()])
            .await
            .unwrap();
        assert_eq!(amal_group.members().unwrap().len(), 1);
        log::info!("Syncing bolas welcomes");
        // See if Bola can see that they were added to the group
        bola.sync_welcomes().await.unwrap();
        let bola_groups = bola.find_groups(None, None, None, None).unwrap();
        assert_eq!(bola_groups.len(), 1);
        let bola_group = bola_groups.first().unwrap();
        log::info!("Syncing bolas messages");
        bola_group.sync(&bola).await.unwrap();
        // TODO: figure out why Bola's status is not updating to be inactive
        // assert!(!bola_group.is_active().unwrap());

        // Bola should have one readable message (them being added to the group)
        let mut bola_messages = bola_group
            .find_messages(None, None, None, None, None)
            .unwrap();
        // TODO:nm figure out why the transcript message is no longer decryptable
        assert_eq!(bola_messages.len(), 1);

        // Add Bola back to the group
        amal_group
            .add_members_by_inbox_id(&amal, vec![bola.inbox_id()])
            .await
            .unwrap();
        bola.sync_welcomes().await.unwrap();

        // Send a message from Amal, now that Bola is back in the group
        amal_group
            .send_message(vec![1, 2, 3].as_slice(), &amal)
            .await
            .unwrap();

        // Sync Bola's state to get the latest
        bola_group.sync(&bola).await.unwrap();
        // Find Bola's updated list of messages
        bola_messages = bola_group
            .find_messages(None, None, None, None, None)
            .unwrap();
        // Bola should have been able to decrypt the last message
        assert_eq!(bola_messages.len(), 2);
        assert_eq!(
            bola_messages.get(1).unwrap().decrypted_message_bytes,
            vec![1, 2, 3]
        )
    }
}
