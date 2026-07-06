use crate::prelude::*;

#[mlua::lua_module]
fn nvim_config(lua: &Lua) -> Result<LuaTop> {
    let globals = lua.unpack(LuaTop::Table(lua.globals()))?;
    let env = Nvim {
        lua: lua.clone(),
        globals,
        req_cache: Default::default(),
    };
    env.load_init();
    Ok(LuaTop::Nil)
}
