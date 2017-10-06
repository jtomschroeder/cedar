
CXX_JSON = 'https://github.com/nlohmann/json/releases/download/v2.1.1/json.hpp'

dep:
	mkdir -p cocoa/ext/json
	curl -L {{CXX_JSON}} -o cocoa/ext/json/json.hpp

cocoa:
	(cd cocoa && cargo build --release)

example EXAMPLE: cocoa
	RUST_BACKTRACE=1 cargo run --release --example {{EXAMPLE}}
