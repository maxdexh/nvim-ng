vim.pack.add({
	"https://github.com/nvim-tree/nvim-web-devicons",
	"https://github.com/akinsho/bufferline.nvim",
})

require("bufferline").setup({})

vim.keymap.set("n", "H", function()
	vim.cmd("BufferLineCyclePrev")
end, { desc = "Previous Buffer" })

vim.keymap.set("n", "L", function()
	vim.cmd("BufferLineCycleNext")
end, { desc = "Next Buffer" })
