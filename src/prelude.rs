pub use crate::env::{AutoCmdOpts, Nvim, NvimConf, PackOpts};

pub(crate) use crate::utils::{LuaDeferErr, LuaTableInit, ResultExt, defer_lua_val, tbl, tbl_seq};

pub use mlua::{FromLua, FromLuaMulti, IntoLua, IntoLuaMulti, Lua, ObjectLike, UserData};

pub use crate::lua::{
    LuaCallable, LuaError, LuaFunc, LuaInt, LuaNil, LuaString, LuaSub, LuaTable, LuaTableMap,
    LuaValue, Result,
};

pub fn do_try<T>(f: impl FnOnce() -> Result<T>) -> Result<T> {
    f()
}
