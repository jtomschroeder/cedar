
#import <Cocoa/Cocoa.h>

#include <iostream>
#include <string>
#include <thread>
#include <unordered_map>

#include <json/json.hpp>

using json = nlohmann::json;

@interface WindowDelegate : NSObject <NSWindowDelegate>
@end

@implementation WindowDelegate

- (NSSize)windowWillResize:(NSWindow *)sender toSize:(NSSize)size {
    printf("Resized Window!: %f %f\n", size.width, size.height);
    fflush(stdout);

    return size;
}

@end

@interface Action : NSObject {
    std::string identifier;
}
@end

@implementation Action

- (id)initWithID:(std::string)ident {
    if (self = [super init]) {
        self->identifier = ident;
    }
    return self;
}

- (void)click:(id)sender {
    std::cout << "click: " << self->identifier << std::endl;
}

@end

extern "C" void run() {
    @autoreleasepool {
        [NSApplication sharedApplication];
        [NSApp setActivationPolicy:NSApplicationActivationPolicyRegular];

        // build menu for window
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

        auto frame = NSMakeRect(0, 0, 500, 500);
        auto styleMask = NSWindowStyleMaskResizable | NSWindowStyleMaskTitled |
                         NSWindowStyleMaskMiniaturizable | NSWindowStyleMaskClosable;

        auto window = [[NSWindow alloc] initWithContentRect:frame
                                                  styleMask:styleMask
                                                    backing:NSBackingStoreBuffered
                                                      defer:NO];

        window.delegate = [[WindowDelegate alloc] init];

        [window cascadeTopLeftFromPoint:NSMakePoint(0, 0)];
        [window center];
        [window setTitle:@"** cedar **"];
        [window makeKeyAndOrderFront:nil];

        // Bring window to front
        auto app = [NSRunningApplication currentApplication];
        [app activateWithOptions:NSApplicationActivateIgnoringOtherApps];

        std::thread([&] {

            std::unordered_map<std::string, NSView *> widgets;

            std::string line;
            while (std::getline(std::cin, line)) {
                // std::cout << "received: " << line << std::endl;

                auto event = json::parse(line.c_str());

                // std::cout << "received: " << event["Create"][0] << std::endl;

                auto ident = event["Create"][0];
                auto widget = event["Create"][1];

                if (widget == "Button") {
                    auto frame = NSMakeRect(0, 0, 100, 100);
                    auto button = [[NSButton alloc] initWithFrame:frame];
                    button.bezelStyle = NSRoundedBezelStyle;

                    auto action = [[Action alloc] initWithID:ident];
                    [button setAction:@selector(click:)];
                    [button setTarget:action];

                    widgets[ident] = button;

                    [window.contentView addSubview:button];
                } else if (widget == "Label") {
                    auto frame = NSMakeRect(100, 100, 100, 100);
                    auto label = [[NSTextField alloc] initWithFrame:frame];

                    [label setStringValue:@"***"];

                    [label setBezeled:NO];
                    [label setDrawsBackground:NO];
                    [label setEditable:NO];
                    [label setSelectable:NO];

                    [label setAlignment:NSTextAlignmentCenter];

                    widgets[ident] = label;

                    [window.contentView addSubview:label];
                }
            }
        }).detach();

        [NSApp run];
    }
}
