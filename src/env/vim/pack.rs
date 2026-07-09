use crate::prelude::*;

crate::utils::builder_struct!({
    struct PackOpts {
        src: LuaString,
        version: LuaVal,
    }
});

crate::utils::from_tbl_proxy!({
    struct VimPack {
        add: LuaCallable<
            LuaSeq<
                LuaUnion<
                    LuaStruct<PackOpts>, //
                    LuaString,
                >,
            >,
            (),
        >,
    }
});
