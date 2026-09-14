use crate::{
    env::gvim::{api::AutoCmdOpts, lsp::VimLspConfig},
    prelude::*,
};

crate::utils::from_tbl_proxy!({
    struct TypstPreview {
        setup: LuaCallable<LuaDict<LuaVal>, ()>,
    }
});
impl NvimConf<'_> {
    fn setup_typst_preview(&self) -> Result<()> {
        self.setup_plugin::<TypstPreview>("typst-preview", |tps| {
            tps.setup()?.call(tbl!(owned, {}))
        })?;

        Ok(())
    }
    pub fn load_typst_lang(&self) {
        self.set_formatter("typst", ["typstyle"]);
        self.formatter_use_nix("typstyle", "typstyle", "typstyle");

        if self.is_vscode() {
            return;
        }

        self.add_packs(["https://github.com/chomosuke/typst-preview.nvim"]);

        self.config_lsp(
            "tinymist",
            mk_builder!(VimLspConfig, {
                cmd = self.nix_shell_cmd("tinymist", ["tinymist"]);
            }),
        );

        let lazy_load = self.mk_sched_callback(|conf, ()| conf.setup_typst_preview());

        self.add_autocmd(
            "FileType",
            mk_builder!(AutoCmdOpts, {
                pattern = "typst";
                callback = lazy_load;
                once = true;
            }),
        );
    }
}
