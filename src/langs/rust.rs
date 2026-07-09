use crate::{env::vim::pack::PackOpts, prelude::*};

impl NvimConf<'_> {
    pub fn load_rust_lang(&self) {
        let env = self.env();
        if let Some(version) = self.version_range("^9").ok_or_notify(self) {
            self.add_packs([mk_builder!(PackOpts, {
                src = "https://github.com/mrcjkb/rustaceanvim";
                version = version;
            })]);
        }

        let ra_opts = tbl!({
            assist = tbl!({
                preferSelf = true;
            });
        });

        do_try(|| {
            env.globals.vim()?.g()?.set(
                "rustaceanvim",
                tbl!({
                    server.default_settings = tbl!({
                        "rust-analyzer" = ra_opts;
                    });
                }),
            )
        })
        .ok_or_notify(env);

        self.ft_set_indent("rust", 4);
        self.ts_install_lang("rust");
        self.set_formatter("rust", ["rustfmt"]);
    }
}
