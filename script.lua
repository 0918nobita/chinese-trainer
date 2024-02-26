local store =
    fc.load_store {
        path = "zhCnWords.avro",
        fingerprint = "a1c786768aa62b7395d18286e42a7e4acd5309dfdb72e07a12061fcd6d5eab0b",
    }

local quiz_bundle = fc.create_quiz_bundle()

local schedule = fc.schedule.immediate

fc.schedule_quiz_bundle(quiz_bundle, schedule)
