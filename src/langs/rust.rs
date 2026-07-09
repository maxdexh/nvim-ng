use crate::{env::vim::pack::PackOpts, prelude::*};

impl NvimConf<'_> {
    pub fn load_rust_lang(&self) {
        let env = self.env();
        do_try(|| {
            let vim = env.globals.vim()?;
            vim.pack()?.add()?.call([
                //
                PackOpts::empty()
                    .with_src("https://github.com/mrcjkb/rustaceanvim")
                    .with_version(vim.version()?.range()?.call("^9")?)
                    .finish(),
            ])
        })
        .ok_or_notify(env);

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
