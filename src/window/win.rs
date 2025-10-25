use {
    compo::prelude::*,
    tracing::{error, info},
    windows::{
        Win32::{
            Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM},
            Graphics::Gdi::HBRUSH,
            System::LibraryLoader::GetModuleHandleW,
            UI::{
                Input::KeyboardAndMouse::EnableWindow,
                WindowsAndMessaging::{
                    CREATESTRUCTW, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, CreateWindowExW,
                    DefWindowProcW, DestroyWindow, GWLP_USERDATA, GetClientRect, HCURSOR, HICON,
                    PostQuitMessage, RegisterClassW, SW_SHOW, SWP_NOZORDER, SetWindowLongPtrW,
                    SetWindowPos, SetWindowTextW, ShowWindow, WM_CREATE, WM_DESTROY, WNDCLASSW,
                    WS_EX_LEFT, WS_OVERLAPPEDWINDOW,
                },
            },
        },
        core::{PCWSTR, w},
    },
};

// Window procedure callback function
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            let create_struct = lparam.0 as *const CREATESTRUCTW;
            let window_ptr = unsafe { (*create_struct).lpCreateParams };
            unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, window_ptr as isize) };
            LRESULT::default()
        }
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            LRESULT::default()
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

// Window component
#[component]
pub async fn window(
    #[default = "Window"] title: &str,
    #[default = 800] width: i32,
    #[default = 600] height: i32,
    #[default = CW_USEDEFAULT] left: i32,
    #[default = CW_USEDEFAULT] top: i32,
    #[default = true] visible: bool,
    #[default = true] enabled: bool,
) {
    #[field]
    // This is a field of the component's internal structure, not a variable in the current scope, so it can persist across multiple renders
    let hwnd: Option<HWND> = None;

    if *visible {
        if hwnd.is_none() {
            // Register window class
            let h_instance = match unsafe { GetModuleHandleW(PCWSTR::null()) } {
                Err(e) => {
                    error!(?e, "Can't get module handle.");
                    return;
                }
                Ok(h) => h.into(),
            };

            let class_name = w!("CompoWindow");
            let wc = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: h_instance,
                hIcon: HICON::default(),
                hCursor: HCURSOR::default(),
                hbrBackground: HBRUSH::default(),
                lpszMenuName: PCWSTR::null(),
                lpszClassName: class_name,
            };

            unsafe {
                RegisterClassW(&wc);

                // Create window
                // Convert string to UTF-16 and ensure it ends with null
                let mut window_title: Vec<u16> = title.encode_utf16().collect();
                window_title.push(0); // Add null terminator
                let hwnd_value = match CreateWindowExW(
                    WS_EX_LEFT,
                    class_name,
                    PCWSTR(window_title.as_ptr()),
                    WS_OVERLAPPEDWINDOW,
                    *left,
                    *top,
                    *width,
                    *height,
                    None,
                    None,
                    Some(h_instance),
                    None,
                ) {
                    Ok(h) => h,
                    Err(e) => {
                        error!(?e, "Can't create window.");
                        return;
                    }
                };

                hwnd.replace(hwnd_value);
            }
        }

        // Show window and update parameters
        if let Some(hwnd) = hwnd {
            let _ = unsafe { ShowWindow(*hwnd, SW_SHOW) };

            // Update window title (supports reactive updates)
            let mut window_title: Vec<u16> = title.encode_utf16().collect();
            window_title.push(0); // Add null terminator
            let _ = unsafe { SetWindowTextW(*hwnd, PCWSTR(window_title.as_ptr())) };

            // Update window position and size (supports reactive updates)
            let _ = unsafe {
                SetWindowPos(
                    *hwnd,
                    None,
                    *left,
                    *top,
                    *width,
                    *height,
                    SWP_NOZORDER, // Don't change Z-order
                )
            };

            // Update window enabled state (supports reactive updates)
            let _ = unsafe { EnableWindow(*hwnd, *enabled) };

            // Get client area size
            let mut rect = RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            let _ = unsafe { GetClientRect(*hwnd, &mut rect) };

            info!(
                "Window updated with client area: {}x{}",
                rect.right - rect.left,
                rect.bottom - rect.top
            );
        } else {
            error!("Failed to create window.");
        }
    } else if let Some(hwnd) = hwnd.take() {
        // If shown is false and window exists, destroy the window
        let _ = unsafe { DestroyWindow(hwnd) };
    }
}
