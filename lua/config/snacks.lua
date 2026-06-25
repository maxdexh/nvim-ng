vim.pack.add({ "https://github.com/folke/snacks.nvim" })
require("snacks").setup({
	bigfile = { enabled = true },
	indent = { enabled = true },
	input = { enabled = true },
	notifier = { enabled = true },
	quickfile = { enabled = true },
	scope = { enabled = true },
	scroll = { enabled = true },
	statuscolumn = { enabled = false },
	words = { enabled = true },
	picker = { ui_select = true },
	dashboard = {
		enabled = true,
		preset = {
			---@type snacks.dashboard.Item[]
			keys = {
				{
					icon = " ",
					key = "f",
					desc = "Find File",
					action = function()
						Snacks.dashboard.pick("files")
					end,
				},
				{ icon = " ", key = "n", desc = "New File", action = ":ene | startinsert" },
				{
					icon = " ",
					key = "g",
					desc = "Find Text",
					action = function()
						Snacks.dashboard.pick("live_grep")
					end,
				},
				{
					icon = " ",
					key = "s",
					desc = "Restore Session",
					action = function()
						require("persistence").load({ last = true })
					end,
				},
				{ icon = " ", key = "q", desc = "Quit", action = ":qa" },
			},
		},
		---@type snacks.dashboard.Section
		sections = {
			{ section = "header" },
			{ section = "keys", gap = 1, padding = 1 },
		},
	},
})

-- Helper function to replicate LazyVim's root directory detection using Snacks
local function get_root()
	return Snacks.git.get_root() or vim.uv.cwd()
end

-- ============================================================================
-- Snacks.picker Keymaps
-- ============================================================================

-- Resume Picker
vim.keymap.set("n", "<leader>sx", function()
	Snacks.picker.resume()
end, { desc = "Resume Picker" })

-- Find Pickers
vim.keymap.set("n", "<leader>fP", function()
	Snacks.picker.pickers()
end, { desc = "Find Pickers" })

-- Find Files (cwd)
vim.keymap.set("n", "<leader>ff", function()
	Snacks.picker.files({ cwd = vim.uv.cwd() })
end, { desc = "Find Files (cwd)" })

-- Find Files (Root Dir)
vim.keymap.set("n", "<leader>fF", function()
	Snacks.picker.files({ cwd = get_root() })
end, { desc = "Find Files (Root Dir)" })

-- Selection Grep (cwd) - Visual Mode
vim.keymap.set("x", "<leader>sw", function()
	Snacks.picker.grep_word({ cwd = vim.uv.cwd() })
end, { desc = "Selection (cwd)" })

-- Selection Grep (Root Dir) - Visual Mode
vim.keymap.set("x", "<leader>sW", function()
	Snacks.picker.grep_word({ cwd = get_root() })
end, { desc = "Selection (Root Dir)" })

-- Grep (cwd)
vim.keymap.set("n", "<leader>sg", function()
	Snacks.picker.grep({ cwd = vim.uv.cwd() })
end, { desc = "Grep (cwd)" })

-- Grep (Root Dir)
vim.keymap.set("n", "<leader>sG", function()
	Snacks.picker.grep({ cwd = get_root() })
end, { desc = "Selection (Root Dir)" })

vim.keymap.set("n", "<leader>ca", function()
	vim.lsp.buf.code_action()
end, { desc = "Code Action" })

vim.keymap.set("n", "gd", function()
	Snacks.picker.lsp_definitions()
end, { desc = "Goto Definition" })
vim.keymap.set("n", "gi", function()
	Snacks.picker.lsp_implementations()
end, { desc = "View Implementations" })
vim.keymap.set("n", "gr", function()
	Snacks.picker.lsp_references()
end, { desc = "View References" })

-- Copy Selection (Visual mode)
vim.keymap.set("v", "<C-c>", '"+y', { desc = "Copy Selection" })

-- Exit Terminal mode (Terminal mode)
vim.keymap.set("t", "<ESC><ESC>", "<C-\\><C-n>", { desc = "Exit Terminal mode" })
