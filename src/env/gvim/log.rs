use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct VimLogLevels {
        #[expect(unused)]
        OFF: LuaInt,
        #[expect(unused)]
        TRACE: LuaInt,
        #[expect(unused)]
        DEBUG: LuaInt,
        #[expect(unused)]
        INFO: LuaInt,
        #[expect(unused)]
        WARN: LuaInt,
        ERROR: LuaInt,
    }
});

crate::utils::from_tbl_proxy!({
    struct VimLog {
        levels: VimLogLevels,
    }
});
