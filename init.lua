vim.pack.add({
	"https://github.com/folke/trouble.nvim",
	"https://github.com/neovim/nvim-lspconfig",
	{
		src = "https://github.com/nvim-treesitter/nvim-treesitter",
		version = "main",
	},
})

require("trouble").setup({})
require("nvim-treesitter").install({ "rust", "lua" })

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

vim.pack.add({ "https://github.com/stevearc/conform.nvim" })
require("conform").setup({
	formatters_by_ft = {
		lua = { "stylua" },
		rust = { "rustfmt" },
	},
	format_on_save = {
		-- These options will be passed to conform.format()
		timeout_ms = 500,
		lsp_format = "fallback",
	},
})

vim.o.shiftwidth = 0
vim.o.tabstop = 2
vim.o.number = true
vim.o.relativenumber = true

vim.g.snacks_animate = false
vim.g.mapleader = " "

vim.diagnostic.config({
	virtual_text = {
		severity = {
			min = vim.diagnostic.severity.WARN,
		},
	},
})

local function set_ft_tab_width(ft, width)
	vim.api.nvim_create_autocmd("FileType", {
		pattern = ft,
		callback = function()
			vim.o.shiftwidth = width
			vim.o.tabstop = width
		end,
	})
end

set_ft_tab_width("rust", 4)
set_ft_tab_width("nix", 2)

require("config.snacks")
require("config.buffers")
require("config.completions")
require("config.colourscheme")
require("config.explorer")
require("config.persistence")
