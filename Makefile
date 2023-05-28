build:
	cargo build
	mv target/debug/ChainSauce_task1 src/

run:
	cargo run
	
debug:
	RUST_LOG=DEBUG cargo run -- run --verbose

test:
	cargo test