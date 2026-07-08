use crate::{plugins::GenericPlugin, prelude::*};

impl NvimConf<'_> {
    pub fn load_completions(&self) {
        let env = self.env();
        env.vim().pack().add(
            "https://github.com/Saghen/blink.cmp",
            PackOpts::empty().with_version(env.vim().version().range("1.*")),
        );

        env.vim()
            .pack()
            .add("https://github.com/L3MON4D3/LuaSnip", PackOpts::empty());

        env.call_require::<GenericPlugin>("blink.cmp")
            .and_then(|it| it.setup()?.call(self.cmp_opts()))
            .ok_or_notify(env);
    }

    fn cmp_opts(&self) -> impl LuaSub<LuaTableAny> {
        tbl!({
            appearance = tbl!({
                kind_icons = tbl!({
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
            cmdline = tbl!({
                completion = tbl!({
                    ghost_text.enabled = true;
                    list.selection.preselect = false;
                });
                enabled = true;
                keymap = tbl!({
                    "<Left>" = false;
                    "<Right>" = false;
                    preset = "cmdline";
                });
            });
            completion = tbl!({
                accept.auto_brackets.enabled = true;
                documentation = tbl!({
                    auto_show = true;
                    auto_show_delay_ms = 200;
                    window.border = "single";
                });
                ghost_text.enabled = true;

                list.selection.auto_insert = false;
                menu = tbl!({
                    border = "single";
                    draw.treesitter = ["lsp"];
                });
            });
            fuzzy = tbl!({
                implementation = "prefer_rust_with_warning";
                sorts = ["exact", "score", "sort_text"];
            });
            keymap = tbl!({
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
