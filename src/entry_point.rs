use std::cell::OnceCell;

use crate::prelude::*;

#[mlua::lua_module]
fn nvim_config(lua: &Lua) -> Result<LuaVal> {
    THREAD_LUA.with(|it| _ = it.get_or_init(|| lua.weak()));
    set_panic_hook();

    let globals = lua.unpack(LuaVal::Table(lua.globals()))?;
    let env = Nvim {
        lua: lua.clone(),
        globals,
        req_cache: Default::default(),
    };
    env.load_init();
    Ok(LuaVal::Nil)
}

thread_local! {
    static THREAD_LUA: OnceCell<mlua::WeakLua> = const { OnceCell::new() };
}

fn set_panic_hook() {
    std::panic::set_hook(Box::new(move |info| {
        let msg = info.payload_as_str().unwrap_or("panic of non-string type");

        crate::env::lua_notify_err(
            THREAD_LUA
                .with(|it| it.get().and_then(|it| it.try_upgrade()))
                .as_ref(),
            msg,
        );
    }));
}
