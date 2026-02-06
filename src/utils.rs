pub fn paste_ready_code(file: &str) -> Result<String, String> {
    let mut src =
        std::fs::read_to_string(file).map_err(|e| format!("Failed to read {}: {e}", file))?;

    if let (Some(i0), Some(i1)) = (src.find("// LOCAL"), src.find("// LOCAL END")) {
        src.replace_range(i0..i1 + 12, "");
    }

    let trim_count = src.len() - src.trim_start().len();
    if trim_count > 0 {
        src.drain(..trim_count);
    }

    Ok(src)
}

pub fn copy_to_clipboard(text: &str) {
    #[cfg(target_os = "linux")]
    {
        use std::env;
        use std::io::Write;
        use std::process::{Command, Stdio};

        fn try_copy(cmd: &str, args: &[&str], text: &str) -> bool {
            let process = Command::new(cmd)
                .args(args)
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();

            if let Ok(mut child) = process
                && let Some(mut stdin) = child.stdin.take()
                && stdin.write_all(text.as_bytes()).is_ok()
            {
                drop(stdin);
                return child.wait().map(|s| s.success()).unwrap_or(false);
            }
            false
        }

        let session_type = env::var("XDG_SESSION_TYPE")
            .unwrap_or_default()
            .to_lowercase();

        if session_type == "wayland" && try_copy("wl-copy", &[], text) {
            return;
        }

        if try_copy("xclip", &["-selection", "clipboard"], text) {
            return;
        }
        if try_copy("xsel", &["--clipboard", "--input"], text) {
            return;
        }

        println!(
            "Warning: No clipboard utilities found. Falling back to arboard (may not persist)."
        );
    }

    let mut ctx = arboard::Clipboard::new().expect("Failed to initialize clipboard");
    #[cfg(target_os = "linux")]
    {
        use arboard::SetExtLinux;
        let _ = ctx.set().wait().text(text);
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = ctx.set_text(text);
    }
}
