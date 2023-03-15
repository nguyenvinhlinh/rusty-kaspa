use addresses::Address;
use consensus_core::tx::ScriptPublicKey;

#[allow(dead_code)]
/// Represents an [`Address`] and its matching [`ScriptPublicKey`] representation
pub struct UtxoAddress {
    address: Address,
    script_public_key: ScriptPublicKey,
}

impl UtxoAddress {
    pub fn new(address: Address, script_public_key: ScriptPublicKey) -> Self {
        Self { address, script_public_key }
    }

    #[inline(always)]
    pub fn address(&self) -> &Address {
        &self.address
    }

    #[inline(always)]
    pub fn script_public_key(&self) -> &ScriptPublicKey {
        &self.script_public_key
    }
}

impl From<Address> for UtxoAddress {
    fn from(_item: Address) -> Self {
        // TODO: call txscript golang PayToAddrScript equivalent when available
        todo!()
    }
}

impl From<ScriptPublicKey> for UtxoAddress {
    fn from(_item: ScriptPublicKey) -> Self {
        // TODO: call txscript golang ExtractScriptPubKeyAddress equivalent when available
        todo!()
    }
}

pub mod test_helpers {
    use super::*;
    use addresses::Prefix;

    pub fn get_3_addresses(sorted: bool) -> Vec<Address> {
        let mut addresses = vec![
            Address { prefix: Prefix::Mainnet, payload: vec![1u8; 32], version: 0 },
            Address { prefix: Prefix::Mainnet, payload: vec![2u8; 32], version: 0 },
            Address { prefix: Prefix::Mainnet, payload: vec![0u8; 32], version: 0 },
        ];
        if sorted {
            addresses.sort()
        }
        addresses
    }
}