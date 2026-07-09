use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct VimPack {
        add: LuaCallable<LuaSeq<LuaUnion<LuaDict<LuaVal>, LuaString>>, ()>,
    }
});
