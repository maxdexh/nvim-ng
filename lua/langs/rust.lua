vim.pack.add({ {
	src = "https://github.com/mrcjkb/rustaceanvim",
	version = vim.version.range("^9"),
} })

vim.g.rustaceanvim = {
	server = {
		default_settings = {
			["rust-analyzer"] = {
				assist = {
					preferSelf = true,
				},
			},
		},
	},
}

vim.api.nvim_create_autocmd("FileType", {
	pattern = "rust",
	callback = function()
		vim.o.shiftwidth = 0
		vim.o.tabstop = 4
	end,
})

vim.api.nvim_create_autocmd("FileType", {
	pattern = "rust",
	callback = function()
		require("nvim-treesitter").install({ "rust" })
	end,
	once = true,
})

require("conform").formatters_by_ft.rust = { "rustfmt" }
