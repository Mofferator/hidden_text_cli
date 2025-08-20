use std::{fs, path::PathBuf, str::FromStr};
use clap::{Parser, Subcommand};
use hidden_text::{encode_hidden, decode_hidden};

#[derive(Parser, Debug)]
#[command(name = "hidden", version, about = "Decodes/Encodes text into zero width characters")]
struct Cli {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    Encode {
        #[arg(short = 'o', long = "output")]
        output: Option<String>,

        #[arg(short = 'p', long = "plain-text")]
        plain_text: Option<String>,

        #[arg(short = 'c', long = "copy")]
        copy: bool,

        text: String
    },

    Decode {
        text: String
    }
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Encode { output, copy, text, plain_text } => {
            let mut encoded_text = match encode_hidden(text) {
                Some(text) => text,
                None => {
                    eprintln!("No text provided");
                    return;
                }
            };
            if let Some(plain) = plain_text {
                encoded_text += plain.as_str();
            }
            if let Some(output) = output {
                let path:PathBuf = match PathBuf::from_str(output.as_str()) {
                    Ok(path) => path,
                    Err(_) => {
                        eprintln!("Invalid path");
                        return
                    }
                };
                match fs::write(&path, encoded_text) {
                    Ok(_) => {println!("Wrote encoded string to {}", path.to_str().unwrap_or(""))},
                    Err(err) => {eprintln!("Failed to write file to {}\nError: {err}", path.to_str().unwrap_or(""))}
                }
            }
            if copy {
                println!("This is where we would copy");
            }
        },
        Commands::Decode { text } => {
            match decode_hidden(text) {
                Ok(Some(text)) => println!("{text}"),
                Ok(None) => eprintln!("No hidden text found"),
                Err(err) => eprintln!("Error decoding hidden text: {}", err)
            };
        }
    }   
    
}
