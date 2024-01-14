use windows::core::Result;
use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::UI::Input::Ime::{ImmGetDefaultIMEWnd, IMC_SETOPENSTATUS};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, SendMessageW, WM_IME_CONTROL};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mode = get_input_mode(args);
    let set_open_status = WPARAM(IMC_SETOPENSTATUS.try_into()?);

    unsafe {
        let hwnd = GetForegroundWindow();
        let ime = ImmGetDefaultIMEWnd(hwnd);

        SendMessageW(ime, WM_IME_CONTROL, set_open_status, mode);
    }
    Ok(())
}

fn get_input_mode(args: Vec<String>) -> LPARAM {
    match args.get(1).map(|arg| arg.parse().unwrap_or_default()) {
        Some(mode) => LPARAM(mode),
        None => LPARAM(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input_mode_with_argument() {
        let args = vec!["path/to/executable".to_string(), "1".to_string()];

        let mode = get_input_mode(args);
        assert_eq!(mode.0, 1);
    }

    #[test]
    fn test_get_input_mode_with_invalid_argument() {
        let args = vec!["path/to/executable".to_string(), "日本語".to_string()];

        let mode = get_input_mode(args);
        assert_eq!(mode.0, 0);
    }

    #[test]
    fn test_get_input_mode_without_argument() {
        let args = vec!["path/to/executable".to_string()];

        let mode = get_input_mode(args);
        assert_eq!(mode.0, 0);
    }
}

