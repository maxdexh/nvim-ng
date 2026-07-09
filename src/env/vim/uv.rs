use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct VimUV {
        cwd: LuaCallable<(), LuaString>,
    }
});
