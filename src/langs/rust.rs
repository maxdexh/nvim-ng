use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_rust_lang(&self) {
        let env = self.env();
        let vim = env.vim();
        vim.pack().add(
            "https://github.com/mrcjkb/rustaceanvim",
            PackOpts::empty().with_version(vim.version().range("^9")),
        );

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
