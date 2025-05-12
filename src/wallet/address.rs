#[derive(Debug, Clone)]
pub struct Address;

impl Address {
    pub fn from_key_pair(_key: &crate::wallet::keys::KeyPair) -> Self { Address }
}
