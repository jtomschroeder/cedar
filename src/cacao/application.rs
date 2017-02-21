
use std::sync::atomic::AtomicPtr;

use objc;
use cocoa::appkit;
use cocoa::base::{id, selector, nil};

use cocoa::foundation::{NSProcessInfo, NSString};

pub struct Application {
    pool: AtomicPtr<objc::runtime::Object>,
    app: AtomicPtr<objc::runtime::Object>,
}

impl Application {
    pub fn new() -> Self {
        use cocoa::foundation::NSAutoreleasePool;
        use cocoa::appkit::{NSMenu, NSMenuItem};
        use cocoa::appkit::NSApplication;

        unsafe {
            let app = appkit::NSApp();
            app.setActivationPolicy_(appkit::NSApplicationActivationPolicyRegular);

            // create Menu Bar
            let menubar = NSMenu::new(nil).autorelease();
            let app_menu_item = NSMenuItem::new(nil).autorelease();
            menubar.addItem_(app_menu_item);
            app.setMainMenu_(menubar);

            // create Application menu
            let app_menu = NSMenu::new(nil).autorelease();
            let quit_prefix = NSString::alloc(nil).init_str("Quit ");
            let quit_title =
                quit_prefix.stringByAppendingString_(NSProcessInfo::processInfo(nil).processName());
            let quit_action = selector("terminate:");
            let quit_key = NSString::alloc(nil).init_str("q");
            let quit_item = NSMenuItem::alloc(nil)
                .initWithTitle_action_keyEquivalent_(quit_title, quit_action, quit_key)
                .autorelease();
            app_menu.addItem_(quit_item);
            app_menu_item.setSubmenu_(app_menu);

            Application {
                pool: AtomicPtr::new(NSAutoreleasePool::new(nil)),
                app: AtomicPtr::new(app),
            }
        }
    }

    pub fn run(mut self) {
        use cocoa::appkit::{NSApplication, NSRunningApplication};

        unsafe {
            // Set `app` to 'running' and run!
            let app = NSRunningApplication::currentApplication(nil);
            app.activateWithOptions_(appkit::NSApplicationActivateIgnoringOtherApps);
            self.app.get_mut().run()
        }
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        use cocoa::foundation::NSAutoreleasePool;
        unsafe { self.pool.get_mut().drain() };
    }
}
