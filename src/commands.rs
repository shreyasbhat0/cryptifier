use super::*;

#[derive(Debug, Parser)]
#[clap(
    name = "Cryptifer",
    about = "Cryptifer is a CLI Application to Encrypt and Decrypt the file",
    version = "0.0.1"
)]
pub struct Cryptifer {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generates Keystore to out file given
    #[clap(arg_required_else_help = true)]
    Generate {
        #[clap(short = 'o', long)]
        output_path: String,
    },
    /// Encrypts file specified using keypath
    #[clap(arg_required_else_help = true)]
    Encrypt {
        #[clap(short = 'f', long)]
        file_path: String,
        #[clap(short = 'k', long)]
        key_path: String,
    },
    /// Decrypts file specified using keypath
    #[clap(arg_required_else_help = true)]
    Decrypt {
        #[clap(short = 'f', long)]
        encrypted_file: String,
        #[clap(short = 'k', long)]
        key_path: String,
    },
}
