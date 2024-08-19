#![recursion_limit = "256"]
pub mod inbox_owner;
pub mod logger;
pub mod mls;
pub mod v2;

pub use crate::inbox_owner::SigningError;
use inbox_owner::FfiInboxOwner;
use logger::FfiLogger;
pub use mls::*;
use std::error::Error;

uniffi::include_scaffolding!("xmtpv3");

#[derive(uniffi::Error, thiserror::Error, Debug)]
#[uniffi(flat_error)]
pub enum GenericError {
    #[error("Client error: {0}")]
    Client(#[from] xmtp_mls::client::ClientError),
    #[error("Client builder error: {0}")]
    ClientBuilder(#[from] xmtp_mls::builder::ClientBuilderError),
    #[error("Storage error: {0}")]
    Storage(#[from] xmtp_mls::storage::StorageError),
    #[error("API error: {0}")]
    ApiError(#[from] xmtp_proto::api_client::Error),
    #[error("Group error: {0}")]
    GroupError(#[from] xmtp_mls::groups::GroupError),
    #[error("Signature: {0}")]
    Signature(#[from] xmtp_cryptography::signature::SignatureError),
    #[error("Group metadata: {0}")]
    GroupMetadata(#[from] xmtp_mls::groups::group_metadata::GroupMetadataError),
    #[error("Group permissions: {0}")]
    GroupMutablePermissions(
        #[from] xmtp_mls::groups::group_permissions::GroupMutablePermissionsError,
    ),
    #[error("Generic {err}")]
    Generic { err: String },
    #[error(transparent)]
    SignatureRequestError(#[from] xmtp_id::associations::builder::SignatureRequestError),
    #[error(transparent)]
    Erc1271SignatureError(#[from] xmtp_id::associations::signature::SignatureError),
}

impl From<String> for GenericError {
    fn from(err: String) -> Self {
        Self::Generic { err }
    }
}

impl GenericError {
    pub fn from_error<T: Error>(err: T) -> Self {
        Self::Generic {
            err: stringify_error_chain(&err),
        }
    }
}

// TODO Use non-string errors across Uniffi interface
fn stringify_error_chain<T: Error>(error: &T) -> String {
    let mut result = format!("Error: {}\n", error);

    let mut source = error.source();
    while let Some(src) = source {
        result += &format!("Caused by: {}\n", src);
        source = src.source();
    }

    result
}

#[uniffi::export]
pub fn get_version_info() -> String {
    include_str!("../libxmtp-version.txt").to_string()
}

#[cfg(test)]
mod tests {
    use crate::get_version_info;

    #[test]
    pub fn test_get_version_info() {
        print!("{}", get_version_info());
    }
}
