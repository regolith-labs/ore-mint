use ore_mint_api::state::Authority;
use solana_client::nonblocking::rpc_client::RpcClient;
use steel::{AccountDeserialize, Clock};

#[tokio::main]
async fn main() {
    // Build transaction
    let rpc = RpcClient::new(std::env::var("RPC").expect("Missing RPC env var"));
    match std::env::var("COMMAND")
        .expect("Missing COMMAND env var")
        .as_str()
    {
        "authority" => {
            log_authority(&rpc).await.unwrap();
        }
        "clock" => {
            log_clock(&rpc).await.unwrap();
        }
        _ => panic!("Invalid command"),
    };
}

async fn log_authority(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let authority_address = ore_mint_api::state::authority_pda().0;
    let authority = get_authority(&rpc).await?;
    println!("Authority");
    println!("  address: {}", authority_address);
    println!("  last_mint_at: {}", authority.last_mint_at);
    Ok(())
}

async fn log_clock(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let clock = get_clock(&rpc).await?;
    println!("Clock");
    println!("  slot: {}", clock.slot);
    println!("  epoch_start_timestamp: {}", clock.epoch_start_timestamp);
    println!("  epoch: {}", clock.epoch);
    println!("  leader_schedule_epoch: {}", clock.leader_schedule_epoch);
    println!("  unix_timestamp: {}", clock.unix_timestamp);
    Ok(())
}

async fn get_authority(rpc: &RpcClient) -> Result<Authority, anyhow::Error> {
    let authority_address = ore_mint_api::state::authority_pda().0;
    let account = rpc.get_account(&authority_address).await?;
    let authority = Authority::try_from_bytes(&account.data)?;
    Ok(*authority)
}

async fn get_clock(rpc: &RpcClient) -> Result<Clock, anyhow::Error> {
    let data = rpc.get_account_data(&solana_sdk::sysvar::clock::ID).await?;
    let clock = bincode::deserialize::<Clock>(&data)?;
    Ok(clock)
}
