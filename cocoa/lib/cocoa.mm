
#import <Cocoa/Cocoa.h>

@interface WindowDelegate : NSObject <NSWindowDelegate>
@end

@implementation WindowDelegate

- (NSSize)windowWillResize:(NSWindow *)sender toSize:(NSSize)size {
    printf("Resized Window!: %f %f\n", size.width, size.height);
    fflush(stdout);

    return size;
}

@end

extern "C" void run() {
    @autoreleasepool {
        printf("TESTING!\n");
        fflush(stdout);

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

        [NSApp run];
    }
}
