use crate::{env::vim::keymap::KeymapOpts, prelude::*};

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
            KeymapOpts::empty().with_desc("Resume Picker").finish(),
        );

        self.set_keymap(
            "n",
            "<leader>fP",
            self.create_cb(|env, ()| call_picker(env, "pickers", LuaNil)),
            KeymapOpts::empty().with_desc("Find Picker").finish(),
        );

        self.set_keymap(
            "n",
            "<leader>ff",
            self.create_cb(|conf, ()| {
                call_picker(
                    conf,
                    "files",
                    Some(tbl!({
                        cwd = get_cwd(conf)?;
                    })),
                )
            }),
            KeymapOpts::empty().with_desc("Find Files (cwd)").finish(),
        );

        self.set_keymap(
            "n",
            "<leader>fF",
            self.create_cb(|env, ()| {
                call_picker(
                    env,
                    "files",
                    Some(tbl!({
                        cwd = get_root(env)?;
                    })),
                )
            }),
            KeymapOpts::empty()
                .with_desc("Find Files (Root Dir)")
                .finish(),
        );

        self.set_keymap(
            "n",
            "<leader>sg",
            self.create_cb(|env, ()| {
                call_picker(
                    env,
                    "grep",
                    Some(tbl!({
                        cwd = get_cwd(env)?;
                    })),
                )
            }),
            KeymapOpts::empty().with_desc("Grep (cwd)").finish(),
        );

        self.set_keymap(
            "n",
            "<leader>sG",
            self.create_cb(|env, ()| {
                call_picker(
                    env,
                    "grep",
                    Some(tbl!({
                        cwd = get_root(env)?;
                    })),
                )
            }),
            KeymapOpts::empty().with_desc("Grep (Root Dir)").finish(),
        );

        self.set_keymap(
            "x",
            "<leader>sg",
            self.create_cb(|env, ()| {
                call_picker(
                    env,
                    "grep_word",
                    Some(tbl!({
                        cwd = get_cwd(env)?;
                    })),
                )
            }),
            KeymapOpts::empty()
                .with_desc("Grep Selection (cwd)")
                .finish(),
        );

        self.set_keymap(
            "x",
            "<leader>sG",
            self.create_cb(|env, ()| {
                call_picker(
                    env,
                    "grep_word",
                    Some(tbl!({
                        cwd = get_root(env)?;
                    })),
                )
            }),
            KeymapOpts::empty()
                .with_desc("Grep Selection (Root Dir)")
                .finish(),
        );

        self.set_keymap(
            "n",
            "<leader>ca",
            self.create_cb(|env, ()| call_picker(env, "code_action", LuaNil)),
            KeymapOpts::empty().with_desc("Code Action").finish(),
        );

        self.set_keymap(
            "n",
            "gd",
            self.create_cb(|env, ()| call_picker(env, "lsp_definitions", LuaNil)),
            KeymapOpts::empty().with_desc("Goto Definition").finish(),
        );

        self.set_keymap(
            "n",
            "gi",
            self.create_cb(|env, ()| call_picker(env, "lsp_implementations", LuaNil)),
            KeymapOpts::empty()
                .with_desc("Goto Implementations")
                .finish(),
        );

        self.set_keymap(
            "n",
            "gr",
            self.create_cb(|env, ()| call_picker(env, "lsp_references", LuaNil)),
            KeymapOpts::empty().with_desc("Goto References").finish(),
        );

        self.set_keymap(
            "v",
            "<C-c>",
            "\"+y",
            KeymapOpts::empty().with_desc("Copy Selection").finish(),
        );

        self.set_keymap(
            "t",
            "<ESC><ESC>",
            "<C-\\><C-n>",
            KeymapOpts::empty().with_desc("Exit Terminal mode").finish(),
        );
    }
}
