
#import <Cocoa/Cocoa.h>

#include "include/cef_application_mac.h"
#include "include/wrapper/cef_helpers.h"

#include "simple_app.h"
#include "simple_handler.h"

// Receives notifications from the application.
@interface SimpleAppDelegate : NSObject <NSApplicationDelegate>
- (void)createApplication:(id)object;
- (void)tryToTerminateApplication:(NSApplication *)app;
@end

// Provide the CefAppProtocol implementation required by CEF.
@interface SimpleApplication : NSApplication <CefAppProtocol> {
@private
    BOOL handlingSendEvent_;
}
@end

@implementation SimpleApplication
- (BOOL)isHandlingSendEvent {
    return handlingSendEvent_;
}

- (void)setHandlingSendEvent:(BOOL)handlingSendEvent {
    handlingSendEvent_ = handlingSendEvent;
}

- (void)sendEvent:(NSEvent *)event {
    CefScopedSendingEvent sendingEventScoper;
    [super sendEvent:event];
}

- (void)terminate:(id)__unused sender {
    SimpleAppDelegate *delegate = static_cast<SimpleAppDelegate *>([NSApp delegate]);
    [delegate tryToTerminateApplication:self];
    // Return, don't exit. The application is responsible for exiting on its own.
}
@end

@implementation SimpleAppDelegate

// Create the application on the UI thread.
- (void)createApplication:(id)__unused object {
    [NSApplication sharedApplication];

    {
        auto menubar = [NSMenu new];

        auto app_menu_item = [NSMenuItem new];
        [menubar addItem:app_menu_item];

        [NSApp setMainMenu:menubar];

        auto app_menu = [NSMenu new];
        auto quit_item = [[NSMenuItem alloc] initWithTitle:@"Quit"
                                                    action:@selector(terminate:)
                                             keyEquivalent:@"q"];
        [app_menu addItem:quit_item];
        [app_menu_item setSubmenu:app_menu];
    }

    // Set the delegate for application events.
    [[NSApplication sharedApplication] setDelegate:self];
}

- (void)tryToTerminateApplication:(NSApplication *)__unused app {
    SimpleHandler *handler = SimpleHandler::GetInstance();
    if (handler && !handler->IsClosing()) {
        handler->CloseAllBrowsers(false);
    }
}

- (NSApplicationTerminateReply)applicationShouldTerminate:(NSApplication *)__unused sender {
    return NSTerminateNow;
}
@end

// Entry point function for the browser process.
extern "C" void cef_app_run(void *renderer) {
    CefMainArgs main_args(0, nullptr);

    NSAutoreleasePool *autopool = [[NSAutoreleasePool alloc] init];

    [SimpleApplication sharedApplication];

    CefSettings settings;

    auto resources = [[NSBundle mainBundle] resourcePath];

    CefRefPtr<SimpleApp> app(new SimpleApp(renderer, [resources UTF8String]));

    // Initialize CEF for the browser process.
    CefInitialize(main_args, settings, app.get(), nullptr);

    // Create the application delegate.
    NSObject *delegate = [[SimpleAppDelegate alloc] init];
    [delegate performSelectorOnMainThread:@selector(createApplication:)
                               withObject:nil
                            waitUntilDone:NO];

    // Run the CEF message loop. This will block until CefQuitMessageLoop() is called.
    CefRunMessageLoop();

    CefShutdown();

    [delegate release];
    [autopool release];
}
