use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct VimApi {
        nvim_create_autocmd: LuaCallable<(LuaString, LuaStruct<AutoCmdOpts>), ()>,
    }
});
crate::utils::builder_struct!({
    struct AutoCmdOpts {
        #[with = with_callback]
        callback: LuaCallable<(), ()>,

        #[with = with_once]
        once: Option<bool>,

        #[with = with_pattern]
        pattern: Option<LuaString>,
    }
});
