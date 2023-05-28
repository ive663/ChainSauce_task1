// use bucket::new_bucket;
pub use clap::{Command,arg, value_parser};

pub mod bucket;
pub mod object;
pub mod bank;
// pub mod 

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

