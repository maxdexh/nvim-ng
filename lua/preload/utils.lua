Utils = {}

function Utils.try(f, ...)
	local function try_err_handler(err)
		if Snacks then
			Snacks.debug.backtrace(tostring(err), { level = "error" })
		else
			vim.notify(tostring(err) .. "\n" .. debug.traceback())
		end
	end

	local function ignore_status(status, ...)
		return ...
	end

	return ignore_status(xpcall(f, try_err_handler, ...))
end

function Utils.auto_import_dir(dirpath)
	local mod_dir = vim.fs.normalize(dirpath)

	for fname, type in vim.fs.dir(mod_dir) do
		if type == "file" and fname:match("%.lua$") then
			local fpath = vim.fs.normalize(vim.fs.joinpath(mod_dir, fname))
			Utils.try(dofile, fpath)
		end
	end
end
