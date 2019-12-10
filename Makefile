test:
	cargo test

coverage: clean
	cargo kcov --all -- --exclude-path=src/download,src/post_processors --exclude-pattern=/.cargo,src/main.rs

open-coverage: ./target/cov/index.html
	xdg-open ./target/cov/index.html

clean:
	cargo clean
