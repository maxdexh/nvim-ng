use crate::{plugins::GenericPlugin, prelude::*};

impl NvimConf<'_> {
    pub fn load_colorscheme(&self) {
        let env = self.env();

        self.add_packs([
            "https://github.com/nvim-treesitter/nvim-treesitter",
            "https://github.com/catppuccin/nvim",
        ]);

        self.req_treesitter()
            .and_then(|it| it.setup()?.call(tbl!({})))
            .ok_or_notify(env);

        self.call_require::<GenericPlugin>("catppuccin")
            .and_then(|it| {
                it.setup()?.call(tbl!({
                    transparent_background = true;
                }))
            })
            .ok_or_notify(env);

        self.run_cmd("colorscheme catppuccin");
    }
}
