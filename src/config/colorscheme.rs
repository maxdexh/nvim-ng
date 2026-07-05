use crate::{plugins::GenericPlugin, prelude::*};

impl NvimConf<'_> {
    pub fn load_colorscheme(&self) {
        let env = self.env();
        env.vim().pack().add(
            "https://github.com/nvim-treesitter/nvim-treesitter",
            PackOpts::empty().with_version("main"),
        );

        env.req_treesitter()
            .and_then(|it| it.setup()?.call(tbl!({})))
            .ok_or_notify(env);

        env.vim()
            .pack()
            .add("https://github.com/catppuccin/nvim", PackOpts::empty());

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
