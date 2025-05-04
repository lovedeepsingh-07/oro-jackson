default:
	just -l

serve:
    cargo run serve --content ../bookshelf/content --output ./build

tw-dev:
	bunx @tailwindcss/cli -i ./src/tailwind.css -o ./_static/style.css --minify --watch
tw-build:
	bunx @tailwindcss/cli -i ./src/tailwind.css -o ./_static/style.css --minify

build:
    cargo run build --content ../bookshelf/content --output ./build

test:
    cargo test -- --nocapture
