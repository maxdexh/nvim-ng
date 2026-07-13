use crate::{env::gvim::pack::PackOpts, prelude::*};

impl NvimConf<'_> {
    pub fn load_completions(&self) {
        if let Some(version) = self.version_range("1.*").ok_or_notify(self) {
            self.add_packs([mk_builder!(PackOpts, {
                src = "https://github.com/Saghen/blink.cmp";
                version = version;
            })]);
        }

        // FIXME: Lazy
        self.setup_plugin_now("blink.cmp", self.cmp_opts())
            .ok_or_notify(self);

        self.add_packs([
            "https://github.com/L3MON4D3/LuaSnip",
            "https://github.com/rafamadriz/friendly-snippets",
        ]);
        self.on_very_lazy(|conf| {
            conf.setup_snippets();
            Ok(())
        })
        .ok_or_notify(self);
    }

    fn setup_snippets(&self) {
        do_try(|| {
            self.env()
                .require::<LuaTableAny>("luasnip.loaders.from_vscode")?
                .get_any::<crate::lua::LuaCallableAny>("lazy_load")?
                .call_any::<()>(())
        })
        .ok_or_notify(self);

        self.setup_plugin_now("luasnip", tbl!(owned, {}))
            .ok_or_notify(self);
    }

    fn cmp_opts(&self) -> impl LuaSub<LuaDict<LuaVal>> {
        tbl!(owned, {
            appearance = tbl!(owned, {
                kind_icons = tbl!(owned, {
                    Array = " ";
                    Boolean = "󰨙 ";
                    Class = " ";
                    Codeium = "󰘦 ";
                    Collapsed = " ";
                    Color = " ";
                    Constant = "󰏿 ";
                    Constructor = " ";
                    Control = " ";
                    Copilot = " ";
                    Enum = " ";
                    EnumMember = " ";
                    Event = " ";
                    Field = " ";
                    File = " ";
                    Folder = " ";
                    Function = "󰊕 ";
                    Interface = " ";
                    Key = " ";
                    Keyword = " ";
                    Method = "󰊕 ";
                    Module = " ";
                    Namespace = "󰦮 ";
                    Null = " ";
                    Number = "󰎠 ";
                    Object = " ";
                    Operator = " ";
                    Package = " ";
                    Property = " ";
                    Reference = " ";
                    Snippet = "󱄽 ";
                    String = " ";
                    Struct = "󰆼 ";
                    Supermaven = " ";
                    TabNine = "󰏚 ";
                    Text = " ";
                    TypeParameter = " ";
                    Unit = " ";
                    Value = " ";
                    Variable = "󰀫 ";
                });
                nerd_font_variant = "mono";
                use_nvim_cmp_as_default = false;
            });
            cmdline = tbl!(owned, {
                completion = tbl!(owned, {
                    ghost_text.enabled = true;
                    list.selection.preselect = false;
                });
                enabled = true;
                keymap = tbl!(owned, {
                    "<Left>" = false;
                    "<Right>" = false;
                    preset = "cmdline";
                });
            });
            completion = tbl!(owned, {
                accept.auto_brackets.enabled = true;
                documentation = tbl!(owned, {
                    auto_show = true;
                    auto_show_delay_ms = 200;
                    window.border = "single";
                });
                ghost_text.enabled = true;

                list.selection.auto_insert = false;
                menu = tbl!(owned, {
                    border = "single";
                    draw.treesitter = ["lsp"];
                });
            });
            fuzzy = tbl!(owned, {
                implementation = "prefer_rust_with_warning";
                sorts = ["exact", "score", "sort_text"];
            });
            keymap = tbl!(owned, {
                "<C-e>" = ["hide", "fallback"];
                "<C-n>" = ["select_next", "fallback"];
                "<C-p>" = ["select_prev", "fallback"];
                "<C-space>" = ["show", "show_documentation", "hide_documentation"];
                "<C-y>" = ["select_and_accept"];
                "<Down>" = ["select_next", "fallback"];
                "<Up>" = ["select_prev", "fallback"];
                preset = "default";
            });

            signature.enabled = true;

            snippets.preset = "luasnip";

            sources.default = ["lsp", "path", "snippets", "buffer"];
        })
    }
}
