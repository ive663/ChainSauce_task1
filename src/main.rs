use clap::{command, arg, Command, value_parser};
use tonic::{transport::Channel, Request};



fn main() {
    let cli = command!()
    .arg_required_else_help(true)
    .subcommand(
        Command::new("bucket")
            .about("support the bucket operation functions, including create/update/delete/head/list")
            .subcommand(
                Command::new("create")
                    .about("create a new bucket")
                    .args([
                        arg!(primarySPFlag: --primarySP <STORAGE_ADDR> "indicate the primarySP address, using the string type")
                            .value_parser(value_parser!(String))
                    ])
            )
    )
    .subcommand(
        Command::new("bank")
            .about("support the bank functions, including transfer in greenfield and query balance")
            .subcommand(
                Command::new("transfer")
                    .about("transfer from your account to a dest account")
                    .args([
                        arg!(toAddressFlag: --toAddress <DEST_ADDR> "the receiver address in BSC")
                            .value_parser(value_parser!(String)),
                        arg!(amountFlag: --amount <AMOUNT> "the amount to be sent")
                            .value_parser(value_parser!(String))
                    ])
            )
            .subcommand(
                Command::new("balance")
                    .about("query a account's balance")
                    .args([
                        arg!(addressFlag: --address <ADDR> "indicate the address's balance to be retrieved")
                            .value_parser(value_parser!(String))
                    ])
            )
    )
    .subcommand(
        Command::new("create-keystore")
            .about("create a new keystore file")
            .args([
                arg!(privKeyFileFlag: --privKeyFile <privKeyFile>)
                    .num_args(1..=2)
                    .value_parser(value_parser!(String))
            ])

    );

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

        Some(("bank", sub_matches)) =>{
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
        
        Some("create-keystore", sub_matches) => {
            let files = sub_matches.get_many::<String>("privKeyFileFlag");
            let files = files.map(|vals| vals.Collect::Vec<<_>>())
                .unwrap_or_default();
        }

        Some((&_, _)) => {
            println!("something")
        }
        None => {
            println!("nothing")
        }
    }


}

// fn rpcClient(){
//     let channel = Channel::from_static("/cosmos.bank.v1beta1.Query/Balance")
//         .connect()
//         .await?;
//     let mut client = ::new(channel);
//     let request = Request::new();

//     let response = client.send(request).await?.into_inner();
//     println!("RESPONSE={:?}", response);
// }