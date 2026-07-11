use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct Noice {
        setup: LuaCallable<LuaDict<LuaVal>, ()>,
    }
});
impl NvimConf<'_> {
    fn req_noice(&self) -> Result<Noice> {
        self.setup_plugin::<Noice>("noice", |snacks| snacks.setup()?.call(self.noice_opts()))
    }

    pub fn load_noice(&self) {
        self.add_packs([
            "https://github.com/MunifTanjim/nui.nvim",
            "https://github.com/folke/noice.nvim",
        ]);

        self.on_very_lazy(|conf| {
            conf.req_noice()?;

            conf.with_vim_opt(|opt| {
                tbl!(out(opt), {
                    cmdheight = 0;
                })
            })?;

            Ok(())
        })
        .ok_or_notify(self);
    }

    fn noice_opts(&self) -> impl LuaSub<LuaDict<LuaVal>> {
        tbl!(owned, {
            lsp = tbl!(owned, {
                "override" = tbl!(owned, {
                    "vim.lsp.util.convert_input_to_markdown_lines" = true;
                    "vim.lsp.util.stylize_markdown" = true;
                    "cmp.entry.get_documentation" = true;
                });
            });
            routes = [tbl!(owned, {
                filter = tbl!(owned, {
                    event = "msg_show";
                    any = tbl_seq![
                        tbl!(owned, {
                            find = "%d+L, %d+B";
                        }),
                        tbl!(owned, {
                            find = "; after #%d+";
                        }),
                        tbl!(owned, {
                            find = "; before #%d+";
                        }),
                    ];
                });
                view = "mini";
            })];
            presets = tbl!(owned, {
                bottom_search = true;
                command_palette = true;
                long_message_to_split = true;
            });
            cmdline = tbl!(owned, {
                view = "cmdline";
            });
        })
    }
}
