use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct VimVersion {
        range: LuaCallable<LuaString, LuaVal>,
    }
});
