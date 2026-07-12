use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct VimLspBuf {
        code_action: LuaCallable<(), ()>,
        rename: LuaCallable<(), ()>,
        hover: LuaCallable<(), ()>,
        signature_help: LuaCallable<(), ()>,
    }
});

crate::utils::builder_struct!({
    struct VimLspConfig {
        cmd: Option<LuaSeq<LuaString>>,
        filetypes: Option<LuaSeq<LuaString>>,
        settings: Option<LuaDict<LuaVal>>,
        on_init: Option<LuaCallable<LuaDictMut<LuaVal>, ()>>,
    }
});

crate::utils::from_tbl_proxy!({
    struct VimLsp {
        buf: VimLspBuf,
        enable: LuaCallable<(LuaString, Option<bool>), ()>,
        config: LuaCallable<(LuaString, LuaStruct<VimLspConfig>), ()>,
    }
});
