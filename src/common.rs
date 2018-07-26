#[cfg(target_os = "windows")]
pub fn newline() -> &'static str {
    "\r\n"
}

#[cfg(not(target_os = "windows"))]
pub fn newline() -> &'static str {
    "\n"
}
