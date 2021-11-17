use crate::SecureChannelError;
use ockam_core::compat::string::String;
use ockam_core::{Decodable, Encodable, LocalInfo, LocalMessage, Result};
use serde::{Deserialize, Serialize};

/// SecureChannel LocalInfo unique Identifier
pub const SECURE_CHANNEL_IDENTIFIER: &str = "SECURE_CHANNEL_IDENTIFIER";

/// Entity SecureChannel LocalInfo used for LocalMessage
#[derive(Serialize, Deserialize)]
pub struct SecureChannelLocalInfo {
    key_exchange: String,
    auth_hash: Vec<u8>,
}

impl SecureChannelLocalInfo {
    /// TODO
    pub fn from_local_info(value: &LocalInfo) -> Result<Self> {
        if value.type_identifier() != SECURE_CHANNEL_IDENTIFIER {
            return Err(SecureChannelError::InvalidLocalInfoType.into());
        }

        if let Ok(info) = SecureChannelLocalInfo::decode(value.data()) {
            return Ok(info);
        }

        Err(SecureChannelError::InvalidLocalInfoType.into())
    }

    /// TODO
    pub fn to_local_info(&self) -> Result<LocalInfo> {
        Ok(LocalInfo::new(
            SECURE_CHANNEL_IDENTIFIER.into(),
            self.encode()?,
        ))
    }

    /// TODO
    pub fn find_info(local_msg: &LocalMessage) -> Result<Self> {
        if let Some(local_info) = local_msg
            .local_info()
            .iter()
            .find(|x| x.type_identifier() == SECURE_CHANNEL_IDENTIFIER)
        {
            Self::from_local_info(local_info)
        } else {
            Err(SecureChannelError::InvalidLocalInfoType.into())
        }
    }
}

impl SecureChannelLocalInfo {
    /// Key exchange name
    pub fn key_exchange(&self) -> &str {
        &self.key_exchange
    }
    /// Auth hash
    pub fn auth_hash(&self) -> &[u8] {
        &self.auth_hash
    }
}

impl SecureChannelLocalInfo {
    /// Constructor
    pub fn new(key_exchange: String, auth_hash: Vec<u8>) -> Self {
        Self {
            key_exchange,
            auth_hash,
        }
    }
}
