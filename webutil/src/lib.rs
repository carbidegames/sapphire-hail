/// A formatted HTML string.
pub struct HtmlString {
    pub raw: String,
}

impl HtmlString {
    /// Blesses a regular string to be safe HTML. Do not give this any data you do not trust.
    pub fn bless<S: ToString>(raw: S) -> Self {
        HtmlString {
            raw: raw.to_string()
        }
    }

    /// Escapes a string into HTML.
    pub fn escape<S: ToString>(string: S) -> Self {
        let mut raw = String::new();

        for c in string.to_string().chars() {
            match c {
                '\'' => raw.push_str("&apos;"),
                '"' => raw.push_str("&quot;"),
                '&' => raw.push_str("&amp;"),
                '<' => raw.push_str("&lt;"),
                '>' => raw.push_str("&gt;"),
                c => raw.push(c)
            }
        }

        Self::bless(raw)
    }
}

#[cfg(test)]
mod tests {
    use HtmlString;

    #[test]
    fn bless_keeps_intact() {
        let string = "<p>Hello, World!</p>";

        let html = HtmlString::bless(string);

        assert_eq!(html.raw, string);
    }

    #[test]
    fn escape_escapes() {
        let string = "<p>Hello & \"World''!</p>";
        let expected = "&lt;p&gt;Hello &amp; &quot;World&apos;&apos;!&lt;/p&gt;";

        let html = HtmlString::escape(string);

        assert_eq!(html.raw, expected);
    }
}
