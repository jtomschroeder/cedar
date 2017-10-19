
#import <Cocoa/Cocoa.h>

#include <iostream>
#include <string>
#include <thread>
#include <unordered_map>

#include "layout.h"
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

@interface WindowDelegate : NSObject <NSWindowDelegate>
@end

@implementation WindowDelegate

- (void)windowDidResize:(NSNotification *)notification {
    NSWindow *window = [notification object];
    [window.contentView.yoga calculate];
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

- (void)click:(id)__unused sender {
    auto command = json{{"Click", {{"id", self->identifier}}}};
    send(command);
}

@end

@interface View : NSView
@end

@implementation View

- (id)initWithFrame:(NSRect)frame {
    if (self = [super initWithFrame:frame]) {
    }
    return self;
}

- (BOOL)isFlipped {
    return YES;
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

        // [self setBezeled:YES];
        // [self setDrawsBackground:YES];
        // [self setEditable:YES];
        // [self setSelectable:YES];

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
    // [view.widthAnchor constraintGreaterThanOrEqualToConstant:150.0].active = YES;
}

auto Window() {
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

    window.delegate = [[WindowDelegate alloc] init];

    return window;
}

auto Stack() {
    auto stack = [[View alloc] init];
    YGNodeStyleSetFlexDirection(stack.yoga.node, YGFlexDirectionColumn);

    // Set background color of `stack` (for debugging)
    [stack setWantsLayer:YES];
    stack.layer.backgroundColor =
        [NSColor colorWithCalibratedRed:0.227f green:0.251f blue:0.667 alpha:0.25].CGColor;

    return stack;
}

void append(NSView *container, NSView *view) {
    std::cout << "append: " << container << " " << view << "\n";

    [container.yoga insert:view];

    dispatch_async(dispatch_get_main_queue(), ^{
      [container addSubview:view];
    });
}

extern "C" void run(void *r) {
    renderer = r;

    {
        // compile-time checks
        static_assert(!__has_feature(objc_arc), "verify ARC is NOT enabled!");
    }

    [NSApplication sharedApplication];
    [NSApp setActivationPolicy:NSApplicationActivationPolicyRegular];

    auto window = Window();

    // Bring window to front
    // NOTE: when Launch Services launches an .app bundle, it takes care of this automatically (?)
    auto app = [NSRunningApplication currentApplication];
    [app activateWithOptions:NSApplicationActivateIgnoringOtherApps];

    window.contentView = [[View alloc] initWithFrame:window.contentView.frame];
    auto container = window.contentView;

    // [container setFrame:window.contentView.frame];
    // [window.contentView addSubview:container];

    std::cout << container.yoga.node << "\n";
    // [container.yoga calculate];

    std::thread([&] {
        std::unordered_map<std::string, NSView *> widgets;

        while (true) {
            auto s = renderer_recv(renderer);
            const auto &command = json::parse(s);
            renderer_string_drop(s);

            std::cerr << command << std::endl;

            if (command.count("Create")) {

                auto &create = command["Create"];

                const std::string &ident = create["id"];
                const std::string &parent = create["parent"];

                auto &widget = create["kind"];
                auto &attributes = create["attributes"];

                if (widget == "Stack") {
                    auto stack = Stack();
                    // [stack setFrame:window.contentView.frame];

                    widgets[ident] = stack;

                    auto prnt = (parent.empty()) ? container : (NSStackView *)widgets[parent];
                    append(prnt, stack);

                } else if (widget == "Button") {
                    auto button = [[NSButton alloc] init];
                    button.bezelStyle = NSRoundedBezelStyle;

                    const std::string &text = attributes["Text"];
                    button.title = [NSString stringWithUTF8String:text.c_str()];

                    auto action = [[Action alloc] initWithID:ident];
                    [button setAction:@selector(click:)];
                    [button setTarget:action];

                    constrain(button);
                    widgets[ident] = button;

                    auto prnt = (parent.empty()) ? container : (NSStackView *)widgets[parent];
                    append(prnt, button);

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

                    auto prnt = (parent.empty()) ? container : (NSStackView *)widgets[parent];
                    append(prnt, label);

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

                    auto prnt = (parent.empty()) ? container : (NSStackView *)widgets[parent];
                    append(prnt, field);

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

            [container.yoga calculate];
        }
    }).detach();

    {
        // TODO: reposition frame origin as well
        auto frame = window.frame;
        frame.size = NSMakeSize(250, 250);
        [window setFrame:frame display:YES animate:YES];
    }

    [NSApp run];
}
