use windows::core::Result;
use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::Input::Ime::{
    ImmGetDefaultIMEWnd, IMC_SETOPENSTATUS, IMN_OPENSTATUSWINDOW,
};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, SendMessageA, WM_IME_CONTROL};

fn main() -> Result<()> {
    unsafe {
        let hwnd = GetForegroundWindow();
        let ime = ImmGetDefaultIMEWnd(hwnd);

        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            SendMessageA(
                ime,
                WM_IME_CONTROL,
                WPARAM(IMN_OPENSTATUSWINDOW.try_into()?),
                LPARAM(0),
            );
        } else {
            let mode = args[1].parse().unwrap_or_default();
            SendMessageA(
                ime,
                WM_IME_CONTROL,
                WPARAM(IMC_SETOPENSTATUS.try_into()?),
                LPARAM(mode),
            );
        }
    }
    Ok(())
}

