
CXX_JSON = 'https://github.com/nlohmann/json/releases/download/v2.1.1/json.hpp'
# CEF = 'Chromium Embedded Framework.framework/Chromium Embedded Framework'

dep:
	mkdir -p cocoa/ext/json
	curl -L {{CXX_JSON}} -o cocoa/ext/json/json.hpp

example EXAMPLE:
	cp -a lib/cef/build/tests/cefsimple/Release/cefsimple.app .

	cargo build --release --example {{EXAMPLE}}

	cp target/release/examples/{{EXAMPLE}} cefsimple.app/Contents/MacOS/cefsimple
	install_name_tool -add_rpath '@executable_path/..' cefsimple.app/Contents/MacOS/cefsimple

	cargo build --release --bin helper

	cp target/release/helper 'cefsimple.app/Contents/Frameworks/cefsimple Helper.app/Contents/MacOS/cefsimple Helper'
	install_name_tool -add_rpath '@executable_path/../../../..' 'cefsimple.app/Contents/Frameworks/cefsimple Helper.app/Contents/MacOS/cefsimple Helper'

	./cefsimple.app/Contents/MacOS/cefsimple
