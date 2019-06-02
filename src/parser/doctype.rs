use crate::regex::prolog;
use crate::Format;

pub struct Doctype<'a> {
    value: Option<&'a str>,
    format: &'a Format,
}

impl<'a> Doctype<'a> {
    pub fn new(format: &'a Format, value: Option<&'a str>) -> Doctype<'a> {
        Doctype { format, value }
    }

    pub fn to_html(&self) -> String {
        match self.format {
            Format::XHtml() => self.xhtml_options().to_owned(),
            Format::Html4() => self.html4_options().to_owned(),
            Format::Html5() => String::new(),
            Format::Xml() => String::new(),
        }
    }

    fn html4_options(&self) -> &str {
        if let Some(value) = &self.value {
            match value.to_lowercase().as_str() {
            "frameset" => r#"<!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01 Frameset//EN" "http://www.w3.org/TR/html4/frameset.dtd">"#,
            "strict" => r#"<!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">"#,
            _ => r#"<!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">"#,
            }
        } else {
            ""
        }
    }

    fn xhtml_options(&self) -> &str {
        if let Some(value) = &self.value {
            match value.to_lowercase().as_str() {
            "strict" => r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Strict//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd">"#,
            "frameset" => r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Frameset//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-frameset.dtd">"#,
            "5" => "<!DOCTYPE html>",
            "1.1" => r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN" "http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd">"#,
            "basic" => r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML Basic 1.1//EN" "http://www.w3.org/TR/xhtml-basic/xhtml-basic11.dtd">"#,
            "mobile" => r#"<!DOCTYPE html PUBLIC "-//WAPFORUM//DTD XHTML Mobile 1.2//EN" "http://www.openmobilealliance.org/tech/DTD/xhtml-mobile12.dtd">"#,
            "rdfa" => r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML+RDFa 1.0//EN" "http://www.w3.org/MarkUp/DTD/xhtml-rdfa-1.dtd">"#,
            _ => ""
        }
        } else {
            ""
        }
    }
}
