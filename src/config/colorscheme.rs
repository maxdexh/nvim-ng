use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_colorscheme(&self) {
        let env = self.env();

        self.add_packs([
            "https://github.com/nvim-treesitter/nvim-treesitter",
            "https://github.com/catppuccin/nvim",
        ]);

        self.req_treesitter()
            .and_then(|it| it.setup()?.call(tbl!(owned, {})))
            .ok_or_notify(env);

        self.setup_plugin(
            "catppuccin",
            tbl!(owned, {
                transparent_background = true;
            }),
        )
        .ok_or_notify(env);

        self.run_cmd("colorscheme catppuccin");
    }
}
