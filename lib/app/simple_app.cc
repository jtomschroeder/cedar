
#include "simple_app.h"

#include <iostream>
#include <string>

#include "include/cef_browser.h"
#include "include/cef_command_line.h"
#include "include/views/cef_window.h"
#include "include/wrapper/cef_helpers.h"

#include "simple_handler.h"

void SimpleApp::OnContextInitialized() {
    CEF_REQUIRE_UI_THREAD();

    auto command_line = CefCommandLine::GetGlobalCommandLine();

    CefRefPtr<SimpleHandler> handler(new SimpleHandler(renderer));

    CefBrowserSettings browser_settings;

    // const std::string url = "http://www.google.com";
    const std::string url = "file://" + resources + "/view.html";

    CefWindowInfo window_info;

#if defined(OS_WIN)
    // On Windows we need to specify certain flags that will be passed to CreateWindowEx().
    window_info.SetAsPopup(nullptr, "cefsimple");
#endif

    CefBrowserHost::CreateBrowser(window_info, handler, url, browser_settings, nullptr);
}

void SimpleApp::OnRenderProcessThreadCreated(CefRefPtr<CefListValue>) {
    std::cout << "Hello from " << __FUNCTION__ << std::endl;
}
