use apache_avro::{
    types::{Record, Value},
    Codec, Reader, Schema, Writer,
};
use clap::{Parser, Subcommand};
use mlua::Lua;
use sha2::Sha256;
use std::fs::{self, File};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Read {},
    Write {},
    Ask {},
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
        Some(Commands::Ask {}) => {
            ask();
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

fn ask() {
    let words = File::open("zhCnWords.avro").unwrap();

    let reader = Reader::new(words).unwrap();

    let schema = reader.writer_schema();
    let fingerprint = schema.fingerprint::<Sha256>();
    println!("Fingerprint: {}", fingerprint);

    for value in reader {
        let value = value.unwrap();
        match value {
            Value::Record(fields) => {
                println!("{:?}", fields);
            }
            _ => {}
        }
    }

    let lua = Lua::new();

    let greet_func = lua
        .create_function(|_, name: String| {
            println!("Hello, {}!", name);
            Ok(())
        })
        .unwrap();

    lua.globals().set("greet", greet_func).unwrap();

    lua.load(fs::read_to_string("script.lua").unwrap())
        .exec()
        .unwrap();
}
