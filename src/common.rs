#[cfg(target_os = "windows")]
pub fn newline() -> &'static str {
    "\r\n"
}

#[cfg(not(target_os = "windows"))]
pub fn newline() -> &'static str {
    "\n"
}

/// Maintain a mapping of tags that do
/// not require a closing tag
pub fn does_tag_close(tag: &str) -> bool {
    match tag {
        "meta" => false,
        "link" => false,
        _ => true,
    }
}
