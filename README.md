# zenhan-rs

Rust implementation of [zenhan](https://github.com/iuchim/zenhan). Switch the mode of IME from terminal.

## Install

Install the binary from release page.

## Usage

- Turn IME off

Switch the mode to Alphanumerical (hankaku-eisu).

```powershell
# run it without options
zenhan-rs.exe

# or more explicitly
zenhan-rs.exe 0
```

- Turn IME on

Switch the mode to Japanese (hiragana).

```powershell
zenhan-rs.exe 1
```

## Settings

Setting examples for various editors and shells.

### Neovim

In `nvim/init.lua`:

```lua
-- Auto IME off when you leave INSERT mode
if vim.fn.executable('zenhan-rs') then
  vim.cmd([[autocmd InsertLeave * call system('zenhan-rs 0')]])
  vim.cmd([[autocmd CmdlineLeave * call system('zenhan-rs 0')]])
end
```

### VSCode Neovim

Same as Neovim.

### Helix

Configure your `helix/config.toml` as follows:

```toml
# Auto IME off when you leave INSERT mode.

# Nushell is significantly faster than PowerShell.
[editor]
shell = ["nu", "-c"]  # or ["pwsh", "-nop", "-c"] but too slow to switch the mode

[keys.insert]
"esc" = ["normal_mode", ":run-shell-command zenhan-rs.exe 0"]
```

> ⚠️ Caution: Do not forget to add "normal_mode" as first argument. Otherwise, you will never get back to NORMAL mode.

### PowerShell

If you are set `$PSReadLineOptions.EditMode` to `Vi`, write the following script in `$PROFILE`.

```PowerShell
# Set the Vi mode indicator and turn off the IME.
function OnViModeChange {
  if ($args[0] -eq 'Command') {
    # Turn off the IME when you leave the INSERT mode.
    zenhan-rs.exe 0
    # Set the cursor to a blinking block.
    Write-Host -NoNewline "`e[1 q"
  }
  else {
    # Set the cursor to a blinking line.
    Write-Host -NoNewline "`e[5 q"
  }
}
Set-PSReadLineOption -ViModeIndicator Script -ViModeChangeHandler $Function:OnViModeChange
```

> Cursor styling is optional.

## NOTES

- This program only works on Windows system.
- If an invalid option other than 0 or 1 is passed, it will be treated as 0 by default.

## LICENSE

This project is licensed under the terms of the [MIT license](./LICENSE).
