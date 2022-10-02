use std::fs;
use std::str::FromStr;
use web3::futures::TryStreamExt;
use web3::types::{BlockNumber, U64};
use web3::{
    types::{Address, FilterBuilder, TransactionId},
};
use anyhow::Result;
use serde_derive::Deserialize;
use ethereum_abi::Abi;

#[derive(Deserialize)]
struct Config{
    from_block: String,
    //to_block
    address: String,
    //infura_key
}

#[tokio::main]
async fn main() -> Result<()> {

    //let content = std::fs::read_to_string("somewhere/file.toml")?;
    //let config: Config = toml::from_str(&content)?;

    let web3 = web3::Web3::new(
        web3::transports::WebSocket::new(
            "wss://eth-mainnet.g.alchemy.com/v2/YZMpOWZXQ1y1bmfXh8OlnPJ1Ju-who3W",
        )
        .await?,
    );

    let filter = FilterBuilder::default()
        .from_block(BlockNumber::Number(U64::from(15655161)))
        .address(vec![Address::from_str("0xdAC17F958D2ee523a2206206994597C13D831ec7",).unwrap()])
        .build();

    let mut sub = web3.eth_subscribe().subscribe_logs(filter).await?;

    let text = fs::read_to_string("/Users/jfinet/Documents/hackathon/abi.json").unwrap();
    let abi: Abi = serde_json::from_str(&text).unwrap();

     

    while let Some(item) = sub.try_next().await? {
        let tx_hash = item.transaction_hash.unwrap();
        let tx = web3.eth().transaction(TransactionId::Hash(tx_hash)).await?.unwrap();
        let input = hex::encode(tx.input.0);
        let (function, decoded_params) = abi.decode_input_from_hex(&input).unwrap();
        let gas_price = tx.gas_price;
        let block_number = tx.block_number;
        let from = tx.from.unwrap();
        let to = tx.to.unwrap();

    }

    Ok(())
}
