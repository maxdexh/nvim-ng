use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct VimApi {
        nvim_create_autocmd: LuaCallable<(LuaString, LuaStruct<AutoCmdOpts>), ()>,
    }
});
crate::utils::builder_struct!({
    struct AutoCmdOpts {
        callback: LuaCallable<(), ()>,
        once: Option<bool>,
        pattern: Option<LuaString>,
    }
});
