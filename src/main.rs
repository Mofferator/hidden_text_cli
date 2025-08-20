use std::{fs, io::{self, Read}, path::PathBuf, str::FromStr};
use clap::{Parser, Subcommand};
use hide::{encode_hidden, decode_hidden};
use log::LevelFilter;
use atty::Stream;
use std::process;

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

        #[arg(short = 'L', long = "low")]
        low_char: Option<char>,

        #[arg(short = 'H', long = "high")]
        high_char: Option<char>,

        text: Option<String>
    },

    #[command(about = "Extract and decode hidden text from a string")]
    Decode {
        #[arg(short = 'L', long = "low")]
        low_char: Option<char>,

        #[arg(short = 'H', long = "high")]
        high_char: Option<char>,

        text: Option<String>
    }
}

#[derive(Parser, Debug)]
struct GlobalFlags {
    #[clap(short, long, default_value_t=false)]
    verbose: bool
}

fn get_text_or_stdin(text: Option<String>) -> io::Result<String> {
    match text {
        Some(s) => Ok(s),
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }
}

fn main() {
    let args = Cli::parse();

    let is_being_piped = !atty::is(Stream::Stdout);
    let level = match (is_being_piped, args.global_flags.verbose) {
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
            let input_text = match get_text_or_stdin(text) {
                Ok(text) => text,
                Err(err) => {
                    log::warn!("Error reading from stdin: {err}");
                    process::exit(1);
                }
            };
            let mut encoded_text = match encode_hidden(input_text, low_char, high_char) {
                Some(text) => text,
                None => {
                    log::warn!("No text provided");
                    process::exit(1);
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
                        process::exit(1);
                    }
                };
                match fs::write(&path, &encoded_text) {
                    Ok(_) => {log::info!("Wrote encoded string to {}", path.to_str().unwrap_or(""))},
                    Err(err) => {log::warn!("Failed to write file to {}\nError: {err}", path.to_str().unwrap_or(""))}
                }
            }
            if copy {
                println!("This is where we would copy");
            }
            println!("{encoded_text}")
        },
        Commands::Decode { text, low_char, high_char } => {
            let input_text = match get_text_or_stdin(text) {
                Ok(text) => text,
                Err(err) => {
                    log::warn!("Error reading from stdin: {err}");
                    process::exit(1);
                }
            };
            match decode_hidden(input_text, low_char, high_char) {
                Ok(Some(text)) => println!("{text}"),
                Ok(None) => log::warn!("No hidden text found"),
                Err(err) => log::warn!("Error decoding hidden text: {err}")
            };
        }
    }   
}
