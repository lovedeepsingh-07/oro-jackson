default:
	just -l

serve:
	cargo watch -x "run serve --config ./config.toml --content ../bookshelf/content --output ./build" -i ./_static
	# cargo run serve --config ./config.toml --content ../bookshelf/content --output ./build

build:
    cargo run build --config ./config.toml --content ../bookshelf/content --output ./build

test:
    cargo test -- --nocapture

tw-serve:
	bunx @tailwindcss/cli -i ./src/tailwind.css -o ./_static/style.css --minify --watch

tw-build:
	bunx @tailwindcss/cli -i ./src/tailwind.css -o ./_static/style.css --minify
