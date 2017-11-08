
#import <Cocoa/Cocoa.h>
#import <WebKit/WebKit.h>

#include <iostream>
#include <thread>

static void *renderer = nullptr;

extern "C" {
void renderer_resp(void *, const char *);

char *renderer_recv(void *);
void renderer_string_drop(char *);
}

@interface ViewController : NSViewController <WKNavigationDelegate, WKScriptMessageHandler>
@end

@implementation ViewController
- (void)loadView {
    auto frame = NSMakeRect(0, 0, 500, 500);

    auto config = [[WKWebViewConfiguration alloc] init];

    auto controller = [[WKUserContentController alloc] init];
    [controller addScriptMessageHandler:self name:@"test"];

    config.userContentController = controller;

    auto webview = [[WKWebView alloc] initWithFrame:frame configuration:config];
    webview.navigationDelegate = self;

    // auto url = @"http://www.apple.com";
    // auto url = @"file:///Users/jtomschroeder/Code/WebKitPlayground/index.html";
    auto url = @"file:///Users/jtomschroeder/Code/cedar/lib/etc/view.html";

    auto req = [NSURLRequest requestWithURL:[NSURL URLWithString:url]];
    [webview loadRequest:req];

    self.view = webview;
}

- (void)viewDidLoad {
    [super viewDidLoad];
}

- (void)userContentController:(WKUserContentController *)userContentController
      didReceiveScriptMessage:(WKScriptMessage *)message {
    auto msg = [[message.body description] UTF8String];
    renderer_resp(renderer, msg);
}

- (void)webView:(WKWebView *)webView didFinishNavigation:(WKNavigation *)navigation {
    std::thread([webView] {
        while (true) {
            auto s = renderer_recv(renderer);
            std::string str{s};
            renderer_string_drop(s);

            const auto code = "window.cedar.command('" + str + "');";
            auto js = [[NSString stringWithUTF8String:code.c_str()] autorelease];

            dispatch_async(dispatch_get_main_queue(), ^{
              [webView evaluateJavaScript:js completionHandler:nil];
            });
        }
    }).detach();
}
@end

extern "C" void cocoa_app_run(void *r) {
    renderer = r;

    [NSAutoreleasePool new];

    [NSApplication sharedApplication];
    [NSApp setActivationPolicy:NSApplicationActivationPolicyRegular];

    auto menubar = [[NSMenu new] autorelease];
    auto appMenuItem = [[NSMenuItem new] autorelease];
    [menubar addItem:appMenuItem];
    [NSApp setMainMenu:menubar];
    auto appMenu = [[NSMenu new] autorelease];
    auto appName = [[NSProcessInfo processInfo] processName];
    auto quitTitle = [@"Quit " stringByAppendingString:appName];
    auto quitMenuItem = [
        [[NSMenuItem alloc] initWithTitle:quitTitle action:@selector(terminate:) keyEquivalent:@"q"]
        autorelease];
    [appMenu addItem:quitMenuItem];
    [appMenuItem setSubmenu:appMenu];

#ifdef MAC_OS_X_VERSION_10_12 // macOS >= 10.12 (for WindowMask deprecation)
    auto styleMask = NSWindowStyleMaskResizable | NSWindowStyleMaskTitled |
                     NSWindowStyleMaskMiniaturizable | NSWindowStyleMaskClosable;
#else
    auto styleMask = NSResizableWindowMask | NSTitledWindowMask | NSMiniaturizableWindowMask |
                     NSClosableWindowMask;
#endif

    auto window = [[[NSWindow alloc] initWithContentRect:NSMakeRect(0, 0, 200, 200)
                                               styleMask:styleMask
                                                 backing:NSBackingStoreBuffered
                                                   defer:NO] autorelease];

    window.contentViewController = [[ViewController alloc] init];

    [window cascadeTopLeftFromPoint:NSMakePoint(20, 20)];
    [window setTitle:appName];
    [window makeKeyAndOrderFront:nil];

    [NSApp activateIgnoringOtherApps:YES];
    [NSApp run];
}
