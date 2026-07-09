use crate::{plugins::GenericPlugin, prelude::*};

impl NvimConf<'_> {
    pub fn load_colorscheme(&self) {
        let env = self.env();
        env.vim()
            .pack()
            .add_one("https://github.com/nvim-treesitter/nvim-treesitter");

        env.req_treesitter()
            .and_then(|it| it.setup()?.call(tbl!({})))
            .ok_or_notify(env);

        env.vim()
            .pack()
            .add_one("https://github.com/catppuccin/nvim");

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
