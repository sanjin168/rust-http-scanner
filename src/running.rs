use std::sync::Arc;

use crate::command::Args;
use crate::utils::{ read_file, parse_title };
use crate::request::Request;
use crate::response::Response;
use futures::stream::{ self, StreamExt };

pub async fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let req: Arc<Box<Request>>;

    if let Some(proxy) = args.proxy {
        req = Arc::new(
            Box::new(
                Request::new(
                    args.timeout,
                    args.follow_redirect,
                    args.max_redirect,
                    None,
                    Some(proxy)
                )
            )
        );
    } else {
        req = Arc::new(
            Box::new(
                Request::new(args.timeout, args.follow_redirect, args.max_redirect, None, None)
            )
        );
    }

    if let Some(target) = args.target {
        match use_target(&target, &req).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("use_target: {}", e);
            }
        }
    }

    if let Some(path) = args.file {
        match use_targets_file(&path, &req, args.threads).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("use_targets_file: {}", e);
            }
        }
    }

    Ok(())
}

// use target argv
pub async fn use_target(target: &str, req: &Request) -> Result<(), Box<dyn std::error::Error>> {
    let _ = process_url(req, target).await;
    Ok(())
}

// use target urls file
pub async fn use_targets_file(
    file_path: &str,
    req: &Request,
    thread_number: usize
) -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_file(&file_path);
    match lines {
        Ok(lines) => {
            cncurrency_request(&lines, req, thread_number).await?;
        }
        Err(e) => {
            log::error!("read_file: {}", e);
        }
    }
    Ok(())
}

async fn process_url(req: &Request, url: &str) {
    let res = match req.send_request(url).await {
        Ok(res) => { res }
        Err(_) => {
            return;
        }
    };
    let status_code = res.status().as_u16();
    let content_length = res.content_length().unwrap_or(0);

    let title_info = if content_length > 0 {
        match res.text().await {
            Ok(body) => { parse_title(&body).ok().flatten() }
            Err(e) => {
                log::error!("{}: {}", &url, e);
                None
            }
        }
    } else {
        None
    };

    if let Some(title) = title_info {
        let response = Response::new(
            url.to_string(),
            status_code,
            content_length,
            title
        );
        log::info!("{}", response);
    } else {
        let response = Response::new(
            url.to_string(),
            status_code,
            content_length,
            String::from("Title NotFound")
        );
        log::info!("{}", response);
    }
}

// Concurrency request
pub async fn cncurrency_request(
    urls: &Vec<String>,
    req: &Request,
    thread_number: usize
) -> Result<(), Box<dyn std::error::Error>> {
    stream
        ::iter(urls)
        .map(|url| {
            let req = req.clone();
            async move {
                let _ = process_url(&req, &url).await;
            }
        })
        .buffer_unordered(thread_number as usize)
        .for_each(|_| async {}).await;

    Ok(())
}
