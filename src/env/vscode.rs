use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct Vscode {
        eval: LuaCallable<LuaString, LuaVal>,
    }
});
