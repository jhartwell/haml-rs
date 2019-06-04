pub const WHITESPACE: &str = r"\s*";
pub const STRING: &str = r"\w+";

pub const TEXT_REGEX: &str = r"^(\s*)(?P<text>.+)";

pub const COMMENT_REGEX: &str = r"\s*/(?P<comment>.*)";

fn element_name() -> String {
    r"[%]{1}[\w|:|\-|_]+".to_owned()
}

pub fn element_class_id() -> String {
    r"[#|.]{1}[\w|\-|_]+".to_owned()
}

fn element_text() -> String {
    r"\s+.+".to_owned()
}

fn self_close() -> String {
    r"[/]".to_owned()
}

pub fn silent_comment() -> String {
    r"^(?P<ws>\s*)(-#)".to_owned()
}

pub fn element() -> String {
    format!(
        "^(?P<ws>{})*(?P<name>{}){{1}}(?P<classid>({})*)(?P<ruby_attributes>({}){{0,1}})(?P<html_attributes>({}){{0,1}})(?P<self_close>{}{{0,1}})(?P<text>{})*",
        WHITESPACE,
        element_name(),
        element_class_id(),
        ruby_attributes(),
        html_attributes(),
        self_close(),
        element_text(),
    )
}

pub fn prolog() -> String {
    r"^\s*!!!\s*(?P<type>([\w|.|\d]*))".to_owned()
}

pub fn sanitize() -> String {
    r"^(\s*)(&=)\s*[']{1}(?P<text>([^'|^\n]*))".to_owned()
}

pub fn div() -> String {
    format!(
        "(?P<ws>{})*(?P<name>{}){{1}}(?P<classid>({})*)(?P<ruby_attributes>({}){{0,1}})(?P<html_attributes>({}){{0,1}})(?P<self_close>{}{{0,1}})(?P<text>{})*",
        WHITESPACE,
        element_class_id(),
        element_class_id(),
        ruby_attributes(),
        html_attributes(),
        self_close(),
        element_text(),
    )
}

fn ruby_attributes() -> String {
    //"[{{]((\\s*[:]\\w+){1}\\s*[=]\\s*[']\\w*[']\\s*)+[}}]".to_owned()
    r#"[{]{1}[^}]*[}]{1}"#.to_owned()
}

fn html_attributes() -> String {
    r"[(]{1}[^)]+[)]{1}".to_owned()
    //r"\([\w:]*\s*[=]\s*[\w]*\)".to_owned()
}

pub fn ruby_attribute() -> String {
    r#"([:]{1}([^\s]+)\s*(=>){1}\s*["]{1}([^"]*)["]{1})*"#.to_owned()
}

pub fn html_attribute() -> String {
    r#"(([^\s|^(]+)\s*(=){1}\s*[']{0,1}([^'|^)]*)[']{0,1})"#.to_owned()
}

pub fn class() -> String {
    r"([.]{1}[^.|^#]+)*".to_owned()
}

pub fn id() -> String {
    r"([#]{1}[^.|^#]+)*".to_owned()
}
