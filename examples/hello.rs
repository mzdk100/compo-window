use {compo_window::prelude::*, std::env::set_var};

#[component]
async fn app() {
    let mut title = "hello";
    let mut width = 800;
    let mut height = 600;
    let mut enabled = false;

    #[render]
    window {
        title,
        width,
        height,
        enabled,
    };

    // Wait 1-second then update the title
    sleep(Duration::from_secs(1)).await;
    title = "你好";

    // Wait another second then update the window size
    sleep(Duration::from_secs(1)).await;
    width = 1000;
    height = 800;
    enabled = true;
}

fn main(#[cfg(target_os = "android")] vm: jni::JavaVM) {
    unsafe { set_var("RUST_BACKTRACE", "1") };
    #[cfg(target_os = "android")]
    run(vm, app);
    #[cfg(not(target_os = "android"))]
    run(app);
}
