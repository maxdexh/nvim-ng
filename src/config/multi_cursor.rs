use crate::{env::gvim::keymap::KeymapOpts, prelude::*};

type NvimKeymapSet = LuaCallable<
    (
        LuaUnion<LuaString, LuaSeq<LuaString>>,
        LuaString,
        LuaUnion<LuaString, LuaCallable<(), ()>>,
        LuaStruct<KeymapOpts>,
    ),
    (),
>;
crate::utils::from_tbl_proxy!({
    struct Multicursor {
        setup: LuaCallable<LuaDict<LuaVal>, ()>,
        addKeymapLayer: LuaCallable<LuaCallable<NvimKeymapSet, ()>, ()>,
        cursorsEnabled: LuaCallable<(), bool>,
        enableCursors: LuaCallable<(), ()>,
        clearCursors: LuaCallable<(), ()>,
        handleMouse: LuaCallable<(), ()>,
        handleMouseRelease: LuaCallable<(), ()>,
        handleMouseDrag: LuaCallable<(), ()>,
        lineAddCursor: LuaCallable<LuaInt, ()>,
        lineSkipCursor: LuaCallable<LuaInt, ()>,
        matchAddCursor: LuaCallable<LuaInt, ()>,
        matchSkipCursor: LuaCallable<LuaInt, ()>,
        prevCursor: LuaCallable<(), ()>,
        nextCursor: LuaCallable<(), ()>,
        deleteCursor: LuaCallable<(), ()>,
    }
});

impl NvimConf<'_> {
    fn req_mc(&self) -> Result<Multicursor> {
        self.setup_plugin::<Multicursor>("multicursor-nvim", |mc| {
            mc.setup()?.call(tbl!(owned, {}))?;
            mc.addKeymapLayer()?.call(self.create_cb(|conf, setter| {
                set_mc_layer(&conf, setter);
                Ok(())
            }))
        })
    }
    pub fn load_multicursor(&self) {
        self.add_packs(["https://github.com/jake-stewart/multicursor.nvim"]);

        self.on_very_lazy(|conf| {
            conf.req_mc()?;
            Ok(())
        })
        .ok_or_notify(self);

        set_mc_maps(self);
    }
}

fn set_mc_layer(conf: &NvimConf, setter: NvimKeymapSet) {
    fn set(
        conf: &NvimConf,
        setter: &NvimKeymapSet,
        modes: impl LuaSub<LuaUnion<LuaString, LuaSeq<LuaString>>>,
        seq: impl LuaSub<LuaString>,
        act: impl LuaSub<LuaUnion<LuaString, LuaCallable<(), ()>>>,
        opts: impl LuaSub<LuaStruct<KeymapOpts>>,
    ) {
        setter.call((modes, seq, act, opts)).ok_or_notify(conf);
    }
    set(
        conf,
        &setter,
        "n",
        "<ESC>",
        conf.create_cb(|conf, ()| {
            let mc = conf.req_mc()?;
            if mc.cursorsEnabled()?.call(())? {
                mc.clearCursors()?.call(())
            } else {
                mc.enableCursors()?.call(())
            }
        }),
        mk_builder!(KeymapOpts, {
            desc = "clear cursors";
        }),
    );
    if let Some(mc) = conf.req_mc().ok_or_notify(conf) {
        set(
            conf,
            &setter,
            ["n", "x"],
            "<left>",
            LuaDeferErr(mc.prevCursor()),
            mk_builder!(KeymapOpts, {
                desc = "previous cursor";
            }),
        );
        set(
            conf,
            &setter,
            ["n", "x"],
            "<right>",
            LuaDeferErr(mc.nextCursor()),
            mk_builder!(KeymapOpts, {
                desc = "next cursor";
            }),
        );
        set(
            conf,
            &setter,
            ["n", "x"],
            "<M-x>",
            LuaDeferErr(mc.deleteCursor()),
            mk_builder!(KeymapOpts, {
                desc = "delete cursor";
            }),
        );
    }
}
fn set_mc_maps(conf: &NvimConf) {
    fn set(
        conf: &NvimConf,
        modes: impl LuaSub<LuaUnion<LuaString, LuaSeq<LuaString>>>,
        seq: impl LuaSub<LuaString>,
        desc: impl LuaSub<LuaString>,
        act: impl Fn(&Multicursor) -> Result<()> + 'static,
    ) {
        conf.set_keymap(
            modes,
            seq,
            conf.create_cb(move |conf, ()| act(&conf.req_mc()?)),
            mk_builder!(KeymapOpts, {
                desc = desc;
            }),
        );
    }
    set(conf, "n", "<C-leftmouse>", "Add cursor", |mc| {
        mc.handleMouse()?.call(())
    });
    set(conf, "n", "<C-leftdrag>", "Add cursor", |mc| {
        mc.handleMouseDrag()?.call(())
    });
    set(conf, "n", "<C-leftrelease>", "Add cursor", |mc| {
        mc.handleMouseRelease()?.call(())
    });
    set(conf, "n", "<M-j>", "Add cursor below", |mc| {
        mc.lineAddCursor()?.call(1)
    });
    set(conf, "n", "<M-k>", "Add cursor above", |mc| {
        mc.lineAddCursor()?.call(-1)
    });
    set(conf, "n", "<M-J>", "Skip cursor below", |mc| {
        mc.lineSkipCursor()?.call(1)
    });
    set(conf, "n", "<M-K>", "Skip cursor above", |mc| {
        mc.lineSkipCursor()?.call(-1)
    });
    set(
        conf,
        ["n", "x"],
        "<M-n>",
        "Add cursor on next occurrence",
        |mc| mc.matchAddCursor()?.call(1),
    );
    set(
        conf,
        ["n", "x"],
        "<M-p>",
        "Add cursor on previous occurrence",
        |mc| mc.matchAddCursor()?.call(-1),
    );
    set(
        conf,
        ["n", "x"],
        "<M-N>",
        "Skip cursor on next occurrence",
        |mc| mc.matchSkipCursor()?.call(1),
    );
    set(
        conf,
        ["n", "x"],
        "<M-P>",
        "Skip cursor on previous occurrence",
        |mc| mc.matchSkipCursor()?.call(-1),
    );
}
