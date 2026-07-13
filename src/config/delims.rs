use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_delims(&self) {
        self.add_packs(["https://github.com/windwp/nvim-autopairs"]);
        self.on_very_lazy(|conf| conf.setup_plugin_now("nvim-autopairs", tbl!(owned, {})))
            .ok_or_notify(self);

        self.add_packs(["https://github.com/HiPhish/rainbow-delimiters.nvim"]);
        self.setup_plugin_now(
            "rainbow-delimiters.setup",
            tbl!(owned, {
                highlight = [
                    "RainbowDelimiterYellow",
                    "RainbowDelimiterRed",
                    "RainbowDelimiterBlue",
                ];
                query = tbl!(owned, {
                    "" = "rainbow-delimiters";
                    lua = "rainbow-blocks";
                });
            }),
        )
        .ok_or_notify(self);
    }
}
