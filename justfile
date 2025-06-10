default:
	just -l

serve:
	cargo watch -x "run build --serve --config ./default/config.toml --theme ./default/theme.css --content ../bookshelf/content --output ./build" -i ./_static

docs:
	cargo build --release
	./target/release/oro-jackson build --serve --config ./default/config.toml --theme ./default/theme.css --content ./docs --output ./build

build:
    cargo run build --config ./default/config.toml --theme ./default/theme.css --content ../bookshelf/content --output ./build

test:
    cargo test -- --nocapture

serve-tw:
	bunx @tailwindcss/cli -i ./tailwind.css -o ./_static/style.css --minify --watch

build-tw:
	bunx @tailwindcss/cli -i ./tailwind.css -o ./_static/style.css --minify
