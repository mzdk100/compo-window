#![cfg(target_os = "ios")]

include!("../../hello.rs");

use {
    objc2_foundation::NSAutoreleasePool,
    std::os::raw::c_int,
    tracing::{info, subscriber::set_global_default},
    tracing_oslog::OsLogger,
    tracing_subscriber::{layer::SubscriberExt, registry},
};

#[unsafe(no_mangle)]
pub extern "C" fn run_app() -> c_int {
    // 初始化日志（iOS平台）
    let collector = registry().with(OsLogger::new("cn.sljtkj.yl", "default"));
    set_global_default(collector).expect("failed to set global subscriber");

    info!("Starting iOS application");

    // 创建自动释放池
    let _pool = unsafe { NSAutoreleasePool::new() };

    // 运行应用
    main();

    0 // 返回成功状态
}
