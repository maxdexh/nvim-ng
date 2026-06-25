vim.o.shiftwidth = 0
vim.o.tabstop = 2
vim.o.number = true
vim.o.relativenumber = true

vim.g.snacks_animate = false
vim.g.mapleader = " " -- NOTE: Must set before leader binds are set

vim.diagnostic.config({
	virtual_text = {
		severity = {
			min = vim.diagnostic.severity.WARN,
		},
	},
})
