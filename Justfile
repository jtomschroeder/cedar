
CXX_JSON = 'https://github.com/nlohmann/json/releases/download/v2.1.1/json.hpp'
CEF = 'Chromium Embedded Framework.framework/Chromium Embedded Framework'

dep:
	mkdir -p cocoa/ext/json
	curl -L {{CXX_JSON}} -o cocoa/ext/json/json.hpp

example EXAMPLE:
	install_name_tool -id "@executable_path/../Frameworks/{{CEF}}" "lib/cef/Release/{{CEF}}"

	cargo build --release --example {{EXAMPLE}}

	cp -a lib/cef/build/tests/cefsimple/Release/cefsimple.app .
	cp target/release/examples/{{EXAMPLE}} cefsimple.app/Contents/MacOS/cefsimple
	./cefsimple.app/Contents/MacOS/cefsimple
