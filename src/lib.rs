// use bucket::new_bucket;
pub use clap::{Command,arg, value_parser};

pub mod bucket;
pub mod object;
pub mod bank;
// pub mod 
pub fn init() -> Command {
    Command::new("gnfd-cmd-rs")
        .subcommands([
            bucket::new(),
            object::new(),
            Command::new("group"),
            Command::new("crosschain"),
            bank::new(),
            Command::new("policy"),
            Command::new("payment"),
            Command::new("sp"),
            Command::new("create-keystore"),
        ])
}