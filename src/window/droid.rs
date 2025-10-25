#![allow(unused)]

use {
    compo::prelude::*,
    compo_platform_loop::prelude::vm_exec,
    jni::{
        JNIEnv,
        errors::Result as JniResult,
        objects::{GlobalRef, JObject},
    },
    std::cell::Cell,
    tracing::{error, info},
};

thread_local! {
    static ACTIVITY: Cell<Option<GlobalRef>> = Cell::new(None);
    static ACTIVITY_REQUEST_RENDERING: EventListener<'static, ()> = EventListener::default();
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
unsafe extern "system" fn Java_rust_compo_CompoActivity_on_1created(env: JNIEnv, this: JObject) {
    ACTIVITY.set(env.new_global_ref(this).ok());
    ACTIVITY_REQUEST_RENDERING.with(|i| i.new_emitter().emit(()));
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
unsafe extern "system" fn Java_rust_compo_CompoActivity_on_1destroyed(env: JNIEnv, this: JObject) {
    ACTIVITY.set(None);
    ACTIVITY_REQUEST_RENDERING.with(|i| i.new_emitter().emit(()));
}

async fn get_activity() -> GlobalRef {
    match ACTIVITY.with(|i| unsafe { transmute::<_, &mut Option<GlobalRef>>(i.as_ptr()) }.clone()) {
        Some(a) if !a.is_null() => a,
        _ => {
            ACTIVITY_REQUEST_RENDERING.with(|i| i.listen()).await;
            Box::pin(get_activity()).await
        }
    }
}

// Window component for Android
#[component]
pub async fn window(
    #[default = "Window"] title: &str,
    #[default = 360] width: i32,
    #[default = 640] height: i32,
    #[default = true] visible: bool,
    #[default = true] enabled: bool,
) {
    let activity_obj = get_activity().await;
    let this2 = this.clone();
    this.spawn(async move {
        ACTIVITY_REQUEST_RENDERING.with(|i| i.listen()).await;
        this2.update();
    });

    // This is a field of the component's internal structure, not a variable in the current scope, so it can persist across multiple renders
    #[field]
    let window_initialized: bool = false;

    if *visible {
        if !*window_initialized {
            // Initialize window for the first time
            if let Err(e) = setup_android_window(&activity_obj, title, *width, *height, *enabled) {
                error!("Failed to setup Android window: {:?}", e);
                return;
            }

            *window_initialized = true;

            info!(
                "Android window initialized: {} ({}x{})",
                title, width, height
            );
        } else {
            // Update window properties if they changed
            let mut needs_update = false;

            if let Err(e) = update_window_title(&activity_obj, title) {
                error!("Failed to update window title: {:?}", e);
            } else {
                needs_update = true;
            }

            if let Err(e) = update_window_size(&activity_obj, *width, *height) {
                error!("Failed to update window size: {:?}", e);
            } else {
                needs_update = true;
            }

            // Update enabled state
            if let Err(e) = update_window_enabled(&activity_obj, *enabled) {
                error!("Failed to update window enabled state: {:?}", e);
            }

            if needs_update {
                info!(
                    "Android window updated: {} ({}x{}), enabled: {}",
                    title, width, height, enabled
                );
            }
        }
    } else if *window_initialized {
        // Hide window when visible is false
        if let Err(e) = hide_android_window(&activity_obj) {
            error!("Failed to hide Android window: {:?}", e);
        } else {
            info!("Android window hidden");
        }
    }
}

//noinspection SpellCheckingInspection
/// Setup Android window (Activity configuration)
fn setup_android_window(
    activity: &JObject,
    title: &str,
    width: i32,
    height: i32,
    enabled: bool,
) -> JniResult<()> {
    vm_exec(|mut env| {
        // Set activity title
        let title = env.new_string(title)?;
        env.call_method(
            activity,
            "setTitle",
            "(Ljava/lang/CharSequence;)V",
            &[(&title).into()],
        )?;

        // Get window from activity
        let window = env.call_method(activity, "getWindow", "()Landroid/view/Window;", &[])?;
        let window_obj = window.l()?;

        // Set window flags and attributes
        if enabled {
            // Clear FLAG_NOT_TOUCHABLE to enable touch
            env.call_method(
                &window_obj,
                "clearFlags",
                "(I)V",
                &[0x00000010.into()], // FLAG_NOT_TOUCHABLE
            )?;
        } else {
            // Set FLAG_NOT_TOUCHABLE to disable touch
            env.call_method(
                &window_obj,
                "addFlags",
                "(I)V",
                &[0x00000010.into()], // FLAG_NOT_TOUCHABLE
            )?;
        }

        // Get window attributes
        let attributes = env.call_method(
            &window_obj,
            "getAttributes",
            "()Landroid/view/WindowManager$LayoutParams;",
            &[],
        )?;
        let attributes_obj = attributes.l()?;

        // Set window size (if supported)
        env.set_field(&attributes_obj, "width", "I", width.into())?;
        env.set_field(&attributes_obj, "height", "I", height.into())?;

        // Apply the attributes
        env.call_method(
            &window_obj,
            "setAttributes",
            "(Landroid/view/WindowManager$LayoutParams;)V",
            &[(&attributes_obj).into()],
        )?;

        Ok(())
    })
}

//noinspection SpellCheckingInspection
// Update window title
fn update_window_title(activity: &JObject, title: &str) -> JniResult<()> {
    vm_exec(|mut env| {
        let title = env.new_string(title)?;
        env.call_method(
            activity,
            "setTitle",
            "(Ljava/lang/CharSequence;)V",
            &[(&title).into()],
        )?;

        Ok(())
    })
}

//noinspection SpellCheckingInspection
// Update window size
fn update_window_size(activity: &JObject, width: i32, height: i32) -> JniResult<()> {
    vm_exec(|mut env| {
        // Get window from activity
        let window = env.call_method(activity, "getWindow", "()Landroid/view/Window;", &[])?;
        let window_obj = window.l()?;

        // Get window attributes
        let attributes = env.call_method(
            &window_obj,
            "getAttributes",
            "()Landroid/view/WindowManager$LayoutParams;",
            &[],
        )?;
        let attributes_obj = attributes.l()?;

        // Update size
        env.set_field(&attributes_obj, "width", "I", width.into())?;
        env.set_field(&attributes_obj, "height", "I", height.into())?;

        // Apply the attributes
        env.call_method(
            &window_obj,
            "setAttributes",
            "(Landroid/view/WindowManager$LayoutParams;)V",
            &[(&attributes_obj).into()],
        )?;

        Ok(())
    })
}

//noinspection SpellCheckingInspection
// Update window enabled state
fn update_window_enabled(activity: &JObject, enabled: bool) -> JniResult<()> {
    vm_exec(|mut env| {
        // Get window from activity
        let window = env.call_method(activity, "getWindow", "()Landroid/view/Window;", &[])?;
        let window_obj = window.l()?;

        if enabled {
            // Clear FLAG_NOT_TOUCHABLE to enable touch
            env.call_method(
                &window_obj,
                "clearFlags",
                "(I)V",
                &[0x00000010.into()], // FLAG_NOT_TOUCHABLE
            )?;
        } else {
            // Set FLAG_NOT_TOUCHABLE to disable touch
            env.call_method(
                &window_obj,
                "addFlags",
                "(I)V",
                &[0x00000010.into()], // FLAG_NOT_TOUCHABLE
            )?;
        }

        Ok(())
    })
}

//noinspection SpellCheckingInspection
// Hide Android window
fn hide_android_window(activity: &JObject) -> JniResult<()> {
    vm_exec(|mut env| {
        // Move activity to background (minimize)
        env.call_method(activity, "moveTaskToBack", "(Z)Z", &[true.into()])?;

        Ok(())
    })
}
