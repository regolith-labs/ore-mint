use ore_mint_api::state::Authority;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
    transaction::Transaction,
};
use steel::{AccountDeserialize, Clock};

#[tokio::main]
async fn main() {
    // Read keypair from file
    let payer =
        read_keypair_file(&std::env::var("KEYPAIR").expect("Missing KEYPAIR env var")).unwrap();

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
        "init" => {
            init(&rpc, &payer).await.unwrap();
        }
        _ => panic!("Invalid command"),
    };
}

async fn init(rpc: &RpcClient, payer: &Keypair) -> Result<(), anyhow::Error> {
    let ix = ore_mint_api::sdk::init(payer.pubkey());
    submit_transaction(rpc, payer, &[ix]).await?;
    Ok(())
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

async fn submit_transaction(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
    instructions: &[solana_sdk::instruction::Instruction],
) -> Result<solana_sdk::signature::Signature, anyhow::Error> {
    let blockhash = rpc.get_latest_blockhash().await?;
    let mut all_instructions = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(1_400_000),
        ComputeBudgetInstruction::set_compute_unit_price(1_000_000),
    ];
    all_instructions.extend_from_slice(instructions);
    let transaction = Transaction::new_signed_with_payer(
        &all_instructions,
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    match rpc.send_and_confirm_transaction(&transaction).await {
        Ok(signature) => {
            println!("Transaction submitted: {:?}", signature);
            Ok(signature)
        }
        Err(e) => {
            println!("Error submitting transaction: {:?}", e);
            Err(e.into())
        }
    }
}
