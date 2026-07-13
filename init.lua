local config_path = vim.fn.stdpath("config") --[[@as string]]
local output = vim.system({
	"cargo",
	"build",
	--"--release",
	"--message-format=json-render-diagnostics",
}, {
	cwd = config_path,
	text = true,
}):wait()
if output.code ~= 0 then
	error(output.stderr)
end
local artifacts = vim.tbl_filter(
	function(msg)
		return msg.reason == "compiler-artifact" and msg.target.name == "nvim_config"
	end,
	vim.tbl_map(
		vim.json.decode, --
		vim.split(output.stdout --[[@as string]], "\n", { trimempty = true })
	)
)
if #artifacts ~= 1 or #artifacts[1].filenames ~= 1 then
	error("expected exactly one compiler-artifact message with one filename, found: \n" .. vim.inspect(artifacts))
end
local libpath = artifacts[1].filenames[1]
package.loadlib(libpath, "luaopen_nvim_config")()
