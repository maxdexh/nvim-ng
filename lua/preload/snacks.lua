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
