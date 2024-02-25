use apache_avro::{types::Record, Codec, Schema, Writer};
use std::fs::File;

fn main() {
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
