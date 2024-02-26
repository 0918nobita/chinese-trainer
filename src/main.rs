use apache_avro::{types::Record, Codec, Schema, Writer};
use clap::{Parser, Subcommand};
use mlua::Lua;
use std::fs::{self, File};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Write {},
    /// Execute User Script
    Exec {},
    Quiz {
        #[command(subcommand)]
        command: QuizCommands,
    },
}

#[derive(Subcommand)]
enum QuizCommands {
    /// List available quiz bundles
    List {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Write {} => {
            write();
        }
        Commands::Exec {} => {
            exec();
        }
        Commands::Quiz {
            command: QuizCommands::List {},
        } => {
            quiz_list();
        }
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

    let schema = Schema::parse_str(raw_schema).unwrap();

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

fn exec() {
    let lua = Lua::new();
    chinese_trainer::init_lua(&lua).expect("Failed to initialize Lua runtime");

    let src = fs::read_to_string("script.lua").unwrap();
    let chunk = lua.load(src).set_name("script.lua");

    match chunk.exec() {
        Ok(()) => {
            println!("Complete");
        }
        Err(err) => {
            eprintln!("Error occurred: {}", err)
        }
    }
}

fn quiz_list() {}
