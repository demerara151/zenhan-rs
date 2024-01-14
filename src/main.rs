use windows::core::Result;
use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::Input::Ime::{ImmGetDefaultIMEWnd, IMC_SETOPENSTATUS};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, SendMessageA, WM_IME_CONTROL};

fn main() -> Result<()> {
    let set_open_status = WPARAM(IMC_SETOPENSTATUS.try_into()?);

    let mode = get_input_mode();

    unsafe {
        let hwnd = GetForegroundWindow();
        let ime = ImmGetDefaultIMEWnd(hwnd);

        SendMessageA(ime, WM_IME_CONTROL, set_open_status, mode);
    }
    Ok(())
}

fn get_input_mode() -> LPARAM {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|arg| arg.parse().unwrap_or_default()) {
        Some(mode) => LPARAM(mode),
        None => LPARAM(0),
    }
}

