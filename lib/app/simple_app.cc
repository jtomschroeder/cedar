
#include "simple_app.h"

#include <string>

#include "include/cef_browser.h"
#include "include/cef_command_line.h"
#include "include/views/cef_window.h"
#include "include/wrapper/cef_helpers.h"

#include "simple_handler.h"

SimpleApp::SimpleApp() {}

void SimpleApp::OnContextInitialized() {
    CEF_REQUIRE_UI_THREAD();

    auto command_line = CefCommandLine::GetGlobalCommandLine();

    // SimpleHandler implements browser-level callbacks.
    CefRefPtr<SimpleHandler> handler(new SimpleHandler);

    // Specify CEF browser settings here.
    CefBrowserSettings browser_settings;

    const std::string url = "http://www.google.com";

    // Information used when creating the native window.
    CefWindowInfo window_info;

#if defined(OS_WIN)
    // On Windows we need to specify certain flags that will be passed to CreateWindowEx().
    window_info.SetAsPopup(NULL, "cefsimple");
#endif

    // Create the first browser window.
    CefBrowserHost::CreateBrowser(window_info, handler, url, browser_settings, NULL);
}
