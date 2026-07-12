use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct VimApi {
        nvim_create_autocmd: LuaCallable<(LuaString, LuaStruct<AutoCmdOpts>), ()>,
        nvim_set_hl: LuaCallable<(LuaInt, LuaString, LuaStruct<HighlightOpts>), ()>,
    }
});
crate::utils::builder_struct!({
    struct AutoCmdOpts {
        callback: LuaUnion<LuaString, LuaCallable<LuaStruct<AutoCmdArgs>, ()>>,
        once: Option<bool>,
        pattern: Option<LuaString>,
    }
});
crate::utils::from_tbl_struct!({
    struct AutoCmdArgs {
        buf: LuaInt,
        r#match: LuaString,
    }
});

crate::utils::builder_struct!({
    struct HighlightOpts {
        underline: Option<bool>,
        sp: Option<LuaString>,
        link: Option<LuaString>,
        fg: Option<LuaString>,
    }
});
