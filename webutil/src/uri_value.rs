/// An assumed safe URI value.
pub struct UriValue {
    pub raw: String,
}

impl UriValue {
    /// Blesses a regular string to be a safe URI value. Do not give this any data you do not trust.
    pub fn bless<S: ToString>(raw: S) -> Self {
        UriValue {
            raw: raw.to_string()
        }
    }

    /// Escapes a string into an URI value.
    //pub fn escape<S: ToString>(_string: S) -> Self {
    //    unimplemented!();
    //}

    /// Unescapes a URI value into a string.
    pub fn unescape(&self) -> String {
        let mut value = String::new();

        let mut iter = self.raw.chars();
        while let Some(c) = iter.next() {
            // Check if we've hit an escaped char
            if c == '%' {
                // Decode the hex value and add the actual char
                if let Some(dec) = Self::decode_escaped(&mut iter) {
                    value.push(dec);
                } else {
                    value.push(c);
                }
            } else {
                // Just pass over the char
                value.push(c);
            }
        }

        value
    }

    fn decode_escaped<I: Iterator<Item=char>>(mut iter: I) -> Option<char> {
        // Construct the hex string
        let mut hex = String::new();
        hex.push(try_opt!(iter.next()));
        hex.push(try_opt!(iter.next()));

        // Decode the hex string
        let value = try_opt!(u8::from_str_radix(&hex, 16).ok());
        Some(value as char)
    }
}

#[cfg(test)]
mod tests {
    use UriValue;

    #[test]
    fn unescapes_escaped() {
        let value = UriValue::bless("%3Ch2%3E");
        let expected = "<h2>";

        let result = value.unescape();

        assert_eq!(result, expected);
    }

    #[test]
    fn malformed_doesnt_panic() {
        let value = UriValue::bless("%3Ch2%3");
        let _result = value.unescape();

        let value = UriValue::bless("%/C");
        let _result = value.unescape();
    }
}
