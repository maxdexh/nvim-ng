use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_colorscheme(&self) {
        let env = self.env();

        self.add_packs(["https://github.com/catppuccin/nvim"]);

        self.setup_plugin_now(
            "catppuccin",
            tbl!(owned, {
                transparent_background = true;
            }),
        )
        .ok_or_notify(env);

        self.run_cmd("colorscheme catppuccin");
    }
}
