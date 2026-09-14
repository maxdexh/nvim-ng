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

        if !self.is_vscode() {
            self.set_keymap(
                "n",
                "<leader>bb",
                "<CMD>e #<CR>",
                mk_builder!(KeymapOpts, {
                    desc = "Previous buffer (cd style)";
                }),
            );
        }
        self.set_keymap(
            "n",
            "<leader>bd",
            if self.is_vscode() {
                LuaUnion::Left("<CMD>call VSCodeNotify('workbench.action.closeActiveEditor')<CR>")
            } else {
                LuaUnion::Right(self.mk_callback(|conf, ()| {
                    conf.req_snacks()?.bufdelete()?.call(()) //
                }))
            },
            mk_builder!(KeymapOpts, {
                desc = "Close Buffer (keep window layout)";
            }),
        );
        if !self.is_vscode() {
            self.set_keymap(
                "n",
                "<leader>bD",
                "<CMD>bd<CR>",
                mk_builder!(KeymapOpts, {
                    desc = "Close Buffer";
                }),
            );
        }
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
