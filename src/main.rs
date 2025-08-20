use std::{fs, path::PathBuf, str::FromStr};
use clap::{Parser, Subcommand};
use hidden_text::{encode_hidden, decode_hidden};
use env_logger;
use log::LevelFilter;
use atty::Stream;

#[derive(Parser, Debug)]
#[command(name = "hidden", version, about = "Decodes/Encodes text into zero width characters")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(flatten)]
    global_flags: GlobalFlags
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Encode a string into hidden text")]
    Encode {
        #[arg(short = 'o', long = "output")]
        output: Option<String>,

        #[arg(short = 'p', long = "plain-text")]
        plain_text: Option<String>,

        #[arg(short = 'c', long = "copy")]
        copy: bool,

        #[arg(short = 'l', long = "low")]
        low_char: Option<char>,

        #[arg(short = 'h', long = "high")]
        high_char: Option<char>,

        text: String
    },

    #[command(about = "Extract and decode hidden text from a string")]
    Decode {
        text: String,

        #[arg(short = 'l', long = "low")]
        low_char: Option<char>,

        #[arg(short = 'h', long = "high")]
        high_char: Option<char>,
    }
}

#[derive(Parser, Debug)]
struct GlobalFlags {
    #[clap(short, long, default_value_t=false)]
    verbose: bool
}


fn main() {
    let args = Cli::parse();

    // set verbosity level
    let level = match (atty::is(Stream::Stdout), args.global_flags.verbose) {
        (true, _) => LevelFilter::Off,
        (_, false) => LevelFilter::Warn,
        (_, true) => LevelFilter::Info
    };
    env_logger::builder()
        .format_timestamp(None)
        .filter_level(level)
        .init();

    match args.command {
        Commands::Encode { output, 
                            copy, 
                            text, 
                            plain_text, 
                            low_char, 
                            high_char } => {
            let mut encoded_text = match encode_hidden(text, low_char, high_char) {
                Some(text) => text,
                None => {
                    log::warn!("No text provided");
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
                        log::warn!("Invalid path");
                        return
                    }
                };
                match fs::write(&path, encoded_text) {
                    Ok(_) => {log::info!("Wrote encoded string to {}", path.to_str().unwrap_or(""))},
                    Err(err) => {log::warn!("Failed to write file to {}\nError: {err}", path.to_str().unwrap_or(""))}
                }
            }
            if copy {
                println!("This is where we would copy");
            }
        },
        Commands::Decode { text, low_char, high_char } => {
            match decode_hidden(text, low_char, high_char) {
                Ok(Some(text)) => println!("{text}"),
                Ok(None) => log::warn!("No hidden text found"),
                Err(err) => log::warn!("Error decoding hidden text: {}", err)
            };
        }
    }   
    
}
