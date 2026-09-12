use crate::{env::gvim::keymap::KeymapOpts, prelude::*};

impl NvimConf<'_> {
    pub fn load_bufferline(&self) {
        if !self.is_vscode() {
            self.add_packs(["https://github.com/akinsho/bufferline.nvim"]);
            self.on_very_lazy(|conf| conf.setup_plugin_now("bufferline", conf.bufferline_opts()))
                .ok_or_notify(self);
        }

        self.set_keymap(
            "n",
            "L",
            if self.is_vscode() {
                "<CMD>call VSCodeNotify('workbench.action.nextEditor')<CR>"
            } else {
                "<CMD>BufferLineCycleNext<CR>"
            },
            mk_builder!(KeymapOpts, {
                desc = "Next Buffer";
            }),
        );

        self.set_keymap(
            "n",
            "H",
            if self.is_vscode() {
                "<CMD>call VSCodeNotify('workbench.action.previousEditor')<CR>"
            } else {
                "<CMD>BufferLineCyclePrev<CR>"
            },
            mk_builder!(KeymapOpts, {
                desc = "Prev Buffer";
            }),
        );

        self.set_keymap(
            "n",
            "<leader>bd",
            if self.is_vscode() {
                "<CMD>call VSCodeNotify('workbench.action.closeActiveEditor')<CR>"
            } else {
                "<CMD>bd<CR>"
            },
            mk_builder!(KeymapOpts, {
                desc = "Close Buffer";
            }),
        );
        self.set_keymap(
            "n",
            "<leader>bl",
            if self.is_vscode() {
                "<CMD>call VSCodeNotify('workbench.action.closeEditorsToTheLeft')<CR>"
            } else {
                "<CMD>BufferLineCloseLeft<CR>"
            },
            mk_builder!(KeymapOpts, {
                desc = "Close Buffers Left";
            }),
        );
        self.set_keymap(
            "n",
            "<leader>br",
            if self.is_vscode() {
                "<CMD>call VSCodeNotify('workbench.action.closeEditorsToTheRight')<CR>"
            } else {
                "<CMD>BufferLineCloseRight<CR>"
            },
            mk_builder!(KeymapOpts, {
                desc = "Close Buffers Right";
            }),
        );
    }

    fn bufferline_opts(&self) -> impl LuaSub<LuaDict<LuaVal>> {
        // TODO: Set diagnostics_indicator
        tbl!(owned, {
            options = tbl!(owned, {
                diagnostics = "nvim_lsp";
            });
        })
    }
}
