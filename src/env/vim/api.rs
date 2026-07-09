use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct VimApi {
        nvim_create_autocmd: LuaCallable<(LuaString, LuaDict<LuaVal>), ()>,
    }
});
