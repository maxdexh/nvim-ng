use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct VimApi {
        nvim_create_autocmd: LuaCallable<(LuaString, LuaStruct<AutoCmdOpts>), ()>,
    }
});

crate::utils::builder_struct!(
    AutoCmdOpts,
    [
        (callback, C, LuaCallable<(), ()>, with_callback),
        (once, O, Option<bool>, with_once),
        (pattern, P, Option<LuaString>, with_pattern),
    ]
);
