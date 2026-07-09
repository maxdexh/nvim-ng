use crate::prelude::*;

crate::utils::builder_struct!(
    PackOpts,
    [
        (src, S, LuaString, with_src),
        (version, V, LuaVal, with_version),
    ]
);

crate::utils::from_tbl_proxy!({
    struct VimPack {
        add: LuaCallable<LuaSeq<LuaUnion<LuaStruct<PackOpts>, LuaString>>, ()>,
    }
});
