use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_conform(&self) {
        let env = self.env();

        do_try(|| {
            env.globals
                .vim()?
                .pack()?
                .add()?
                .call(["https://github.com/stevearc/conform.nvim"])
        })
        .ok_or_notify(env);

        env.req_cache
            .conform
            .register(|_, conform| {
                conform.setup()?.call(tbl!({
                    format_on_save = tbl!({
                        timeout_ms = 500;
                        lsp_format = "fallback";
                    });
                }))
            })
            .ok_or_notify(env);
    }
}
