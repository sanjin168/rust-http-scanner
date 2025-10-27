use reqwest::{ header, Client, Proxy };
use reqwest::header::HeaderMap;
use reqwest::redirect::Policy;

#[derive(Clone)]
pub struct Request {
    client: Client,
}

impl Request {
    pub fn new(
        timeout: u64,
        follow_redirect: bool,
        max_redirect: usize,
        headers: Option<HeaderMap>,
        proxy: Option<String>
    ) -> Self {
        let mut client_builder = Client::builder()
            .timeout(std::time::Duration::from_secs(timeout))
            .redirect(Self::check_is_follow_redirect(follow_redirect, max_redirect));

        if let Some(ref proxy_url) = proxy {
            if let Ok(proxy_obj) = Proxy::all(proxy_url) {
                client_builder = client_builder.proxy(proxy_obj);
            }
        }

        if let Some(ref hdrs) = headers {
            client_builder = client_builder.default_headers(hdrs.clone());
        } else {
            let mut default_headers = HeaderMap::new();
            default_headers.insert(
                header::USER_AGENT,
                header::HeaderValue::from_static(
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"
                )
            );
            client_builder = client_builder.default_headers(default_headers);
        }

        let client = client_builder.build().expect("Failed to build reqwest client");

        Request {
            client,
        }
    }

    fn check_is_follow_redirect(follow_redirect: bool, max_redirect: usize) -> Policy {
        if follow_redirect { Policy::limited(max_redirect) } else { Policy::none() }
    }

    // use client-build send request.
    pub async fn send_request(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        self.client.get(url).send().await
    }
}
