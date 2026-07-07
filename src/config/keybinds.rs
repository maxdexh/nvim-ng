use crate::prelude::*;

fn call_picker(env: &Nvim, name: &str, args: impl LuaSub<Option<LuaTableAny>>) -> Result<()> {
    env.req_snacks()?.picker()?.get(name)?.call(args)
}
fn get_cwd(env: &Nvim) -> Result<LuaString> {
    env.globals.vim()?.uv()?.cwd()?.call(())
}
fn get_root(env: &Nvim) -> Result<LuaString> {
    env.req_snacks()
        .and_then(|snacks| snacks.git()?.get_root()?.call(()))?
        .map_or_else(|| get_cwd(env), Ok)
}
impl NvimConf<'_> {
    pub fn load_keybinds(&self) {
        let keymap = self.keymap();

        keymap.set(["n"], "<leader>sx", "Resume Picker", |env| {
            call_picker(env, "resume", LuaNil)
        });

        keymap.set(["n"], "<leader>fP", "Find Picker", |env| {
            call_picker(env, "pickers", LuaNil)
        });

        keymap.set(["n"], "<leader>ff", "Find Files (cwd)", |env| {
            call_picker(
                env,
                "files",
                Some(tbl!({
                    cwd = get_cwd(env)?;
                })),
            )
        });

        keymap.set(["n"], "<leader>fF", "Find Files (Root Dir)", |env| {
            call_picker(
                env,
                "files",
                Some(tbl!({
                    cwd = get_root(env)?;
                })),
            )
        });

        keymap.set(["n"], "<leader>sg", "Grep (cwd)", |env| {
            call_picker(
                env,
                "grep",
                Some(tbl!({
                    cwd = get_cwd(env)?;
                })),
            )
        });

        keymap.set(["n"], "<leader>sG", "Grep (Root Dir)", |env| {
            call_picker(
                env,
                "grep",
                Some(tbl!({
                    cwd = get_root(env)?;
                })),
            )
        });

        keymap.set(["x"], "<leader>sg", "Grep Selection (cwd)", |env| {
            call_picker(
                env,
                "grep_word",
                Some(tbl!({
                    cwd = get_cwd(env)?;
                })),
            )
        });

        keymap.set(["x"], "<leader>sG", "Grep Selection (Root Dir)", |env| {
            call_picker(
                env,
                "grep_word",
                Some(tbl!({
                    cwd = get_root(env)?;
                })),
            )
        });

        keymap.set(["n"], "<leader>ca", "Code Action", |env| {
            call_picker(env, "code_action", LuaNil)
        });

        keymap.set(["n"], "gd", "Goto Definition", |env| {
            call_picker(env, "lsp_definitions", LuaNil)
        });

        keymap.set(["n"], "gi", "Goto Implementations", |env| {
            call_picker(env, "lsp_implementations", LuaNil)
        });

        keymap.set(["n"], "gr", "Goto References", |env| {
            call_picker(env, "lsp_references", LuaNil)
        });

        keymap.set_base(
            ["v"],
            "<C-c>",
            "\"+y",
            tbl!({
                desc = "Copy Selection";
            }),
        );

        keymap.set_base(
            ["t"],
            "<ESC><ESC>",
            "<C-\\><C-n>",
            tbl!({
                desc = "Exit Terminal mode";
            }),
        );
    }
}
