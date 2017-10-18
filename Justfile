
CXX_JSON = 'https://github.com/nlohmann/json/releases/download/v2.1.1/json.hpp'

dep:
	mkdir -p cocoa/ext/json
	curl -L {{CXX_JSON}} -o cocoa/ext/json/json.hpp

example EXAMPLE:
	RUST_BACKTRACE=full cargo run --release --example {{EXAMPLE}}
