use crate::prelude::*;

#[mlua::lua_module]
fn nvim_config(lua: &Lua) -> Result<LuaValue> {
    let globals = lua.unpack(LuaValue::Table(lua.globals()))?;
    let env = Nvim {
        lua: lua.clone(),
        shared: crate::env::NvimEnv {
            globals,
            req_cache: Default::default(),
        },
    };
    env.load_init();
    Ok(LuaValue::Nil)
}
