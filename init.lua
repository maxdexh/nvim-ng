require("preload.utils")
Utils.try(require, "preload.snacks")
Utils.try(require, "preload.options")

vim.pack.add({
	"https://github.com/neovim/nvim-lspconfig",
	{
		src = "https://github.com/nvim-treesitter/nvim-treesitter",
		version = "main",
	},
})
for _, dirname in ipairs({ "config", "langs" }) do
	Utils.auto_import_dir(vim.fs.joinpath(vim.fn.stdpath("config"), "lua", dirname))
end
