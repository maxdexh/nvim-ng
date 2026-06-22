vim.pack.add({
	{ src = "https://github.com/barrettruth/canola.nvim" },
})

require("oil").setup({
	default_file_explorer = true,
	buf_options = {
		buflisted = false,
	},
	float = {
		border = "rounded",
	},
	delete_to_trash = true,
	-- TODO: Reenable when better
	--	skip_confirm_for_simple_edits = true,
	prompt_save_on_select_new_entry = true,
})

vim.keymap.set("n", "<leader>fe", "<cmd>Oil --float<cr>", { desc = "Oil (Float)" })
vim.keymap.set("n", "<leader>fE", "<cmd>Oil<cr>", { desc = "Oil (Buffer)" })
