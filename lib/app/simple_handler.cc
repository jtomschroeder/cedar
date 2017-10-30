
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

namespace {

SimpleHandler *g_instance = nullptr;

} // namespace

extern "C" {
void renderer_resp(void *, const char *);

char *renderer_recv(void *);
void renderer_string_drop(char *);
}

SimpleHandler::SimpleHandler(void *renderer) : renderer(renderer), is_closing_(false) {
    DCHECK(!g_instance);
    g_instance = this;

    // std::thread([] {
    //     auto handler = SimpleHandler::GetInstance();
    //     auto s = renderer_recv(handler->renderer);

    //     std::cout << "Command: " << s << std::endl;
    //     std::cout << "#B: " << handler->browser_list_.size() << std::endl;

    //     renderer_string_drop(s);
    // }).detach();
}

SimpleHandler::~SimpleHandler() {
    g_instance = nullptr;
}

// static
SimpleHandler *SimpleHandler::GetInstance() {
    return g_instance;
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
    std::cout << ">> console: " << msg << std::endl;

    renderer_resp(renderer, msg.c_str());

    return false;
}

void SimpleHandler::OnAfterCreated(CefRefPtr<CefBrowser> browser) {
    CEF_REQUIRE_UI_THREAD();
    browser_list_.push_back(browser);
}

bool SimpleHandler::DoClose(CefRefPtr<CefBrowser>) {
    CEF_REQUIRE_UI_THREAD();

    // Closing the main window requires special handling. See the DoClose()
    // documentation in the CEF header for a detailed destription of this
    // process.
    if (browser_list_.size() == 1) {
        // Set a flag to indicate that the window close should be allowed.
        is_closing_ = true;
    }

    // Allow the close. For windowed browsers this will result in the OS close
    // event being sent.
    return false;
}

void SimpleHandler::OnBeforeClose(CefRefPtr<CefBrowser> browser) {
    CEF_REQUIRE_UI_THREAD();

    // Remove from the list of existing browsers.
    BrowserList::iterator bit = browser_list_.begin();
    for (; bit != browser_list_.end(); ++bit) {
        if ((*bit)->IsSame(browser)) {
            browser_list_.erase(bit);
            break;
        }
    }

    if (browser_list_.empty()) {
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
    std::cout << "Load end!\n";

    if (frame->IsMain()) {
        // Once the main frame is finished loaded, kick off command receiver.
        std::thread([] {
            auto handler = SimpleHandler::GetInstance();
            while (true) {
                auto s = renderer_recv(handler->renderer);
                std::string str{s};
                renderer_string_drop(s);

                std::cout << "Command: " << str << std::endl;
                std::cout << "#B: " << handler->browser_list_.size() << std::endl;

                auto browser = handler->browser_list_.front();

                auto frame = browser->GetMainFrame();

                auto code = "window.cedar.command('" + str + "');";
                frame->ExecuteJavaScript(code, frame->GetURL(), 0);
            }
        }).detach();
    }
}

void SimpleHandler::CloseAllBrowsers(bool force_close) {
    if (!CefCurrentlyOn(TID_UI)) {
        // Execute on the UI thread.
        CefPostTask(TID_UI, base::Bind(&SimpleHandler::CloseAllBrowsers, this, force_close));
        return;
    }

    if (browser_list_.empty()) {
        return;
    }

    BrowserList::const_iterator it = browser_list_.begin();
    for (; it != browser_list_.end(); ++it) {
        (*it)->GetHost()->CloseBrowser(force_close);
    }
}
