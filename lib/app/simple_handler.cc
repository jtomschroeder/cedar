
#include "simple_handler.h"

#include <iostream>
#include <sstream>
#include <string>
#include <thread>

#include "include/base/cef_bind.h"
#include "include/cef_app.h"
#include "include/views/cef_browser_view.h"
#include "include/views/cef_window.h"
#include "include/wrapper/cef_closure_task.h"
#include "include/wrapper/cef_helpers.h"

extern "C" {
void renderer_resp(void *, const char *);

char *renderer_recv(void *);
void renderer_string_drop(char *);
}

SimpleHandler *SimpleHandler::instance = nullptr;

SimpleHandler::SimpleHandler(void *renderer) : renderer(renderer), closing(false) {
    DCHECK(!instance);
    instance = this;
}

SimpleHandler::~SimpleHandler() {
    instance = nullptr;
}

SimpleHandler *SimpleHandler::GetInstance() {
    return instance;
}

void SimpleHandler::OnTitleChange(CefRefPtr<CefBrowser> browser, const CefString &title) {
    CEF_REQUIRE_UI_THREAD();
    PlatformTitleChange(browser, title);
}

bool SimpleHandler::OnConsoleMessage(CefRefPtr<CefBrowser>, const CefString &message,
                                     const CefString &, int) {
    // HACK: as a temporary solution, let's commandeer `console.log` to send events from JS to cedar
    //
    // long term:
    // - provide JS bindings from CEF renderer process and use IPC to talk to browser process

    auto msg = std::string{message};
    // std::cout << ">> console: " << msg << std::endl;

    renderer_resp(renderer, msg.c_str());

    return false;
}

void SimpleHandler::OnAfterCreated(CefRefPtr<CefBrowser> browser) {
    CEF_REQUIRE_UI_THREAD();
    browsers.push_back(browser);

    // Load cedar front-end code.

    // std::stringstream ss;
    // ss << "<html><body bgcolor=\"white\">"
    //       "<h2>Failed to load URL "
    //    << "</h2></body></html>";
    // browser->GetMainFrame()->LoadString(ss.str(), "cedar:home");
}

bool SimpleHandler::DoClose(CefRefPtr<CefBrowser>) {
    CEF_REQUIRE_UI_THREAD();

    // Closing the main window requires special handling. See the DoClose()
    // documentation in the CEF header for a detailed description of this
    // process.
    if (browsers.size() == 1) {
        // Set a flag to indicate that the window close should be allowed.
        closing = true;
    }

    // Allow the close. For windowed browsers this will result in the OS close
    // event being sent.
    return false;
}

void SimpleHandler::OnBeforeClose(CefRefPtr<CefBrowser> browser) {
    CEF_REQUIRE_UI_THREAD();

    // Remove from the list of existing browsers.
    auto it = browsers.begin();
    for (; it != browsers.end(); ++it) {
        if ((*it)->IsSame(browser)) {
            browsers.erase(it);
            break;
        }
    }

    if (browsers.empty()) {
        // All browser windows have closed. Quit the application message loop.
        CefQuitMessageLoop();
    }
}

void SimpleHandler::OnLoadError(CefRefPtr<CefBrowser>, CefRefPtr<CefFrame> frame,
                                ErrorCode errorCode, const CefString &errorText,
                                const CefString &failedUrl) {
    CEF_REQUIRE_UI_THREAD();

    // Don't display an error for downloaded files.
    if (errorCode == ERR_ABORTED) {
        return;
    }

    // Display a load error message.
    std::stringstream ss;
    ss << "<html><body bgcolor=\"white\">"
          "<h2>Failed to load URL "
       << std::string(failedUrl) << " with error " << std::string(errorText) << " (" << errorCode
       << ").</h2></body></html>";
    frame->LoadString(ss.str(), failedUrl);
}

void SimpleHandler::OnLoadEnd(CefRefPtr<CefBrowser>, CefRefPtr<CefFrame> frame, int) {
    // std::cout << "Load end!\n";

    if (frame->IsMain()) {
        // Once the main frame is finished loaded, kick off command receiver.
        std::thread([] {
            auto handler = SimpleHandler::GetInstance();
            while (true) {
                auto s = renderer_recv(handler->renderer);
                std::string str{s};
                renderer_string_drop(s);

                // std::cout << "Command: " << str << std::endl;
                // std::cout << "#B: " << handler->browsers.size() << std::endl;

                auto browser = handler->browsers.front();

                auto frame = browser->GetMainFrame();

                const auto code = "window.cedar.command('" + str + "');";
                frame->ExecuteJavaScript(code, frame->GetURL(), 0);
            }
        }).detach();
    }
}

void SimpleHandler::CloseAllBrowsers(bool force_close) {
    if (!CefCurrentlyOn(TID_UI)) {
        CefPostTask(TID_UI, base::Bind(&SimpleHandler::CloseAllBrowsers, this, force_close));
        return;
    }

    if (browsers.empty()) {
        return;
    }

    auto it = browsers.begin();
    for (; it != browsers.end(); ++it) {
        (*it)->GetHost()->CloseBrowser(force_close);
    }
}
