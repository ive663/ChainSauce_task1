// use clap::{command, arg, Command, value_parser};
// use std::fs;
// use serde::Deserialize;

// #[derive(serde::Deserialize)]
// pub struct CliConfig{
//     rpcAddr: String,
//     chainId: String,
//     passwordFile: String,
// }

// pub fn init() -> Command{
//     command!()
//     .arg_required_else_help(true)
//     .subcommand(
//         Command::new("bucket")
//             .about("support the bucket operation functions, including create/update/delete/head/list")
//             .subcommand(
//                 Command::new("create")
//                     .about("create a new bucket")
//                     .args([
//                         arg!(primarySPFlag: --primarySP <STORAGE_ADDR> "indicate the primarySP address, using the string type")
//                             .value_parser(value_parser!(String))
//                     ])
//             )
//     )
//     .subcommand(
//         Command::new("bank")
//             .about("support the bank functions, including transfer in greenfield and query balance")
//             .subcommand(
//                 Command::new("transfer")
//                     .about("transfer from your account to a dest account")
//                     .args([
//                         arg!(toAddressFlag: --toAddress <DEST_ADDR> "the receiver address in BSC")
//                             .value_parser(value_parser!(String)),
//                         arg!(amountFlag: --amount <AMOUNT> "the amount to be sent")
//                             .value_parser(value_parser!(String))
//                     ])
//             )
//             .subcommand(
//                 Command::new("balance")
//                     .about("query a account's balance")
//                     .args([
//                         arg!(addressFlag: --address <ADDR> "indicate the address's balance to be retrieved")
//                             .value_parser(value_parser!(String))
//                     ])
//             )
//     )
//     .subcommand(
//         Command::new("create-keystore")
//             .about("create a new keystore file")
//             .args([
//                 arg!(privKeyFileFlag: --privKeyFile <privKeyFile>)
//                     .num_args(1..=2)
//                     .value_parser(value_parser!(String))
//             ])

//     )
// }

// pub fn parse_config(path: String) -> CliConfig{
//     let content = fs::read_to_string(path)
//         .expect("can't read config file");

//     let config: CliConfig = toml::from_str(content.as_str()).unwrap();
    
//     return config
// }


// use hex::decode;
// use rand_core::{OsRng, RngCore};
// use password_hash::SaltString;
// pub fn generate_key(key_file: String, config_file: String){
//     let config = parse_config(config_file);

//     let private_key =  fs::read_to_string(key_file)
//         .expect("can't read password file");

//     // println!("{private_key}");

//     // TODO! add checking len of addr equal 32
//     let addr = hex::decode(&private_key).expect("can't decode private key to hex");
//     //println!("{addr:?}");

//     let secp256k1_private = generate_secp256k1(&addr).unwrap();
//     //println!("{secp256k1_private}");

//     let password = fs::read_to_string("src/".to_owned() + &config.passwordFile).expect("can't read password from file");
//     //println!("{password}");

//     let scrypt_n = 262144;
//     let scrypt_p = 1;

//     let mut random = OsRng;
//     let binding = SaltString::generate(&mut random);
//     let salt = binding.as_str().as_bytes();

//     let derived_key = key(password.as_bytes(), salt, scrypt_n, scrypt_p).expect("Derive password with scrypt");
//     // println!("{:?}", derived_key);

//     let encrypt_key = &derived_key[..16];
//     // println!("{:?}", encrypt_key);
    
//     let mut iv = [0u8; 16];
//     random.fill_bytes(&mut iv);
//     //println!("{:?}", iv);

// }

// use secp256k1::{SecretKey, PublicKey};
// fn generate_secp256k1(private_key: &Vec<u8>) -> Result<String, String>{
//     let context = secp256k1::Secp256k1::new();
//     let secret_key = SecretKey::from_slice(private_key).expect("can't create SecretKey");
//     let public_key = PublicKey::from_secret_key(&context, &secret_key);

//     //println!("{:?}", public_key.to_string());
//     return Ok(public_key.to_string())
// }


// use scrypt::{scrypt, Params};
// // Key derives a key from the password, salt, and cost parameters, returning
// // a byte slice that can be used as cryptographic key.
// pub fn key(auth: &[u8], salt: &[u8], scrypt_n: u64, scrypt_p: u32) -> Result<Vec<u8>, String> {
//     // const values
//     let scrypt_r = 8;
//     let scrypt_key_len = 32;

//     let scrypt_n = (scrypt_n as f64).log2() as u8;
//     // Generate Scrypt parameters
//     let params = Params::new(scrypt_n, scrypt_r, scrypt_p, scrypt_key_len)
//         .expect("can't create params for scrypt");

//     // Generate a key using Scrypt
//     let mut derived_key = vec![0u8; 32];
//     scrypt(auth, salt, &params, &mut derived_key)
//         .expect("hashing password using scrypt");

//     Ok(derived_key)
// }


// use aes::{Aes128, cipher::{KeyInit}};
// use ctr::{Ctr64LE};
// type Aes128Ctr64LE = Ctr64LE<Aes128>;
// fn aesCTRXOR(encrypt_key: &[u8], intext: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
//     let aes_block = Aes128::new_from_slice(encrypt_key).expect("create new aes128 block");
    
//     let mut stream = Aes128Ctr64LE::new_from_slice(aes_block, iv);
//     //println!("{stream}")

//     return Ok(Vec::new());
// }

// fn get_ctr_stream_cipher(key: &[u8], iv: &[u8]) -> Result<Ctr<Aes128, NoPadding>, Box<dyn std::error::Error>> {
//     // Create an AES-128 cipher from the key
//     let aes_block = Aes128::new_from_slice(key).expect("create new aes128 block");

//     // Create a CTR cipher mode with the AES block and IV
//     let ctr = CtrCore::<Aes128>::new_var(&aes_block, iv)?;

//     Ok(ctr)
// }
