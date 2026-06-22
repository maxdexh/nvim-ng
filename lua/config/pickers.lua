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
