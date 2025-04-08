default:
	just -l

serve:
    cargo run serve --content ../bookshelf/content --output ./build

tw-dev:
	bun run dev
tw-build:
	bun run build

build:
    cargo run build --content ../bookshelf/content --output ./build

test:
    cargo test -- --nocapture
