
#pragma once

#include "include/cef_client.h"

#include <list>

class SimpleHandler : public CefClient,
                      public CefDisplayHandler,
                      public CefLifeSpanHandler,
                      public CefLoadHandler {
public:
    explicit SimpleHandler(void *renderer);
    ~SimpleHandler();

    // Provide access to the single global instance of this object.
    static SimpleHandler *GetInstance();

    // CefClient methods:
    CefRefPtr<CefDisplayHandler> GetDisplayHandler() override {
        return this;
    }
    CefRefPtr<CefLifeSpanHandler> GetLifeSpanHandler() override {
        return this;
    }
    CefRefPtr<CefLoadHandler> GetLoadHandler() override {
        return this;
    }

    // CefDisplayHandler methods:
    void OnTitleChange(CefRefPtr<CefBrowser> browser, const CefString &title) override;

    // CefLifeSpanHandler methods:
    void OnAfterCreated(CefRefPtr<CefBrowser> browser) override;
    bool DoClose(CefRefPtr<CefBrowser> browser) override;
    void OnBeforeClose(CefRefPtr<CefBrowser> browser) override;

    // CefLoadHandler methods:
    void OnLoadError(CefRefPtr<CefBrowser>, CefRefPtr<CefFrame>, ErrorCode,
                     const CefString &errorText, const CefString &failedUrl) override;
    void OnLoadEnd(CefRefPtr<CefBrowser>, CefRefPtr<CefFrame>, int httpStatusCode) override;

    // Request that all existing browser windows close.
    void CloseAllBrowsers(bool force_close);

    bool IsClosing() const {
        return is_closing_;
    }

private:
    void PlatformTitleChange(CefRefPtr<CefBrowser> browser, const CefString &title);

    using BrowserList = std::list<CefRefPtr<CefBrowser>>;
    BrowserList browser_list_;

    void *renderer;
    bool is_closing_;

    IMPLEMENT_REFCOUNTING(SimpleHandler);
};
