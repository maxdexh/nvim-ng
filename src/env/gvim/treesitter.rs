use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct VimTreesitterLang {
        get_lang: LuaCallable<LuaString, Option<LuaString>>,
    }
});

crate::utils::from_tbl_proxy!({
    struct VimTreesitterQuery {
        get: LuaCallable<(LuaString, LuaString), Option<LuaVal>>,
    }
});

crate::utils::from_tbl_proxy!({
    struct VimTreesitter {
        start: LuaCallable<(), ()>,
        language: VimTreesitterLang,
        query: VimTreesitterQuery,
    }
});
