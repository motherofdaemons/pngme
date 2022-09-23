use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Args, Debug, PartialEq)]
pub struct EncodeArgs {
    pub input_file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file_path: Option<PathBuf>,
}

#[derive(Args, Debug, PartialEq)]
pub struct DecodeArgs {
    pub input_file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct RemoveArgs {
    pub input_file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct PrintArgs {
    pub input_file_path: PathBuf,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    pub fn test_encode_command() {
        let expected = Commands::Encode(EncodeArgs {
            input_file_path: PathBuf::from("/a/b/c"),
            chunk_type: "RuSt".to_string(),
            message: "Secret decoder ring".to_string(),
            output_file_path: None,
        });
        let cli = Cli::from_iter(vec![
            "pngme",
            "encode",
            "/a/b/c",
            "RuSt",
            "Secret decoder ring",
        ]);
        let actual = cli.command;

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_decode_command() {
        let expected = Commands::Decode(DecodeArgs {
            input_file_path: PathBuf::from("/a/b/c"),
            chunk_type: "RuSt".to_string(),
        });
        let cli = Cli::from_iter(vec![
            "pngme",
            "decode",
            "/a/b/c",
            "RuSt",
        ]);
        let actual = cli.command;

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_remove_command() {
        let expected = Commands::Remove(RemoveArgs {
            input_file_path: PathBuf::from("/a/b/c"),
            chunk_type: "RuSt".to_string(),
        });
        let cli = Cli::from_iter(vec![
            "pngme",
            "remove",
            "/a/b/c",
            "RuSt",
        ]);
        let actual = cli.command;

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_print_command() {
        let expected = Commands::Print(PrintArgs {
            input_file_path: PathBuf::from("/a/b/c"),
        });
        let cli = Cli::from_iter(vec![
            "pngme",
            "print",
            "/a/b/c",
        ]);
        let actual = cli.command;

        assert_eq!(expected, actual);
    }
}
