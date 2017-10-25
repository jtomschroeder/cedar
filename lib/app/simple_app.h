
#pragma once

#include "include/cef_app.h"

// Implement application-level callbacks for the browser process.
class SimpleApp : public CefApp, public CefBrowserProcessHandler {
public:
    SimpleApp();

    // CefApp methods:
    CefRefPtr<CefBrowserProcessHandler> GetBrowserProcessHandler() override {
        return this;
    }

    // CefBrowserProcessHandler methods:
    void OnContextInitialized() override;
    void OnRenderProcessThreadCreated(CefRefPtr<CefListValue>) override;

private:
    IMPLEMENT_REFCOUNTING(SimpleApp);
};
