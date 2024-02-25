use apache_avro::{types::Record, Codec, Reader, Schema, Writer};
use clap::{Parser, Subcommand};
use std::fs::File;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Read {},
    Write {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Read {}) => {
            read();
        }
        Some(Commands::Write {}) => {
            write();
        }
        None => {
            eprintln!("No subcommand specified");
            std::process::exit(1);
        }
    }
}

fn read() {
    let input = File::open("zhCnWords.avro").unwrap();

    let reader = Reader::new(input).unwrap();

    for value in reader {
        println!("{:?}", value.unwrap());
    }
}

fn write() {
    let raw_schema = r#"
        {
            "name": "ZhCnWord",
            "type": "record",
            "fields": [
                {
                    "name": "zhCn",
                    "type": "string"
                },
                {
                    "name": "jaJp",
                    "type": "string"
                }
            ]
        }
    "#;

    let schema = Schema::parse_str(&raw_schema).unwrap();

    let file = File::create("zhCnWords.avro").unwrap();

    let mut writer = Writer::with_codec(&schema, file, Codec::Deflate);

    let mut record_1 = Record::new(&schema).unwrap();
    record_1.put("zhCn", "提高");
    record_1.put("jaJp", "向上させる");

    writer.append(record_1).unwrap();

    let mut record_2 = Record::new(&schema).unwrap();
    record_2.put("zhCn", "告诉");
    record_2.put("jaJp", "伝える");

    writer.append(record_2).unwrap();

    writer.flush().unwrap();
}
