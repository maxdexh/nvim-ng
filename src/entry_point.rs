use crate::prelude::*;

#[mlua::lua_module]
fn nvim_config(lua: &Lua) -> Result<LuaVal> {
    let globals = lua.unpack(LuaVal::Table(lua.globals()))?;
    let env = Nvim {
        lua: lua.clone(),
        globals,
        req_cache: Default::default(),
    };
    env.load_init();
    Ok(LuaVal::Nil)
}
