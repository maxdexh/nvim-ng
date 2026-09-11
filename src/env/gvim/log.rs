use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct VimLogLevels {
        #[expect(unused)]
        OFF: LuaInt,
        #[expect(unused)]
        TRACE: LuaInt,
        #[expect(unused)]
        DEBUG: LuaInt,
        INFO: LuaInt,
        WARN: LuaInt,
        ERROR: LuaInt,
    }
});

crate::utils::from_tbl_proxy!({
    struct VimLog {
        levels: VimLogLevels,
    }
});
