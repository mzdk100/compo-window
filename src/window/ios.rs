use {
    compo::prelude::*,
    objc2::{ClassType, MainThreadMarker, MainThreadOnly, Message, msg_send, rc::Retained},
    objc2_foundation::{NSObjectProtocol, NSPoint, NSRect, NSSize},
    objc2_ui_kit::{UIApplication, UIColor, UIScene, UIViewController, UIWindow, UIWindowScene},
    tracing::{error, info},
};

// Window component for iOS
#[component]
pub async fn window(
    #[default = "Window example"] title: &str,
    #[default = 375] width: i32,
    #[default = 667] height: i32,
    #[default = 100] left: i32,
    #[default = 100] top: i32,
    #[default = true] visible: bool,
    #[default = true] enabled: bool,
) {
    // Get main thread marker
    let Some(mtm) = MainThreadMarker::new() else {
        error!("Window component must be running on main thread.");
        return;
    };

    // This is a field of the component's internal structure, not a variable in the current scope, so it can persist across multiple renders
    #[field]
    let window: Option<Retained<UIWindow>> = None;
    #[field]
    let view_controller: Option<Retained<UIViewController>> = None;

    if *visible {
        if window.is_none() {
            // Create new window
            unsafe {
                // Create window frame (iOS uses different coordinate system)
                let frame = NSRect {
                    origin: NSPoint {
                        x: *left as f64,
                        y: *top as f64,
                    },
                    size: NSSize {
                        width: *width as f64,
                        height: *height as f64,
                    },
                };

                // Create window
                let window_instance: Retained<UIWindow>;

                // Try to create window with scene (iOS 13+)
                if let Some(scene) = get_current_window_scene(mtm) {
                    window_instance = msg_send![UIWindow::alloc(mtm), initWithWindowScene: &*scene];
                } else {
                    // Fallback for older iOS versions
                    window_instance = msg_send![UIWindow::alloc(mtm), initWithFrame: frame];
                }

                // Create root view controller
                let root_controller = UIViewController::new(mtm);

                // Set background color
                if let Some(view) = root_controller.view() {
                    let white_color = UIColor::whiteColor();
                    view.setBackgroundColor(Some(&white_color));
                } else {
                    error!("Can't get the view in root controller.");
                }

                // Set root view controller
                window_instance.setRootViewController(Some(&root_controller));

                // Configure window
                window_instance.setFrame(frame);

                // Store references
                window.replace(window_instance);
                view_controller.replace(root_controller);
            }
        }

        // Update window properties
        if let Some(window_ref) = window.as_ref() {
            unsafe {
                // Update window frame (position and size)
                let frame = NSRect {
                    origin: NSPoint {
                        x: *left as f64,
                        y: *top as f64,
                    },
                    size: NSSize {
                        width: *width as f64,
                        height: *height as f64,
                    },
                };
                window_ref.setFrame(frame);

                // Set window enabled state
                window_ref.setUserInteractionEnabled(*enabled);

                // Make window visible and key
                window_ref.makeKeyAndVisible();

                info!(
                    "iOS window updated: {}x{} at ({}, {}), enabled: {}, visible: {}",
                    *width, *height, *left, *top, *enabled, *visible
                );
            }
        } else {
            error!("Failed to create iOS window.");
        }
    } else if let Some(window_ref) = window.take() {
        // Hide window when visible is false
        unsafe {
            window_ref.setHidden(true);
            // Also clear view controller reference
            view_controller.take();
        }
        info!("iOS window hidden");
    }
}

// Helper function to get current window scene (iOS 13+)
unsafe fn get_current_window_scene(mtm: MainThreadMarker) -> Option<Retained<UIScene>> {
    let app = UIApplication::sharedApplication(mtm);
    let connected_scenes = app.connectedScenes();

    // Try to get the first window scene
    let scene_enumerator = unsafe { connected_scenes.objectEnumerator() };
    while let Some(scene) = scene_enumerator.nextObject() {
        // Check if it's a UIWindowScene
        if scene.isKindOfClass(&UIWindowScene::class()) {
            return Some(scene.retain());
        }
    }

    None
}
