PROJECT_DIRS := project/data_structure fruit-salad vecdeque-fruit-salad linkedinlist_fruit_salad hashmap_count hashmap_language_comparison data_race btree_fruit_salad hashmap_fruit binaryheap_fruit

rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	for dir in $(PROJECT_DIRS); do \
		cd $$dir && cargo fmt --quiet && cd ..; \
	done

lint:
	for dir in $(PROJECT_DIRS); do \
		cd $$dir && cargo clippy --quiet && cd ..; \
	done

test:
	for dir in $(PROJECT_DIRS); do \
		cd $$dir && cargo test && cd ..; \
	done

run:
	for dir in $(PROJECT_DIRS); do \
		cd $$dir && cargo run && cd ..; \
	done

release:
	for dir in $(PROJECT_DIRS); do \
		cd $$dir && cargo build --release && cd ..; \
	done

all: format lint test run