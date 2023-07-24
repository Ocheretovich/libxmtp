use core::fmt;
use std::fmt::Formatter;

use thiserror::Error;
use vodozemac::olm::OlmMessage;

use crate::{
    api_utils::get_contacts,
    app_context::AppContext,
    contact::{Contact, ContactError},
    conversations::Conversations,
    session::SessionManager,
    storage::{StorageError, StoredInstallation},
    types::networking::{PublishRequest, XmtpApiClient},
    types::Address,
    utils::{build_envelope, build_user_contact_topic},
    Store,
};
use xmtp_proto::xmtp::message_api::v1::Envelope;

#[derive(Clone, Copy, Default, Debug)]
pub enum Network {
    Local(&'static str),
    #[default]
    Dev,
    Prod,
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("contact error {0}")]
    Contact(#[from] ContactError),
    #[error("could not publish: {0}")]
    PublishError(String),
    #[error("storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("Query failed: {0}")]
    QueryError(String),
    #[error("unknown client error")]
    Unknown,
}

pub struct Client<'c, A>
where
    A: XmtpApiClient,
{
    pub app_context: AppContext<A>, // Temporarily exposed outside crate for CLI client
    conversations: Option<Conversations<'c, A>>,
    // conversations: Conversations<'c, A>,
    is_initialized: bool,
}

impl<'c, A> core::fmt::Debug for Client<'c, A>
where
    A: XmtpApiClient,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Client({:?})::{}",
            self.app_context.network,
            self.app_context.account.addr()
        )
    }
}

impl<'c, A> Client<'c, A>
where
    A: XmtpApiClient,
{
    pub fn new(app_context: AppContext<A>) -> Self {
        // let conversations = Conversations::new(&app_context);

        // Self {
        //     app_context,
        //     conversations,
        //     is_initialized: false,
        // }
        let mut client = Self {
            app_context,
            conversations: None,
            is_initialized: false,
        };
        client
    }

    pub(crate) fn init_conversations(&mut self) {
        self.conversations = Some(Conversations::new(&self.app_context));
    }

    pub fn wallet_address(&self) -> Address {
        self.app_context.account.addr()
    }

    pub fn conversations(&self) -> &Conversations<A> {
        self.conversations.as_ref().unwrap()
        // &self.conversations
        // if self.conversations.is_none() {
        //     self.conversations = Some(Conversations::new(&self.app_context));
        // }
        // match &self.conversations {
        //     Some(conversations) => &self.conversations.as_ref().unwrap(),
        //     None => {
        //         self.conversations = Some(Conversations::new(&self.app_context));
        //         &self.conversations.unwrap()
        //     }
        // }
    }

    pub async fn init(&mut self) -> Result<(), ClientError> {
        let app_contact_bundle = self.app_context.account.contact();
        let registered_bundles = get_contacts(&self.app_context, &self.wallet_address()).await?;

        if !registered_bundles
            .iter()
            .any(|contact| contact.installation_id() == app_contact_bundle.installation_id())
        {
            self.publish_user_contact().await?;
        }

        self.is_initialized = true;
        Ok(())
    }

    pub async fn my_other_devices(&self) -> Result<Vec<Contact>, ClientError> {
        let contacts =
            get_contacts(&self.app_context, self.app_context.account.addr().as_str()).await?;
        let my_contact_id = self.app_context.account.contact().installation_id();
        Ok(contacts
            .into_iter()
            .filter(|c| c.installation_id() != my_contact_id)
            .collect())
    }

    pub async fn refresh_user_installations(&self, user_address: &str) -> Result<(), ClientError> {
        let contacts = get_contacts(&self.app_context, user_address).await?;

        let stored_contacts: Vec<StoredInstallation> =
            self.app_context.store.get_contacts(user_address)?.into();
        println!("{:?}", contacts);
        for contact in contacts {
            println!(" {:?} ", contact)
        }

        Ok(())
    }

    pub fn create_inbound_session(
        &self,
        contact: Contact,
        // Message MUST be a pre-key message
        message: Vec<u8>,
    ) -> Result<(SessionManager, Vec<u8>), ClientError> {
        let olm_message: OlmMessage =
            serde_json::from_slice(message.as_slice()).map_err(|_| ClientError::Unknown)?;
        let msg = match olm_message {
            OlmMessage::PreKey(msg) => msg,
            _ => return Err(ClientError::Unknown),
        };

        let create_result = self
            .app_context
            .account
            .create_inbound_session(&contact, msg)
            .map_err(|_| ClientError::Unknown)?;

        let session = SessionManager::from_olm_session(create_result.session, &contact)
            .map_err(|_| ClientError::Unknown)?;

        session.store(&self.app_context.store)?;

        Ok((session, create_result.plaintext))
    }

    async fn publish_user_contact(&self) -> Result<(), ClientError> {
        let envelope = self.build_contact_envelope()?;
        self.app_context
            .api_client
            .publish(
                "".to_string(),
                PublishRequest {
                    envelopes: vec![envelope],
                },
            )
            .await
            .map_err(|e| ClientError::PublishError(format!("Could not publish contact: {}", e)))?;

        Ok(())
    }

    fn build_contact_envelope(&self) -> Result<Envelope, ClientError> {
        let contact = self.app_context.account.contact();

        let envelope = build_envelope(
            build_user_contact_topic(self.wallet_address()),
            contact.try_into()?,
        );

        Ok(envelope)
    }
}

#[cfg(test)]
mod tests {
    use xmtp_proto::xmtp::v3::message_contents::installation_contact_bundle::Version;
    use xmtp_proto::xmtp::v3::message_contents::vmac_unsigned_public_key::Union::Curve25519;
    use xmtp_proto::xmtp::v3::message_contents::vmac_unsigned_public_key::VodozemacCurve25519;

    use crate::api_utils::get_contacts;
    use crate::test_utils::test_utils::gen_test_client;
    use crate::ClientBuilder;

    #[tokio::test]
    async fn registration() {
        gen_test_client().await;
    }

    #[tokio::test]
    async fn refresh() {
        let client = ClientBuilder::new_test().build().unwrap();
        client
            .refresh_user_installations(&client.wallet_address())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_publish_user_contact() {
        let client = ClientBuilder::new_test().build().unwrap();
        client
            .publish_user_contact()
            .await
            .expect("Failed to publish user contact");

        let contacts = get_contacts(&client.app_context, client.wallet_address().as_str())
            .await
            .unwrap();

        assert_eq!(contacts.len(), 1);
        let installation_bundle = match contacts[0].clone().bundle.version.unwrap() {
            Version::V1(bundle) => bundle,
        };
        assert!(installation_bundle.fallback_key.is_some());
        assert!(installation_bundle.identity_key.is_some());
        contacts[0].vmac_identity_key();
        contacts[0].vmac_fallback_key();

        let key_bytes = installation_bundle
            .clone()
            .identity_key
            .unwrap()
            .key
            .unwrap()
            .union
            .unwrap();

        match key_bytes {
            Curve25519(VodozemacCurve25519 { bytes }) => {
                assert_eq!(bytes.len(), 32);
                assert_eq!(
                    client
                        .app_context
                        .account
                        .olm_account()
                        .unwrap()
                        .get()
                        .curve25519_key()
                        .to_bytes()
                        .to_vec(),
                    bytes
                )
            }
        }
    }
}
