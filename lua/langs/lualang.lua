vim.api.nvim_create_autocmd("FileType", {
	pattern = "lua",
	callback = function()
		vim.o.shiftwidth = 0
		vim.o.tabstop = 2
	end,
})

vim.api.nvim_create_autocmd("FileType", {
	pattern = "lua",
	callback = function()
		require("nvim-treesitter").install({ "lua" })
	end,
	once = true,
})

require("conform").formatters_by_ft.lua = { "stylua" }
