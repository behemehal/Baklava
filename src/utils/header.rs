use anyhow::Result;

/// HTTP Header
/// ##### [https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers]
#[derive(Debug, Clone)]
pub struct Header {
    /// The name of the header
    pub name: String,
    /// The value of the header
    pub value: String,
}

impl Header {
    /// Parse raw http header response to [`Header`] struct
    /// ## Parameters
    /// * `line` - The raw http header response
    /// ## Returns
    /// [`Header`] if the header was successfully parsed else [`error::Error`]
    /// ## Example
    /// ```
    /// use menemen::request::Header;
    /// let header = Header::parse("Content-Type: text/html; charset=utf-8").unwrap();
    /// assert_eq!(header.name.clone(), "Content-Type");
    /// assert_eq!(header.value, "text/html; charset=utf-8");
    /// ```
    pub fn parse(line: &str) -> Result<Header> {
        if !line.contains(":") {
            return Err(anyhow::anyhow!("Failed to parse response info"));
        }

        let parts = line.split(": ").collect::<Vec<_>>();
        let name = parts[0].to_string();
        let value = if parts.len() == 1 {
            String::new()
        } else {
            parts[1].to_string()
        };
        Ok(Header { name, value })
    }
}
