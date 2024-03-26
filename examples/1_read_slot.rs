use solana_client::rpc_client::RpcClient;
use std::{error::Error, fs::File};
use tokio;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the RPC client
    let rpc_url = "https://api.devnet.solana.com".to_string(); // Use the appropriate network URL
    let rpc_client = RpcClient::new(rpc_url);

    // Specify the block number you want to read logs for
    let slot = 268573327;

    let slot_data = rpc_client.get_block(slot)?;
    // print!("{:?}", slot_data);

    let mut file = File::create("logs.txt")?;
    writeln!(file, "{:?}", slot_data)?;

    Ok(())
}
