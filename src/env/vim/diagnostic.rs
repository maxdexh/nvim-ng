use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct VimDiagnostic {
        config: LuaCallable<LuaDict<LuaVal>, ()>,
    }
});
