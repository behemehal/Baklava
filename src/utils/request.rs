/// List of RequestTypes
/// #### https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
#[derive(Debug)]
pub enum RequestTypes {
    /// GET Method
    GET,
    /// POST Method
    POST,
    /// PUT Method
    PUT,
    /// DELETE Method
    DELETE,
}

impl RequestTypes {
    /// Get the string representation of the RequestType
    pub fn get_type(&self) -> String {
        match self {
            RequestTypes::GET => "GET".to_string(),
            RequestTypes::POST => "POST".to_string(),
            RequestTypes::PUT => "PUT".to_string(),
            RequestTypes::DELETE => "DELETE".to_string(),
        }
    }
}

/// ContentTypes
/// #### https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types
#[derive(Clone, Debug)]
pub enum ContentTypes {
    /// application/json
    JSON,
    /// text/html
    HTML,
    /// text/plain
    Text,
    /// image/png
    Png,
    /// audio/mp3
    MP3,
    /// text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
    Any,
    /// application/octet-stream
    OctetStream,
}

impl Default for ContentTypes {
    fn default() -> Self {
        ContentTypes::Any
    }
}

impl ContentTypes {
    /// Get the string representation of the ContentType
    /// ## Example
    /// ```
    /// use menemen::request::ContentTypes;
    /// let content_type = ContentTypes::JSON;
    /// assert_eq!(content_type.get_type(), "application/json");
    /// ```
    pub fn get_type(&self) -> &str {
        match self {
            ContentTypes::JSON => "application/json",
            ContentTypes::HTML => "text/html",
            ContentTypes::Text => "text/plain",
            ContentTypes::Png => "image/png",
            ContentTypes::MP3 => "audio/mp3",
            ContentTypes::Any => "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            ContentTypes::OctetStream => "application/octet-stream",
        }
    }
}

/// Request struct
#[derive(Debug)]
pub struct Request {
    /// Url of the request [`Url`]
    url: Url,
    request_type: RequestTypes,
    /// ContentType of the request [`ContentTypes`]
    pub content_type: ContentTypes,
    /// Headers of the request [`Vec<Header>`]
    headers: Vec<Header>,
    /// Timeout of the request [`u64`]
    timeout: u64,
    redirect: bool,
    /// Is the request sent
    sent: bool,
}
