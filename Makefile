win-min:
	cargo build --target x86_64-pc-windows-gnu --profile min-size
win:
	cargo build -r --target x86_64-pc-windows-gnu

local-min:
	cargo build --profile min-size
local:
	cargo build -r