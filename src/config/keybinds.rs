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
                    Some(tbl!({
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
                    Some(tbl!({
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
                    Some(tbl!({
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
                    Some(tbl!({
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
                    Some(tbl!({
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
                    Some(tbl!({
                        cwd = get_root(env)?;
                    })),
                )
            }),
            mk_builder!(KeymapOpts, {
                desc = "Grep Selection (Root Dir)";
            }),
        );

        self.set_keymap(
            "n",
            "<leader>ca",
            self.create_cb(|env, ()| call_picker(env, "code_action", LuaNil)),
            mk_builder!(KeymapOpts, {
                desc = "Code Action";
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
    }
}
