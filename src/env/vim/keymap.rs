use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct VimKeymap {
        set: LuaCallable<
            (
                LuaUnion<LuaString, LuaSeq<LuaString>>,
                LuaString,
                LuaUnion<LuaString, LuaCallable<(), ()>>,
                LuaStruct<KeymapOpts>,
            ),
            (),
        >,
    }
});

crate::utils::builder_struct!({
    struct KeymapOpts {
        desc: LuaString,
    }
});
