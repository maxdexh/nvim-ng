use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_delims(&self) {
        self.add_packs(["https://github.com/windwp/nvim-autopairs"]);
        self.on_very_lazy(|conf| conf.setup_plugin_now("nvim-autopairs", tbl!(owned, {})))
            .ok_or_notify(self);
    }
}
