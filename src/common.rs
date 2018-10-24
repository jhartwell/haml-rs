/// Maintain a mapping of tags that do
/// not require a closing tag
pub fn does_tag_close(tag: &str) -> bool {
    match tag {
        "meta" => false,
        "link" => false,
        _ => true,
    }
}
