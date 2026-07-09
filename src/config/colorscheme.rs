use crate::{plugins::GenericPlugin, prelude::*};

impl NvimConf<'_> {
    pub fn load_colorscheme(&self) {
        let env = self.env();

        do_try(|| {
            env.globals.vim()?.pack()?.add()?.call([
                "https://github.com/nvim-treesitter/nvim-treesitter",
                "https://github.com/catppuccin/nvim",
            ])
        })
        .ok_or_notify(env);

        env.req_treesitter()
            .and_then(|it| it.setup()?.call(tbl!({})))
            .ok_or_notify(env);

        env.call_require::<GenericPlugin>("catppuccin")
            .and_then(|it| {
                it.setup()?.call(tbl!({
                    transparent_background = true;
                }))
            })
            .ok_or_notify(env);

        env.vim().run_cmd("colorscheme catppuccin");
    }
}
