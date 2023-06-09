// use std::fs;
#[allow(dead_code)]
use crate::client::RpcClient;
// use pkg::{generate_key}; 
use ChainSauce_task1::init;

pub mod client;
pub mod pkg;

#[allow(dead_code)]
#[derive(Debug)]
struct BucketResponse {
    id: String,
    bucket_name: String,
}

#[tokio::main]
async fn main() {
    let cli = crate::init();

    let matches = cli.get_matches();
    let rpc_client =
        crate::RpcClient::new("https://gnfd-testnet-fullnode-tendermint-us.bnbchain.org:443")
            .expect("init client");

    match matches.subcommand() {
        Some(("bucket", sub_matches)) => {
            let bucket_command = sub_matches.subcommand().unwrap_or(("help", sub_matches));

            match bucket_command {
                ("ls", _sub_matches) => {
                    let response = rpc_client.bucket.list_bucket().await.unwrap();
                    
                    let address = "0x1060D988E6b1235d1Bd0A01E6378A934b6aC763e".to_string();
                    let res = response
                        .bucket_infos
                        .into_iter()
                        .filter(|x| x.owner == address)
                        .collect::<Vec<_>>();

                    println!("bucket list:");
                    res.iter().for_each(|x| println!("bucket name: {}, bucket id: {}", x.bucket_name, x.id));

                }

                ("create", sub_matches) => {
                    let bucket_url = sub_matches.get_one::<String>("bucket_url");
                    if bucket_url.is_none() {
                        println!("error!");
                    }
            

                    let _primary_sp_addr:String = match sub_matches.get_one::<String>("primary_sp") {
                        Some(addr) => addr.clone(),
                        None => "some_address_from_client".to_string(),
                    };

                    let _payment_addr: String =
                        match sub_matches.get_one::<String>("paymentAddress") {
                            Some(addr) => addr.clone(),
                            None => "".to_string(),
                        };

                    let _visibility: String = match sub_matches.get_one::<String>("visibility") {
                        Some(x) => x.clone(),
                        None => "private".to_string(),
                    };

                    let _charge_quota: u64 = match sub_matches.get_one::<u64>("chargedQuota") {
                        Some(&quota) if quota > 0 => quota.clone(),
                        _ => 0,
                    };

                    // sdk get bucket options
                    // set bucket options from flags
                    // send request

                    println!("Bucket was created... kidding :) work in progress.")
                },
                (&_, _) => {
                    println!("Invalid command")
                },
            }
            
        }

        Some(("bank", sub_matches)) => {
            let bank_command = sub_matches.subcommand().unwrap_or(("help", sub_matches));

            match bank_command {
                ("transfer", sub_matches) => {
                    let destination_addr = sub_matches.get_one::<String>("toAddressFlag");
                    if destination_addr.is_none() {
                        println!("flag toAddress is none.");
                    } else {
                        let def_dest_addr = "0x0000".to_string();
                        let destination_addr = destination_addr.unwrap_or(&def_dest_addr);
                        println!("{destination_addr}")
                    }

                    let amount = sub_matches.get_one::<String>("amountFlag");
                    if amount.is_none() {
                        println!("flag amount is none.");
                    } else {
                        let def_amount = "10".to_string();
                        let amount = amount.unwrap_or(&def_amount);
                        println!("{amount}")
                    }

                    // do transfer
                }

                ("balance", sub_matches) => {
                    let address = sub_matches.get_one::<String>("addressFlag");
                    if address.is_none() {
                        println!("flag address is none.");
                    } else {
                        let def_addr = "0x0000".to_string();
                        let address = address.unwrap_or(&def_addr);
                        
                        let balances = rpc_client
                            .bank
                            .all_balances(address, None)
                            .await
                            .unwrap();

                        balances.balances.iter().for_each(|x| println!("balance: {}{}\n", x.amount, x.denom));
                    }
                }

                (&_, _) => {
                    println!("something")
                }
            }
        }

        Some(("create-keystore", sub_matches)) => {
            let files = sub_matches
                .get_many::<String>("privKeyFileFlag")
                .map(|vals| vals.collect::<Vec<_>>())
                .unwrap_or_default();

            let key_path = "src/".to_owned() + files[0];
            // println!("{password_path}");

            let config_path = "src/config.toml".to_string();
            // let content = fs::read_to_string(password_path)
            //     .expect("Should have been able to read the file");
            // println!("{content}");

//             generate_key(key_path, config_path);
        }

        Some((&_, _)) => {
            println!("something")
        }
        None => {
            println!("nothing")
        }
    }
}
