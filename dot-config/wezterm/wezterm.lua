local wezterm = require("wezterm")

local config = wezterm.config_builder()
local act = wezterm.action

config.font = wezterm.font("DankMono Nerd Font")
config.font_size = 13

local my_gruvbox = wezterm.color.get_builtin_schemes()["Gruvbox dark, hard (base16)"]
my_gruvbox.background = "#181818"

-- config.color_scheme = "Gruvbox dark, hard (base16)"
config.color_schemes = {
	["MyGruvbox"] = my_gruvbox,
}
config.color_scheme = "MyGruvbox"
config.colors = {
	cursor_bg = "white",
	cursor_border = "white",
}
config.window_decorations = "RESIZE"

config.hide_tab_bar_if_only_one_tab = true
config.window_padding = {
	left = 0,
	right = 0,
	top = 0,
	bottom = 0,
}

config.keys = {
	{
		key = "o",
		mods = "SUPER",
		action = act.Multiple({
			act.SendKey({
				key = "a",
				mods = "CTRL",
			}),
			act.SendKey({
				key = "p",
				mods = "CTRL|ALT",
			}),
		}),
	},
}

return config
