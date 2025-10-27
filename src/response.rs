use std::fmt::Display;

pub struct Response {
    pub url: String,
    pub status_code: u16,
    pub content_length: u64,
    pub title: String,
}

impl Response {
    pub fn new(
        url: String,
        status_code: u16,
        content_length: u64,
        title: String
    ) -> Self {
        Self { url, status_code, content_length, title }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} - (Length: {}, Title: \"{}\")",
            self.status_code, self.url, self.content_length, self.title
        )
    }
}