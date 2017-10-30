
CXX_JSON = "https://github.com/nlohmann/json/releases/download/v2.1.1/json.hpp"

APP = "out/cefsimple.app"
HELPER = "out/cefsimple.app/Contents/Frameworks/cefsimple Helper.app"
CEF = "$HOME/.cedar/lib/Chromium Embedded Framework.framework"

setup:
	mkdir -p "$HOME/.cedar/lib"
	cp -a "lib/cef/Release/Chromium Embedded Framework.framework" $HOME/.cedar/lib/.
	install_name_tool -id "{{CEF}}/Chromium Embedded Framework" "{{CEF}}/Chromium Embedded Framework"

dep:
	mkdir -p cocoa/ext/json
	curl -L {{CXX_JSON}} -o cocoa/ext/json/json.hpp

example EXAMPLE:
	# cp -a lib/cef/build/tests/cefsimple/Release/cefsimple.app .

	mkdir -p {{APP}}/Contents/{Frameworks,MacOS,Resources}

	cp lib/app/mac/Info.plist {{APP}}/Contents/.
	cp -a lib/app/mac/{Info.plist,*.icns,English.lproj} {{APP}}/Contents/Resources/.
	cp etc/*.html {{APP}}/Contents/Resources/.

	cp -a "{{CEF}}" {{APP}}/Contents/Frameworks/.

	mkdir -p "{{HELPER}}/Contents/MacOS"
	cp lib/app/mac/helper-Info.plist "{{HELPER}}/Contents/Info.plist"

	cargo build --release --example {{EXAMPLE}}

	cp target/release/examples/{{EXAMPLE}} {{APP}}/Contents/MacOS/cefsimple
	install_name_tool -add_rpath "@executable_path/.." {{APP}}/Contents/MacOS/cefsimple

	cargo build --release --bin helper

	cp target/release/helper "{{HELPER}}/Contents/MacOS/cefsimple Helper"
	install_name_tool -add_rpath "@executable_path/../../../.." "{{HELPER}}/Contents/MacOS/cefsimple Helper"

	./{{APP}}/Contents/MacOS/cefsimple
