use crate::{env::gvim::pack::PackOpts, prelude::*};

impl NvimConf<'_> {
    pub fn load_rust_lang(&self) {
        if let Some(version) = self.version_range("^9").ok_or_notify(self) {
            self.add_packs([mk_builder!(PackOpts, {
                src = "https://github.com/mrcjkb/rustaceanvim";
                version = version;
            })]);
        }

        self.with_vim_g(|g| {
            tbl!(out(g), {
                rustaceanvim = self.rustaceanvim_opts();
            })
        })
        .ok_or_notify(self);

        self.ft_set_indent("rust", 4);
        self.set_formatter("rust", ["rustfmt"]);
    }

    fn rustaceanvim_opts(&self) -> impl LuaSub<LuaVal> {
        let ra_opts = tbl!(owned, {
            assist = tbl!(owned, {
                preferSelf = true;
            });
        });

        tbl!(owned, {
            server.default_settings = tbl!(owned, {
                "rust-analyzer" = ra_opts;
            });
        })
    }
}
