use mlua::{Lua, UserData, UserDataMethods};

struct MyStruct(i32);

impl UserData for MyStruct {
    fn add_methods<'lua, M>(methods: &mut M)
    where
        M: UserDataMethods<'lua, Self>,
    {
        methods.add_method("foo", |_, this, ()| {
            println!("Inner value: {}", this.0);
            Ok(())
        })
    }
}

pub fn lua() -> Lua {
    let lua = Lua::new();

    let load_user_data_func = lua.create_function(|_, ()| Ok(MyStruct(42))).unwrap();

    lua.globals()
        .set("load_user_data", load_user_data_func)
        .unwrap();

    lua
}
