pub use crate::env::{AutoCmdOpts, Nvim, NvimConf, PackOpts};

pub(crate) use crate::utils::{LuaTableInit, ResultExt, tbl, tbl_seq};

pub use mlua::{FromLua, FromLuaMulti, IntoLua, IntoLuaMulti, Lua, ObjectLike, UserData};

pub use crate::lua::{
    LuaCallable, LuaDeferErr, LuaError, LuaIgnoreSub, LuaInt, LuaNil, LuaString, LuaSub, LuaTable,
    LuaTableMap, LuaValue, Result, defer_lua_val, lua_conv_sub,
};

pub fn do_try<T>(f: impl FnOnce() -> Result<T>) -> Result<T> {
    f()
}
