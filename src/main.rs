use std::fs;
use pkg::{init, generate_key};

pub mod pkg;

fn main() {
    let cli = init();

    let matches = cli.get_matches();

    match matches.subcommand() {
        Some(("bucket", sub_matches)) => {
            let bucket_command = sub_matches.subcommand().unwrap_or(("help", sub_matches));

            match bucket_command{
                ("create", sub_matches) => {
                    let primary_sp =  sub_matches.get_one::<String>("primarySPFlag");
                    if primary_sp.is_none(){
                        println!("flag primarySP is none.")
                    }
                    else{
                        let def_primary_sp = "default".to_string(); 
                        let primary_sp = primary_sp.unwrap_or(&def_primary_sp);
                        println!("{primary_sp}")
                    }
                }
                (&_, _) => {
                    println!("something")
                }
            }
            
        }

        Some(("bank", sub_matches)) => {
            let bank_command = sub_matches.subcommand().unwrap_or(("help", sub_matches));

            match bank_command{
                ("transfer", sub_matches) => {
                    let destination_addr = sub_matches.get_one::<String>("toAddressFlag");
                    if destination_addr.is_none(){
                        println!("flag toAddress is none.");
                    }
                    else{
                        let def_dest_addr = "0x0000".to_string();
                        let destination_addr = destination_addr.unwrap_or(&def_dest_addr);
                        println!("{destination_addr}")
                    }

                    let amount = sub_matches.get_one::<String>("amountFlag");
                    if amount.is_none(){
                        println!("flag amount is none.");
                    }
                    else{
                        let def_amount = "10".to_string();
                        let amount = amount.unwrap_or(&def_amount);
                        println!("{amount}")
                    }
                    
                }

                ("balance", sub_matches) => {
                    let address = sub_matches.get_one::<String>("addressFlag");
                    if address.is_none(){
                        println!("flag address is none.");
                    }
                    else{
                        let def_addr = "0x0000".to_string();
                        let address = address.unwrap_or(&def_addr);
                    }
                }

                (&_, _) => {
                    println!("something")
                }
            }
        }
        
        Some(("create-keystore", sub_matches)) => {
            let files = sub_matches.get_many::<String>("privKeyFileFlag")
                .map(|vals| vals.collect::<Vec<_>>())
                .unwrap_or_default();

            let key_path = "src/".to_owned() + files[0];
            // println!("{password_path}");

            let config_path = "src/config.toml".to_string();
            // let content = fs::read_to_string(password_path)
            //     .expect("Should have been able to read the file");
            // println!("{content}");

            generate_key(key_path, config_path);
        }

        Some((&_, _)) => {
            println!("something")
        }
        None => {
            println!("nothing")
        }
    }
}