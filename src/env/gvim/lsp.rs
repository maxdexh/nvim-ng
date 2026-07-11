use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct VimLspBuf {
        code_action: LuaCallable<(), ()>,
        rename: LuaCallable<(), ()>,
        hover: LuaCallable<(), ()>,
        signature_help: LuaCallable<(), ()>,
    }
});

crate::utils::from_tbl_proxy!({
    struct VimLsp {
        buf: VimLspBuf,
    }
});
