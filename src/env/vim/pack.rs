use crate::prelude::*;

crate::utils::builder_struct!({
    struct PackOpts {
        #[with = with_src]
        src: LuaString,

        #[with = with_version]
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
