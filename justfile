default:
	just -l

serve:
	cargo watch -x "run build --serve --config ./config.toml --content ../bookshelf/content --output ./build" -i ./_static

build:
    cargo run build --config ./config.toml --content ../bookshelf/content --output ./build

test:
    cargo test -- --nocapture

serve-tw:
	bunx @tailwindcss/cli -i ./tailwind.css -o ./_static/style.css --minify --watch

build-tw:
	bunx @tailwindcss/cli -i ./tailwind.css -o ./_static/style.css --minify
