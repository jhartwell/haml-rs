pub const WHITESPACE: &str = r"\s*";
pub const STRING: &str = r"\w+";

pub const TEXT_REGEX: &str = r"^(\s*)[\\]";

fn element_name() -> String {
    format!("[%]{{1}}{}", STRING)
}

fn element_class_id() -> String {
    format!("[#|.]{{1}}\\w+")
}

fn element_text() -> String {
    r"\s+.+".to_owned()
}

pub fn element() -> String {
    format!(
        "^(?P<ws>{})*(?P<name>{}){{1}}(?P<classid>({})*)(?P<attributes>({}){{0,1}})(?P<text>{})*",
        WHITESPACE,
        element_name(),
        element_class_id(),
        attributes(),
        element_text()
    )
}

pub fn div() -> String {
    format!(
        "(?P<ws>{})*(?P<name>{}){{1}}(?P<classid>({})*)(?P<attributes>({}){{0,1}})(?P<text>{})*",
        WHITESPACE,
        element_class_id(),
        element_class_id(),
        attributes(),
        element_text()
    )
}

fn attributes() -> String {
    //"[{{]((\\s*[:]\\w+){1}\\s*[=]\\s*[']\\w*[']\\s*)+[}}]".to_owned()
    r#"[{]{1}[^}]*[}]{1}"#.to_owned()
}




pub fn break_attributes() -> String {
    r#"([:]{1}([^\s]+)\s*(=>){1}\s*["]{1}([^"]*)["]{1})*"#.to_owned()
}