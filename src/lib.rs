use apache_avro::{types::Value, Reader, Schema};
use mlua::{Lua, Table, UserData, UserDataRef};
use sha2::Sha256;
use std::{fs::File, ops::Deref};

#[derive(Debug)]
struct QuizBundle;

impl UserData for QuizBundle {}

#[derive(Debug)]
enum QuizBundleSchedule {
    Immediate,
    SpecificDateTime,
    DateTimeOffset,
}

impl UserData for QuizBundleSchedule {}

#[derive(Debug)]
struct Store {
    path: String,
    schema: Schema,
    values: Vec<Value>,
}

impl UserData for Store {}

pub fn init_lua(lua: &Lua) -> mlua::Result<()> {
    let globals = lua.globals();

    let load_store = lua.create_function(|_, options: Table| -> mlua::Result<Store> {
        let Ok(path) = options.get::<_, mlua::String>("path") else {
            return Err(mlua::Error::RuntimeError(
                "path (string) field is required".to_owned(),
            ));
        };

        let Ok(fingerprint) = options.get::<_, mlua::String>("fingerprint") else {
            return Err(mlua::Error::RuntimeError(
                "fingerprint (string) field is required".to_owned(),
            ));
        };

        let path = path.to_str()?;
        let fingerprint = fingerprint.to_str()?;

        let Ok(avro) = File::open(path) else {
            return Err(mlua::Error::RuntimeError(format!(
                "Failed to open `{}`",
                path
            )));
        };

        let reader = Reader::new(avro).unwrap();
        let schema = reader.writer_schema().clone();
        let computed_fingerprint = schema.fingerprint::<Sha256>().to_string();

        if computed_fingerprint != fingerprint {
            return Err(mlua::Error::RuntimeError(format!(
                "Fingerprint mismatch:\n  expected {}\n  got {}",
                fingerprint, computed_fingerprint
            )));
        }

        let mut values = Vec::<Value>::new();

        for value in reader {
            values.push(value.unwrap());
        }

        let store = Store {
            path: path.to_owned(),
            schema,
            values,
        };

        println!("{:?}", store);

        Ok(store)
    })?;

    let create_quiz_bundle = lua.create_function(|_, ()| Ok(QuizBundle))?;

    let schedule_quiz_bundle = lua.create_function(
        |_,
         (quiz_bundle, schedule): (
            UserDataRef<'_, QuizBundle>,
            UserDataRef<'_, QuizBundleSchedule>,
        )| {
            println!("Quiz bundle: {:?}", quiz_bundle.deref());
            println!("Schedule: {:?}", schedule.deref());
            Ok(())
        },
    )?;

    let schedule_immediate = lua.create_userdata(QuizBundleSchedule::Immediate)?;

    let schedule = lua.create_table()?;
    schedule.set("immediate", schedule_immediate)?;

    let fc = lua.create_table()?;
    fc.set("load_store", load_store)?;
    fc.set("create_quiz_bundle", create_quiz_bundle)?;
    fc.set("schedule", schedule)?;
    fc.set("schedule_quiz_bundle", schedule_quiz_bundle)?;

    globals.set("fc", fc)?;

    Ok(())
}
