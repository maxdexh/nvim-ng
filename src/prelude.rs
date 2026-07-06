pub use crate::env::{AutoCmdOpts, Nvim, NvimConf, PackOpts};

pub(crate) use crate::utils::{LuaTableInit, ResultExt, tbl, tbl_seq};

pub use mlua::{Lua, ObjectLike, UserData};

pub use crate::lua::{
    LuaCallable, LuaCastIntoAny, LuaDeferErr, LuaError, LuaInt, LuaNil, LuaString, LuaTableMap,
    LuaTop, LuaTopTable, Result, defer_lua_val, lua_conv_sub,
};
pub use crate::typing::{FromLuaMultiTyped, FromLuaTyped, IntoLuaMultiTyped, IntoLuaTyped, LuaSub};

pub fn do_try<T>(f: impl FnOnce() -> Result<T>) -> Result<T> {
    f()
}
