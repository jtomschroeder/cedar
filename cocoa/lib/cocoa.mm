
#import <Cocoa/Cocoa.h>

#include <iostream>
#include <string>
#include <thread>
#include <unordered_map>

#include <json/json.hpp>

using json = nlohmann::json;

static void *interconnect = nullptr;
extern "C" {
void ic_send(void *, const char *);

char *ic_recv(void *);
void ic_string_drop(char *);
}

template <class C>
void send(const C &command) {
    ic_send(interconnect, command.dump().c_str());
}

@interface WindowDelegate : NSObject <NSWindowDelegate>
@end

@implementation WindowDelegate

- (void)windowDidResize:(NSNotification *)notification {
    NSWindow *window = [notification object];
    const auto frame = window.contentView.frame;

    auto command = json{{"Resize", {{"width", frame.size.width}, {"height", frame.size.height}}}};
    send(command);
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

- (id)initWithFrame:(NSRect)frame ID:(std::string)ident {
    if (self = [super initWithFrame:frame]) {
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

extern "C" void run(void *ic) {
    static_assert(!__has_feature(objc_arc), "verify ARC is NOT enabled!");

    interconnect = ic;

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

    [window setContentView:[[View alloc] init]];
    [window setDelegate:[[WindowDelegate alloc] init]];

    [window cascadeTopLeftFromPoint:NSMakePoint(0, 0)];
    [window center];
    [window setTitle:@"** cedar **"];
    [window makeKeyAndOrderFront:nil];

    // Bring window to front
    auto app = [NSRunningApplication currentApplication];
    [app activateWithOptions:NSApplicationActivateIgnoringOtherApps];

    std::thread([&] {
        std::unordered_map<std::string, NSView *> widgets;

        while (true) {
            auto s = ic_recv(interconnect);
            const auto &command = json::parse(s);
            ic_string_drop(s);

            if (command.count("Create")) {
                // std::cerr << command << std::endl;

                auto &create = command["Create"];
                auto &ident = create["id"];
                auto &widget = create["kind"];
                auto &frame = create["frame"];
                auto &attributes = create["attributes"];

                const float left = frame[0];
                const float top = frame[1];
                const float width = frame[2];
                const float height = frame[3];

                const auto rframe = NSMakeRect(left, top, width, height);

                if (widget == "Button") {
                    auto button = [[NSButton alloc] initWithFrame:rframe];
                    button.bezelStyle = NSRoundedBezelStyle;

                    const std::string &text = attributes["Text"];
                    button.title = [NSString stringWithUTF8String:text.c_str()];

                    auto action = [[Action alloc] initWithID:ident];
                    [button setAction:@selector(click:)];
                    [button setTarget:action];

                    widgets[ident] = button;
                    [window.contentView addSubview:button];
                } else if (widget == "Label") {
                    auto label = [[NSTextField alloc] initWithFrame:rframe];

                    const std::string &text = attributes["Text"];
                    [label setStringValue:[NSString stringWithUTF8String:text.c_str()]];

                    [label setBezeled:NO];
                    [label setDrawsBackground:NO];
                    [label setEditable:NO];
                    [label setSelectable:NO];

                    [label setAlignment:NSTextAlignmentCenter];

                    widgets[ident] = label;
                    [window.contentView addSubview:label];
                } else if (widget == "Field") {
                    auto field = [[TextField alloc] initWithFrame:rframe ID:ident];

                    auto placeholder = attributes.find("Placeholder");
                    if (placeholder != attributes.end()) {
                        const std::string &text = *placeholder;

                        auto string = [[NSAttributedString alloc]
                            initWithString:[NSString stringWithUTF8String:text.c_str()]];

                        [field setPlaceholderAttributedString:string];
                    }

                    widgets[ident] = field;
                    [window.contentView addSubview:field];
                } else {
                    std::cerr << "Unknown widget: " << widget << std::endl;
                }

            } else if (command.count("Update")) {
                auto &update = command["Update"];
                auto ident = update[0];
                auto attribute = update[1];
                std::string value = update[2];

                if (attribute == "Text") {
                    auto field = (NSTextField *)(widgets[ident]);
                    [field setStringValue:[NSString stringWithUTF8String:value.c_str()]];
                }
            } else if (command.count("Move")) {
                // auto &move = command["Move"];
                // std::cerr << move << std::endl;

                for (auto &move : command["Move"]) {
                    auto ident = move[0];

                    auto widget = widgets.find(ident);
                    if (widget != widgets.end()) {
                        auto &frame = move[1];

                        const float left = frame[0];
                        const float top = frame[1];
                        const float width = frame[2];
                        const float height = frame[3];

                        [widget->second setFrame:NSMakeRect(left, top, width, height)];
                    }
                }

            } else {
                std::cerr << "Unknown command: " << command << std::endl;
            }
        }
    }).detach();

    [NSApp run];
}
