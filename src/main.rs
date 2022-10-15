use block_modes::BlockMode;
use clap::{Parser, Subcommand};
use rand::*;
mod commands;
use commands::Cryptifer;
mod encrypt;
mod generate;
use aes::Aes128;
use block_modes::{block_padding::Pkcs7, Cbc};
use encrypt::encrypt;
use generate::Secret;
use serde_derive::{Deserialize, Serialize};
use serde_json::to_vec;
use std::fs::{read, write, File};
use std::io::Error;
use std::io::Write;
use std::str;
mod decrypt;
use decrypt::decrypt;

pub type AES = Cbc<Aes128, Pkcs7>;

fn main() {
    let cryptifer = Cryptifer::parse();

    match cryptifer.command {
        commands::Commands::Generate { output_path } => {
            Secret::new(output_path).unwrap();
        }
        commands::Commands::Encrypt {
            file_path,
            key_path,
        } => {
            encrypt(file_path, key_path).unwrap();
        }
        commands::Commands::Decrypt {
            encrypted_file,
            key_path,
        } => decrypt(encrypted_file, key_path).unwrap(),
    }
}
