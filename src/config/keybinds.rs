use crate::{env::gvim::keymap::KeymapOpts, prelude::*};

fn call_picker(
    conf: NvimConf,
    name: &str,
    args: impl LuaSub<Option<LuaDict<LuaVal>>>,
) -> Result<()> {
    conf.req_snacks()?.picker()?.get(name)?.call(args)
}
fn get_cwd(conf: NvimConf) -> Result<LuaString> {
    conf.env().globals.vim()?.uv()?.cwd()?.call(())
}
fn get_root(conf: NvimConf) -> Result<LuaString> {
    conf.req_snacks()
        .and_then(|snacks| snacks.git()?.get_root()?.call(()))?
        .map_or_else(|| get_cwd(conf), Ok)
}
impl NvimConf<'_> {
    pub fn load_keybinds(&self) {
        self.set_keymap(
            "n",
            "<leader>sx",
            self.create_cb(|conf, ()| call_picker(conf, "resume", LuaNil)),
            mk_builder!(KeymapOpts, {
                desc = "Resume Picker";
            }),
        );

        self.set_keymap(
            "n",
            "<leader>fP",
            self.create_cb(|env, ()| call_picker(env, "pickers", LuaNil)),
            mk_builder!(KeymapOpts, {
                desc = "Find Picker";
            }),
        );

        self.set_keymap(
            "n",
            "<leader>ff",
            self.create_cb(|conf, ()| {
                call_picker(
                    conf,
                    "files",
                    Some(tbl!(owned, {
                        cwd = get_cwd(conf)?;
                    })),
                )
            }),
            mk_builder!(KeymapOpts, {
                desc = "Find Files (cwd)";
            }),
        );

        self.set_keymap(
            "n",
            "<leader>fF",
            self.create_cb(|env, ()| {
                call_picker(
                    env,
                    "files",
                    Some(tbl!(owned, {
                        cwd = get_root(env)?;
                    })),
                )
            }),
            mk_builder!(KeymapOpts, {
                desc = "Find Files (Root Dir)";
            }),
        );

        self.set_keymap(
            "n",
            "<leader>sg",
            self.create_cb(|env, ()| {
                call_picker(
                    env,
                    "grep",
                    Some(tbl!(owned, {
                        cwd = get_cwd(env)?;
                    })),
                )
            }),
            mk_builder!(KeymapOpts, {
                desc = "Grep (cwd)";
            }),
        );

        self.set_keymap(
            "n",
            "<leader>sG",
            self.create_cb(|env, ()| {
                call_picker(
                    env,
                    "grep",
                    Some(tbl!(owned, {
                        cwd = get_root(env)?;
                    })),
                )
            }),
            mk_builder!(KeymapOpts, {
                desc = "Grep (Root Dir)";
            }),
        );

        self.set_keymap(
            "x",
            "<leader>sg",
            self.create_cb(|env, ()| {
                call_picker(
                    env,
                    "grep_word",
                    Some(tbl!(owned, {
                        cwd = get_cwd(env)?;
                    })),
                )
            }),
            mk_builder!(KeymapOpts, {
                desc = "Grep Selection (cwd)";
            }),
        );

        self.set_keymap(
            "x",
            "<leader>sG",
            self.create_cb(|env, ()| {
                call_picker(
                    env,
                    "grep_word",
                    Some(tbl!(owned, {
                        cwd = get_root(env)?;
                    })),
                )
            }),
            mk_builder!(KeymapOpts, {
                desc = "Grep Selection (Root Dir)";
            }),
        );

        if let Some(lsp_buf) = do_try(|| self.env().globals.vim()?.lsp()?.buf()).ok_or_notify(self)
        {
            self.set_keymap(
                "n",
                "<leader>ca",
                LuaDeferErr(lsp_buf.code_action()),
                mk_builder!(KeymapOpts, {
                    desc = "Code Action";
                }),
            );
            self.set_keymap(
                "n",
                "<leader>cr",
                LuaDeferErr(lsp_buf.rename()),
                mk_builder!(KeymapOpts, {
                    desc = "Rename Symbol";
                }),
            );
            self.set_keymap(
                "n",
                "K",
                LuaDeferErr(lsp_buf.hover()),
                mk_builder!(KeymapOpts, {
                    desc = "Open Symbol Hover";
                }),
            );
            self.set_keymap(
                "i",
                "<C-h>",
                LuaDeferErr(lsp_buf.signature_help()),
                mk_builder!(KeymapOpts, {
                    desc = "Signature Help";
                }),
            );
        }
        self.set_keymap(
            "n",
            "<leader>xc",
            LuaDeferErr(do_try(|| {
                self.env().globals.vim()?.diagnostic()?.open_float()
            })),
            mk_builder!(KeymapOpts, {
                desc = "Show Diagnostic";
            }),
        );

        self.set_keymap(
            "n",
            "<leader>xx",
            self.create_cb(|env, ()| call_picker(env, "diagnostics", LuaNil)),
            mk_builder!(KeymapOpts, {
                desc = "Diagnostics";
            }),
        );
        self.set_keymap(
            "n",
            "<leader>xX",
            self.create_cb(|env, ()| call_picker(env, "diagnostics_buffer", LuaNil)),
            mk_builder!(KeymapOpts, {
                desc = "Diagnostics (Buffer)";
            }),
        );

        self.set_keymap(
            "n",
            "gd",
            self.create_cb(|env, ()| call_picker(env, "lsp_definitions", LuaNil)),
            mk_builder!(KeymapOpts, {
                desc = "Goto Definition";
            }),
        );

        self.set_keymap(
            "n",
            "gi",
            self.create_cb(|env, ()| call_picker(env, "lsp_implementations", LuaNil)),
            mk_builder!(KeymapOpts, {
                desc = "Goto Implementations";
            }),
        );

        self.set_keymap(
            "n",
            "gr",
            self.create_cb(|env, ()| call_picker(env, "lsp_references", LuaNil)),
            mk_builder!(KeymapOpts, {
                desc = "Goto References";
            }),
        );

        self.set_keymap(
            "v",
            "<C-c>",
            "\"+y",
            mk_builder!(KeymapOpts, {
                desc = "Copy Selection";
            }),
        );

        self.set_keymap(
            "t",
            "<ESC><ESC>",
            "<C-\\><C-n>",
            mk_builder!(KeymapOpts, {
                desc = "Exit Terminal mode";
            }),
        );

        self.set_keymap(
            "n",
            "<leader>bd",
            "<CMD>bd<CR>",
            mk_builder!(KeymapOpts, {
                desc = "Delete buffer and window";
            }),
        );

        self.set_keymap(
            "n",
            "<C-Down>",
            "<CMD>resize -2<CR>",
            mk_builder!(KeymapOpts, {
                desc = "Decrease window size";
            }),
        );
        self.set_keymap(
            "n",
            "<C-Up>",
            "<CMD>resize +2<CR>",
            mk_builder!(KeymapOpts, {
                desc = "Increase window size";
            }),
        );

        macro_rules! goto_window {
            ($k:expr, $desc:expr) => {
                self.set_keymap(
                    "n",
                    concat!("<C-", $k, ">"),
                    concat!("<C-w>", $k),
                    mk_builder!(KeymapOpts, {
                        desc = concat!("Go to ", $desc, " window");
                    }),
                )
            };
        }
        goto_window!("j", "down");
        goto_window!("k", "up");
        goto_window!("h", "left");
        goto_window!("l", "right");
    }
}
