use crate::{
    env::gvim::api::{AutoCmdOpts, HighlightOpts},
    prelude::*,
};

impl NvimConf<'_> {
    pub fn load_colorscheme(&self) {
        self.add_packs(["https://github.com/Mofiqul/vscode.nvim"]);

        self.add_autocmd(
            "ColorScheme",
            mk_builder!(AutoCmdOpts, {
                callback = self.create_cb(|conf, ()| {
                    conf.set_overrides();
                    Ok(())
                });
            }),
        );

        self.setup_plugin_now(
            "vscode",
            tbl!(owned, {
                codedark_modern = true;
                transparent = true;
                terminal_colors = true;
            }),
        )
        .ok_or_notify(self);

        self.run_cmd("colorscheme vscode");
    }

    fn set_overrides(&self) {
        self.set_hl(
            "DiagnosticUnderlineWarn",
            mk_builder!(HighlightOpts, {
                underline = true;
                sp = "NvimLightYellow";
            }),
        );
        self.set_hl(
            "DiagnosticUnderlineError",
            mk_builder!(HighlightOpts, {
                underline = true;
                sp = "NvimLightRed";
            }),
        );

        self.set_fgs([
            ("RainbowDelimiterYellow", "#FFD700"),
            ("RainbowDelimiterRed", "#DA70D6"),
            ("RainbowDelimiterBlue", "#179FFF"),
            ("@lsp.typemod.function.declaration", "#DCDCAA"),
            ("@type", "#39C8B0"),
            ("@interface", "#B8D7A3"),
            ("@lsp.type.enumMember", "#D3D3D3"),
            ("Macro", "#4EADE5"),
            ("@lsp.type.function", "#FFC66D"),
            ("@lsp.type.typeParameter", "#20999D"),
            ("parameter", "#9CDCFE"),
            ("keyword", "#499CD5"),
            ("@module", "#DCDCAA"),
            ("DiffAdd", "#00FF00"),
            ("DiffDelete", "#FF0000"),
            ("DiffChange", "#AAAA00"),
        ]);

        self.set_links([
            ("@lsp.type.typeAlias", "@type"),
            ("@lsp.type.union", "@type"),
            ("@lsp.type.enum", "@type"),
            ("@lsp.type.struct", "@type"),
            ("@lsp.type.class", "@type"),
            ("@lsp.type.builtinType", "@type"),
            ("variable", "@variable"),
            ("@lsp.type.const", "@variable"),
            ("@lsp.type.builtinAttribute.rust", "@attribute"),
            ("@lsp.type.interface", "@interface"),
            ("@namespace", "@module"),
            ("@lsp.type.macro", "Macro"),
            ("@function.macro.rust", "Macro"),
            ("@lsp.type.method", "@lsp.type.function"),
            (
                "@lsp.typemod.method.declaration",
                "@lsp.typemod.function.declaration",
            ),
            ("@lsp.type.parameter", "parameter"),
            ("@lsp.type.selfKeyword.rust", "keyword"),
            ("rustMacroVariable", "parameter"),
            ("@keyword", "keyword"),
            ("@constant", "@variable"),
            ("@lsp.type.lifetime", "@lsp.type.typeParameter"),
            ("@keyword.import.rust", "keyword"),
            ("@variable.builtin.rust", "keyword"),
            ("rustModPath", "@module"),
            ("rustAttribute", "operator"),
            // typically constant.builtin would be something like `null`, but in
            // rust it refers only to enum members from the prelude (e.g. `None`, `Some`)
            ("@constant.builtin.rust", "@lsp.type.enumMember.rust"),
        ]);

        self.rem_hls([
            "rustAssert",                   // works only sometimes in macros
            "DiagnosticUnnecessary",        // Intrusive
            "@punctuation.bracket", // Breaks rainbow-brackets because treesitter takes precedence
            "rustFoldBraces",       // See above
            "@lsp.type.operator.lua", // For some reason this is applied to brackets and braces, breaking rainbow-brackets
            "@markup.link.markdown_inline", // disable underlining of links (for comment highlighting)
            "@lsp.type.comment.rust",       // would override injections due to high priority
        ]);

        self.run_cmd("hi StatusLine guibg=NONE ctermbg=NONE");
        self.run_cmd("hi MoreMsg guibg=NONE");
        self.run_cmd("hi ModeMsg guibg=NONE");
        self.run_cmd("hi TabLineFill guibg=NONE");
        self.run_cmd("hi NormalFloat guibg=NONE");
    }

    fn set_links<'a>(&self, links: impl IntoIterator<Item = (&'a str, &'a str)>) {
        for (name, link) in links {
            self.set_hl(
                name,
                mk_builder!(HighlightOpts, {
                    link = link;
                }),
            );
        }
    }
    fn set_fgs<'a>(&self, fgs: impl IntoIterator<Item = (&'a str, &'a str)>) {
        for (name, fg) in fgs {
            self.set_hl(
                name,
                mk_builder!(HighlightOpts, {
                    fg = fg;
                }),
            );
        }
    }
    fn rem_hls<'a>(&self, names: impl IntoIterator<Item = &'a str>) {
        for name in names {
            self.set_hl(name, mk_builder!(HighlightOpts, {}));
        }
    }

    fn set_hl(&self, name: &str, val: impl LuaSub<LuaStruct<HighlightOpts>>) {
        do_try(|| {
            self.env()
                .globals
                .vim()?
                .api()?
                .nvim_set_hl()?
                .call((0, name, val))
        })
        .ok_or_notify(self);
    }
}
