
#import <Cocoa/Cocoa.h>

#include <iostream>
#include <string>
#include <thread>
#include <unordered_map>

#include "json/json.hpp"

using json = nlohmann::json;

static void *renderer = nullptr;
extern "C" {
void renderer_resp(void *, const char *);

char *renderer_recv(void *);
void renderer_string_drop(char *);
}

template <class C>
void send(const C &command) {
    renderer_resp(renderer, command.dump().c_str());
}

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

- (void)click:(id)__unused sender {
    auto command = json{{"Click", {{"id", self->identifier}}}};
    send(command);
}

@end

// TODO: create Label & Button classes
// TODO: split into separate files

@interface TextField : NSTextField <NSTextFieldDelegate> {
    std::string identifier;
}
@end

@implementation TextField

- (id)initWithID:(std::string)ident {
    if (self = [super init]) {
        self->identifier = ident;

        [self setBezeled:YES];
        [self setDrawsBackground:YES];
        [self setEditable:YES];
        [self setSelectable:YES];

        [self setDelegate:self];
    }
    return self;
}

- (void)controlTextDidChange:(NSNotification *)__unused notification {
    auto value = [[self stringValue] UTF8String];
    auto command = json{{"Change", {{"id", self->identifier}, {"value", value}}}};
    send(command);
}

@end

void constrain(NSView *view) {
    // Set 'minimum width' using anchor
    auto constraint = [view.widthAnchor constraintGreaterThanOrEqualToConstant:120.0];
    constraint.active = YES;
}

NSStackView *make_stack() {
    auto stack = [[NSStackView alloc] init];

    // Set background color of `stack` (for debugging)
    // [stack setWantsLayer:YES];
    // stack.layer.backgroundColor =
    //     [NSColor colorWithCalibratedRed:0.227f green:0.251f blue:0.667 alpha:0.5].CGColor;

    [stack setAutoresizingMask:(NSViewWidthSizable | NSViewHeightSizable)];

    [stack setOrientation:NSUserInterfaceLayoutOrientationVertical];
    // [stack setSpacing:5.0];

    [stack setDistribution:NSStackViewDistributionGravityAreas];
    // [stack setDistribution:NSStackViewDistributionEqualSpacing];

    [stack setEdgeInsets:NSEdgeInsetsMake(10, 10, 10, 10)];

    [stack setHuggingPriority:NSLayoutPriorityWindowSizeStayPut
               forOrientation:NSLayoutConstraintOrientationHorizontal];

    return stack;
}

extern "C" void run(void *r) {
    renderer = r;

    {
        // compile-time checks
        static_assert(!__has_feature(objc_arc), "verify ARC is NOT enabled!");
    }

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

    auto frame = NSMakeRect(0, 0, 250, 250);

#ifdef MAC_OS_X_VERSION_10_12 // macOS >= 10.12 (for WindowMask deprecation)
    auto styleMask = NSWindowStyleMaskResizable | NSWindowStyleMaskTitled |
                     NSWindowStyleMaskMiniaturizable | NSWindowStyleMaskClosable;
#else
    auto styleMask = NSResizableWindowMask | NSTitledWindowMask | NSMiniaturizableWindowMask |
                     NSClosableWindowMask;
#endif

    auto window = [[NSWindow alloc] initWithContentRect:frame
                                              styleMask:styleMask
                                                backing:NSBackingStoreBuffered
                                                  defer:NO];

    [window cascadeTopLeftFromPoint:NSMakePoint(0, 0)];
    [window center];
    [window setTitle:@"** cedar **"];
    [window makeKeyAndOrderFront:nil];

    // Bring window to front
    // NOTE: when Launch Services launches an .app bundle, it takes care of this automatically (?)
    auto app = [NSRunningApplication currentApplication];
    [app activateWithOptions:NSApplicationActivateIgnoringOtherApps];

    auto stack = make_stack();

    [stack setFrame:window.contentView.frame];
    [window.contentView addSubview:stack];

    std::thread([&] {
        std::unordered_map<std::string, NSView *> widgets;

        while (true) {
            auto s = renderer_recv(renderer);
            const auto &command = json::parse(s);
            renderer_string_drop(s);

            if (command.count("Create")) {
                // std::cerr << command << std::endl;

                auto &create = command["Create"];
                auto &ident = create["id"];
                auto &widget = create["kind"];
                auto &attributes = create["attributes"];

                if (widget == "Button") {
                    auto button = [[NSButton alloc] init];
                    button.bezelStyle = NSRoundedBezelStyle;

                    const std::string &text = attributes["Text"];
                    button.title = [NSString stringWithUTF8String:text.c_str()];

                    auto action = [[Action alloc] initWithID:ident];
                    [button setAction:@selector(click:)];
                    [button setTarget:action];

                    constrain(button);
                    widgets[ident] = button;

                    dispatch_async(dispatch_get_main_queue(), ^{
                      [stack addView:button inGravity:NSStackViewGravityTop];
                    });
                } else if (widget == "Label") {
                    auto label = [[NSTextField alloc] init];

                    const std::string &text = attributes["Text"];
                    [label setStringValue:[NSString stringWithUTF8String:text.c_str()]];

                    [label setBezeled:NO];
                    [label setDrawsBackground:NO];
                    [label setEditable:NO];
                    [label setSelectable:NO];

                    [label setAlignment:NSTextAlignmentCenter];

                    constrain(label);
                    widgets[ident] = label;

                    dispatch_async(dispatch_get_main_queue(), ^{
                      [stack addView:label inGravity:NSStackViewGravityTop];
                    });
                } else if (widget == "Field") {
                    auto field = [[TextField alloc] initWithID:ident];

                    auto placeholder = attributes.find("Placeholder");
                    if (placeholder != attributes.end()) {
                        const std::string &text = *placeholder;

                        auto string = [[NSAttributedString alloc]
                            initWithString:[NSString stringWithUTF8String:text.c_str()]];

                        [field setPlaceholderAttributedString:string];
                    }

                    constrain(field);
                    widgets[ident] = field;

                    dispatch_async(dispatch_get_main_queue(), ^{
                      [stack addView:field inGravity:NSStackViewGravityTop];
                    });
                } else {
                    std::cerr << "Unknown widget: " << widget << std::endl;
                }

            } else if (command.count("Update")) {
                auto &update = command["Update"];
                auto &ident = update["id"];
                auto &attribute = update["attribute"];
                std::string value = update["value"];

                if (attribute == "Text") {
                    auto field = (NSTextField *)(widgets[ident]);
                    dispatch_async(dispatch_get_main_queue(), ^{
                      [field setStringValue:[NSString stringWithUTF8String:value.c_str()]];
                    });
                }
            } else if (command.count("Remove")) {
                auto remove = command["Remove"];
                auto &ident = remove["id"];

                auto it = widgets.find(ident);
                if (it != widgets.end()) {
                    auto widget = it->second;
                    widgets.erase(it);

                    dispatch_async(dispatch_get_main_queue(), ^{
                      [widget removeFromSuperview];
                      [widget release];
                    });
                }
            } else {
                std::cerr << "Unknown command: " << command << std::endl;
            }
        }
    }).detach();

    [NSApp run];
}
