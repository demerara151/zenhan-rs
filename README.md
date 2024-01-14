# zenhan-rs

Rust implementation of [zenhan](https://github.com/iuchim/zenhan). Switch the mode of IME from terminal.

## Install

Download the binary from [releases](https://github.com/demerara151/zenhan-rs/releases) page.

Then save it to your desired location and set the path to the executable file.

## Usage

- Turn off the IME

Switch the mode to Alphanumerical (hankaku-eisu).

```powershell
# run it without options
zenhan-rs

# or more explicitly
zenhan-rs 0
```

- Turn on the IME

Switch the mode to Japanese (hiragana).

```powershell
zenhan-rs 1
```

> Unsigned integer larger than 1 is passed, it will be treated as 1. For example, `zenhan-rs 42` will turn on the IME.

## Changes

- Binary: Only 64-bit binary is supported.
- Size: The size is approximately 3-4 times larger than the original, but it still remains small.
- Output: No results will be output, such as 0 or 1.
- Safety: Rust ensures memory safety, but does not take responsibility for any issues related to 'winapi'.

## Settings

Setting examples for various editors and shells to auto IME off when you leave INSERT mode.

These examples are compatible with the original zenhan, and you can replace them with zenhan if you want.

### Neovim

In `nvim/init.lua`:

```lua
-- Auto IME off when you leave INSERT mode
if vim.fn.executable('zenhan-rs') then
  vim.cmd([[autocmd InsertLeave * call system('zenhan-rs 0')]])
  vim.cmd([[autocmd CmdlineLeave * call system('zenhan-rs 0')]])
end
```

### Visual Studio Code

Same as [Neovim](###Neovim) if you are using `VSCode Neovim` extension in VSCode.

If you want to use this feature only works in VSCode, then write them in this vscode scope.

```lua
if g.vscode then
  -- Auto IME off when you leave INSERT mode
  if vim.fn.executable('zenhan-rs') then
    vim.cmd([[autocmd InsertLeave * call system('zenhan-rs 0')]])
    vim.cmd([[autocmd CmdlineLeave * call system('zenhan-rs 0')]])
  end
end
```

### Helix

Configure your `helix/config.toml` as follows:

```toml
# Nushell is significantly faster than PowerShell.
[editor]
shell = ["nu", "-c"]  # or ["pwsh", "-nop", "-c"] but too slow to switch the mode

# Auto IME off when you leave INSERT mode.
[keys.insert]
"esc" = ["normal_mode", ":run-shell-command zenhan-rs 0"]
```

> ⚠️ Caution: Do not forget to add "normal_mode" as first argument. Otherwise, you will never get back to NORMAL mode.

### PowerShell

If you are set `$PSReadLineOptions.EditMode` to `Vi`, write the following script in `$PROFILE`.

```PowerShell
# Set the style of Vi mode indicator and turn off the IME.
function OnViModeChange {
  if ($args[0] -eq 'Command') {
    # Turn off the IME when you leave the INSERT mode.
    zenhan-rs 0
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

### Nushell

> 💡Does not work as intended. Still work in progress.

```nu
$env.config = {
    ...

    keybindings: [
      {
        name: escape
        modifier: none
        keycode: escape
        mode: vi_insert
        event: { send: executehostcommand, cmd: "zenhan-rs 0" }
      }
    ]

    ...
  }
```

## NOTES

- This program only works on Windows system but it might work on other systems as well.
- If you pass an invalid option that can't be parsed as an integer, it will default to 0.
- There's no need to replace original `zenhan` with `zenhan-rs` if you don't have any issues.

## References

- [iuchim/zenhan](https://github.com/iuchim/zenhan): Original program. Thanks for creating useful app.

## LICENSE

This project is licensed under the terms of the [MIT license](./LICENSE).
