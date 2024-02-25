use mlua::{Lua, UserData, UserDataRef};
use std::ops::Deref;

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

pub fn init_lua() -> Lua {
    let lua = Lua::new();

    let create_quiz_bundle = lua.create_function(|_, ()| Ok(QuizBundle)).unwrap();

    let schedule_quiz_bundle = lua
        .create_function(
            |_,
             (quiz_bundle, schedule): (
                UserDataRef<'_, QuizBundle>,
                UserDataRef<'_, QuizBundleSchedule>,
            )| {
                println!("Quiz bundle: {:?}", quiz_bundle.deref());
                println!("Schedule: {:?}", schedule.deref());
                Ok(())
            },
        )
        .unwrap();

    let schedule_immediate = lua.create_userdata(QuizBundleSchedule::Immediate).unwrap();

    let schedule = lua.create_table().unwrap();
    schedule.set("immediate", schedule_immediate).unwrap();

    let fc = lua.create_table().unwrap();
    fc.set("create_quiz_bundle", create_quiz_bundle).unwrap();
    fc.set("schedule", schedule).unwrap();
    fc.set("schedule_quiz_bundle", schedule_quiz_bundle).unwrap();

    lua.globals().set("fc", fc).unwrap();
    lua
}
