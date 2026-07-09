use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct VimKeymap {
        set: LuaCallable<
            (
                LuaUnion<LuaString, LuaSeq<LuaString>>,
                LuaString,
                LuaVal,
                LuaDict<LuaVal>,
            ),
            (),
        >,
    }
});
