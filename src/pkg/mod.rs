use clap::{command, arg, Command, value_parser};
use std::fs;
// use serde::Deserialize;
use secp256k1::{SecretKey, PublicKey};

//extern crate rustc_serialize;
use hex;

#[derive(serde::Deserialize)]
pub struct CliConfig{
    _rpc_addr: String,
    _chain_id: String,
    password_file: String,
}

// #[derive(ValueEnum, Clone)]
// pub enum VisbilityType {
//     PublicRead,
//     Private,
//     Inherit,
// }

pub fn init() -> Command{
    command!()
    .arg_required_else_help(true)
    .subcommand(
        Command::new("bucket")
            .about("support the bucket operation functions, including create/update/delete/head/list")
            .subcommand(
                Command::new("create")
                    .about("create a new bucket")
                    .long_about("Create a new bucket and set a createBucketMsg to storage provider.
                    The bucket name should unique and the default visibility is private.
                    The command need to set the primary SP address with --primarySP.
                    
                    Examples:
                    # Create a new bucket called gnfd-bucket, visibility is public-read
                    $ gnfd-cmd bucket create --visibility=public-read  gnfd://gnfd-bucket")
                    .args([
                        arg!(bucket_url: <"BUCKET-URL">).required(true)
                            .value_parser(value_parser!(String)),
                        arg!(--primary_sp [PRIMARY_SP] "indicate the primarySP address, using the string type")
                            .value_parser(value_parser!(String)).required(false).require_equals(true)
                            .default_value(""),
                        arg!(--payment_address [PAYMENT_ADDR] "indicate the PaymentAddress info, using the string type")
                            .value_parser(value_parser!(String)).required(false)
                            .default_value(""),
                        arg!(--charged_quota [CHARGE_QUOTA] "indicate the read quota info of the bucket")
                            .value_parser(value_parser!(u64)).required(false).require_equals(true),
                        arg!(--visibility [VISIBILITY] "set visibility of the bucket")
                            .value_parser(value_parser!(String)).required(false).require_equals(true)
                            .default_value("private"),
                    ])
            )
            .subcommand(
<<<<<<< HEAD
                Command::new("update")
                .about("update bucket meta on chain")
                .long_about("Update the visibility, payment account or read quota meta of the bucket.
                The visibility value can be public-read, private or inherit.
                You can update only one item or multiple items at the same time.
                
                Examples:
                update visibility and the payment address of the gnfd-bucket
                $ gnfd-cmd bucket update --visibility=public-read --paymentAddress xx  gnfd://gnfd-bucket")
                .args([
                    arg!(bucket_url: <"BUCKET-URL">).required(true)
                        .value_parser(value_parser!(String)),
                    arg!(--payment_address [PAYMENT_ADDR] "indicate the PaymentAddress info, using the string type")
                        .value_parser(value_parser!(String)).required(false)
                        .default_value(""),
                    arg!(--charged_quota [CHARGE_QUOTA] "indicate the read quota info of the bucket")
                        .value_parser(value_parser!(u64)).required(false).require_equals(true),
                    arg!(--visibility [VISIBILITY] "set visibility of the bucket")
                        .value_parser(value_parser!(String)).required(false).require_equals(true)
                        .default_value("private"),
                ])
            )
            .subcommand(
                Command::new("delete")
                .about("delete an existed bucket")
                .long_about("Send a deleteBucket txn to greenfield chain, the bucket must be empty before deleting

                Examples:
                # Delete an existed bucket called gnfd-bucket
                $ gnfd-cmd bucket delete gnfd://gnfd-bucket/gnfd-object")
                .arg(arg!(bucket_url: <"BUCKET-URL">).required(true)
                    .value_parser(value_parser!(String)))
            )
            .subcommand(
                Command::new("head")
                .about("query bucket info")
                .long_about("send headBucket txn to chain and fetch bucket info on greenfield chain
                Examples:
                $ gnfd-cmd bucket head gnfd://bucket-name")
                .arg(arg!(bucket_url: <"BUCKET-URL">).required(true)
                    .value_parser(value_parser!(String)))
            )
            .subcommand(
                Command::new("ls")
                .about("list buckets")
                .long_about("List the bucket names and bucket ids of the user.

                Examples:
                $ gnfd-cmd bucket ls")
=======
                Command::new("ls")
                    .about("list buckets")
                    .long_about("List the bucket names and bucket ids of the user.
                    Examples:
                    $ gnfd-cmd bucket ls")
>>>>>>> c5ec25baaeae393efd4662e42c9cbd9d992e646b
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

    )
}

pub fn parse_config(path: String) -> CliConfig{
    let content = fs::read_to_string(path)
        .expect("can't read config file");

    let config: CliConfig = toml::from_str(content.as_str()).unwrap();

    // println!("{} - {}", config.rpcAddr, config.chainId);
    
    return config
}

pub fn generate_key(key_file: String, config_file: String){
    let config = parse_config(config_file);

    let private_key =  fs::read_to_string(key_file)
        .expect("can't read password file");

    // println!("{private_key}");

    // TODO! add checking len of addr equal 32
    let addr = hex::decode(&private_key).expect("can't decode private key to hex");
    //println!("{addr:?}");

    let _secp256k1_private = generate_secp256k1(&addr).unwrap();
    //println!("{secp256k1_private}");

    let _password = fs::read_to_string("src/".to_owned() + &config.password_file).expect("can't read password from file");
    //println!("{password}");


}

fn generate_secp256k1(private_key: &Vec<u8>) -> Result<String, String>{
    let context = secp256k1::Secp256k1::new();
    let secret_key = SecretKey::from_slice(private_key).expect("can't create SecretKey");
    let public_key = PublicKey::from_secret_key(&context, &secret_key);

    //println!("{:?}", public_key.to_string());
    return Ok(public_key.to_string())
}


// // Key derives a key from the password, salt, and cost parameters, returning
// // a byte slice of length keyLen that can be used as cryptographic key.
// fn Key(addr: &[u8], private_key: &[u8], n, r, p, key_len: i32) -> Result<String, String>{
//     if n < 1 || n & (n - 1) != 0{
//         return Err("scrypt: n must be > 1 and a power of 2");
//     }

//     if r as u64 * p as u64 >= 1 << 30 || 
// }