use solana_client::pubsub_client::PubsubClient;
use solana_client::rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};
use solana_sdk::commitment_config::CommitmentConfig;

fn main() {
    let ws_url = String::from("wss://api.devnet.solana.com/");
    let filter = RpcTransactionLogsFilter::All;
    let config = RpcTransactionLogsConfig {
        commitment: Some(CommitmentConfig::finalized()),
    };

    if let Ok(subscription) = PubsubClient::logs_subscribe(&ws_url, filter, config) {
        let (mut ws_client, receiver) = subscription;

        std::thread::spawn(move || loop {
            match receiver.recv() {
                Ok(logs) => {
                    println!("Transaction executed in slot: {}", logs.context.slot);
                    println!("  Signature: {}", logs.value.signature);
                    println!(
                        "  Status: {}",
                        logs.value
                            .err
                            .map(|err| err.to_string())
                            .unwrap_or_else(|| "Ok".to_string())
                    );
                    println!("  Log Messages:");
                    for log in logs.value.logs {
                        println!("    {log}");
                    }
                }
                Err(err) => {
                    println!("disconnected {:}", err);
                    break;
                }
            }
        })
        .join()
        .unwrap();
        ws_client.shutdown().unwrap();
    } else {
        println!("could not connect to subscription service");
    }
}
