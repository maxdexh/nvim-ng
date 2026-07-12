use crate::{env::gvim::lsp::VimLspConfig, prelude::*};

crate::utils::from_tbl_proxy!({
    struct TypstPreview {
        setup: LuaCallable<LuaDict<LuaVal>, ()>,
    }
});
impl NvimConf<'_> {
    pub fn load_typst_lang(&self) {
        self.add_packs(["https://github.com/chomosuke/typst-preview.nvim"]);

        self.set_formatter("typst", ["typstyle"]);
        self.formatter_use_nix("typstyle", "typstyle", "typstyle");

        self.config_lsp(
            "tinymist",
            mk_builder!(VimLspConfig, {
                cmd = self.nix_shell_cmd("tinymist", ["tinymist"]);
            }),
        );

        // TODO: on filetype
        self.on_very_lazy(|conf| {
            conf.setup_plugin::<TypstPreview>("typst-preview", |tps| {
                tps.setup()?.call(tbl!(owned, {}))
            })?;

            Ok(())
        })
        .ok_or_notify(self);
    }
}
