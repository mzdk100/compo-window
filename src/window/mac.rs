use {
    compo::prelude::*,
    objc2::{MainThreadMarker, MainThreadOnly, msg_send, rc::Retained},
    objc2_app_kit::{
        NSApplication, NSApplicationActivationPolicy, NSBackingStoreType, NSEventModifierFlags,
        NSMenu, NSMenuItem, NSWindow, NSWindowStyleMask,
    },
    objc2_foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString},
    std::sync::Once,
    tracing::{error, info},
};

static MENU_SETUP: Once = Once::new();

// Setup application menu with keyboard shortcuts
unsafe fn setup_app_menu(app: &NSApplication, mtm: MainThreadMarker) {
    // Create main menu bar
    let main_menu = NSMenu::new(mtm);

    // Create app menu (first menu item)
    let app_menu_item = NSMenuItem::new(mtm);
    let app_menu = NSMenu::new(mtm);

    // Add Quit menu item with Command+Q shortcut
    let quit_title = NSString::from_str("Quit");
    let quit_key = NSString::from_str("q");
    let quit_item: Retained<NSMenuItem> = msg_send![NSMenuItem::alloc(mtm),
        initWithTitle: &*quit_title,
        action: objc2::sel!(terminate:),
        keyEquivalent: &*quit_key,
    ];
    quit_item.setKeyEquivalentModifierMask(NSEventModifierFlags::Command);
    unsafe { quit_item.setTarget(Some(&*app)) };
    app_menu.addItem(&quit_item);

    app_menu_item.setSubmenu(Some(&app_menu));
    main_menu.addItem(&app_menu_item);

    // Create Window menu
    let window_menu_item = NSMenuItem::new(mtm);
    let window_menu_title = NSString::from_str("Window");
    window_menu_item.setTitle(&window_menu_title);

    let window_menu = NSMenu::new(mtm);
    window_menu.setTitle(&window_menu_title);

    // Add Close Window menu item with Command+W shortcut
    let close_title = NSString::from_str("Close Window");
    let close_key = NSString::from_str("w");
    let close_item: Retained<NSMenuItem> = msg_send![NSMenuItem::alloc(mtm),
        initWithTitle: &*close_title,
        action: objc2::sel!(performClose:),
        keyEquivalent: &*close_key,
    ];
    close_item.setKeyEquivalentModifierMask(NSEventModifierFlags::Command);
    window_menu.addItem(&close_item);

    window_menu_item.setSubmenu(Some(&window_menu));
    main_menu.addItem(&window_menu_item);

    // Set the main menu
    app.setMainMenu(Some(&main_menu));
}

// Window component
#[component]
pub async fn window(
    #[default = "Window"] title: &str,
    #[default = 800] width: i32,
    #[default = 600] height: i32,
    #[default = 100] left: i32,
    #[default = 100] top: i32,
    #[default = true] visible: bool,
    #[default = true] enabled: bool,
) {
    #[field]
    // This is a field of the component's internal structure, not a variable in the current scope, so it can persist across multiple renders
    let window: Option<Retained<NSWindow>> = None;

    let Some(mtm) = MainThreadMarker::new() else {
        error!("Window component must be running on main thread.");
        return;
    };

    if *visible {
        // Initialize NSApplication if needed
        let app = NSApplication::sharedApplication(mtm);

        // Setup application menu (only once)
        MENU_SETUP.call_once(|| unsafe {
            app.setActivationPolicy(NSApplicationActivationPolicy::Regular);
            setup_app_menu(&*app, mtm);
        });
        // Activate application and bring to front
        app.activate();

        if window.is_none() {
            // Create new window
            unsafe {
                // Create window frame
                let frame = NSRect::new(
                    NSPoint::new(*left as f64, *top as f64),
                    NSSize::new(*width as f64, *height as f64),
                );

                // Create window with style mask
                let style_mask = NSWindowStyleMask::Titled
                    | NSWindowStyleMask::Closable
                    | NSWindowStyleMask::Miniaturizable
                    | NSWindowStyleMask::Resizable;

                // Create window using alloc and init pattern
                let window_instance: Retained<NSWindow> = msg_send![NSWindow::alloc(mtm),
                    initWithContentRect: frame,
                    styleMask: style_mask,
                    backing: NSBackingStoreType::Buffered,
                    defer: false
                ];

                window.replace(window_instance);
            }
        }

        // Show window and update parameters
        if let Some(window_ref) = window.as_ref() {
            unsafe {
                // Set window title (supports reactive updates)
                let ns_title = NSString::from_str(title);
                window_ref.setTitle(&ns_title);

                // Update window frame (position and size) (supports reactive updates)
                let frame = NSRect::new(
                    NSPoint::new(*left as f64, *top as f64),
                    NSSize::new(*width as f64, *height as f64),
                );
                let _: () = msg_send![&**window_ref, setFrame: frame, display: true];

                // Set window enabled state (supports reactive updates)
                window_ref.setIgnoresMouseEvents(!*enabled);

                // Make window visible
                window_ref.makeKeyAndOrderFront(None);

                // Get window content size for logging
                let content_rect: NSRect = msg_send![&**window_ref, contentRectForFrameRect: frame];
                info!(
                    "Window updated with content area: {}x{}",
                    content_rect.size.width as i32, content_rect.size.height as i32
                );
            }
        } else {
            error!("Failed to create window.");
        }
    } else if let Some(window_ref) = window.take() {
        // If visible is false and window exists, close the window
        unsafe {
            let _: () = msg_send![&*window_ref, close];
        }
    }
}
