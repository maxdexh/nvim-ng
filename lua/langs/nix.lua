vim.api.nvim_create_autocmd("FileType", {
	pattern = "nix",
	callback = function()
		vim.o.shiftwidth = 0
		vim.o.tabstop = 2
	end,
})

vim.api.nvim_create_autocmd("FileType", {
	pattern = "nix",
	callback = function()
		require("nvim-treesitter").install({ "nix" })
	end,
	once = true,
})
