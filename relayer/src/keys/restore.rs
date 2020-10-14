use crate::chain::CosmosSDKChain;
use crate::config::ChainConfig;
use crate::error;
use crate::error::Error;

#[derive(Clone, Debug)]
pub struct KeysRestoreOptions {
    pub name: String,
    pub mnemonic: String,
    pub chain_config: ChainConfig,
}

pub fn restore_key(opts: KeysRestoreOptions) -> Result<Vec<u8>, Error> {
    // Get the destination chain
    let mut chain = CosmosSDKChain::from_config(opts.clone().chain_config)?;

    chain.keybase.add_from_mnemonic(opts.clone().name, &opts.mnemonic).map_err(|e| error::Kind::KeyBase.context(e))?;

    let key = chain.keybase.get(opts.name).unwrap();
    Ok(key.clone().address)
}
