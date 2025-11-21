use regex::Regex;
use reqwest::{header::HeaderMap, Client};

#[derive(Debug, Clone, serde::Serialize)]
pub struct RednoteEngagement {
    pub likes: String,
    pub comments: String,
    pub collects: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RednoteDownload {
    pub quality: String,
    pub url: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RednoteResult {
    #[serde(rename = "noteId")]
    pub note_id: String,
    pub nickname: String,
    pub title: String,
    pub desc: String,
    pub keywords: String,
    pub duration: String,
    pub engagement: RednoteEngagement,
    pub images: Vec<String>,
    pub downloads: Vec<RednoteDownload>,
}

fn capture_single(html: &str, pattern: &str) -> String {
    Regex::new(pattern)
        .unwrap()
        .captures(html)
        .and_then(|c| c.get(1))
        .map(|v| v.as_str().trim().to_string())
        .unwrap_or_default()
}

fn capture_multi(html: &str, pattern: &str) -> Vec<String> {
    Regex::new(pattern)
        .unwrap()
        .captures_iter(html)
        .filter_map(|c| c.get(1))
        .map(|v| v.as_str().trim().to_string())
        .collect()
}

pub async fn scrape(url: String) -> Result<RednoteResult, String> {
    // --- Headers Chrome lengkap seperti axios ---
    let mut headers = HeaderMap::new();
    headers.insert("Accept", "*/*".parse().unwrap());
    headers.insert("Accept-Language", "en-US,en;q=0.9".parse().unwrap());
    headers.insert("Cache-Control", "no-cache".parse().unwrap());
    headers.insert("Pragma", "no-cache".parse().unwrap());
    headers.insert("Referer", "https://www.xiaohongshu.com/".parse().unwrap());
    headers.insert("Origin", "https://www.xiaohongshu.com".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        r#""Chromium";v="132", "Not_A Brand";v="99""#.parse().unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    headers.insert("Accept-Encoding", "gzip, deflate, br".parse().unwrap());

    let client = Client::builder()
        .default_headers(headers)
        .redirect(reqwest::redirect::Policy::limited(10))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36")
        .brotli(true)
        .gzip(true)
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Build client error: {}", e))?;

    let html = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?
        .text()
        .await
        .map_err(|e| format!("Read HTML failed: {}", e))?;

    // println!("{}", html); // DEBUG — kalau masih error, aktifkan

    // --- Parsing persis seperti TypeScript ---
    let title = capture_single(&html, r#"(?is)<title>(.*?)</title>"#);
    let desc = capture_single(
        &html,
        r#"(?is)<meta[^>]*name="description"[^>]*content="(.*?)""#,
    );
    let keywords = capture_single(
        &html,
        r#"(?is)<meta[^>]*name="keywords"[^>]*content="(.*?)""#,
    );
    let video_url = capture_single(
        &html,
        r#"(?is)<meta[^>]*name="og:video"[^>]*content="(.*?)""#,
    );
    let og_url = capture_single(&html, r#"(?is)<meta[^>]*name="og:url"[^>]*content="(.*?)""#);

    let note_id = og_url.split('/').next_back().unwrap_or("").to_string();

    let duration = capture_single(
        &html,
        r#"(?is)<meta[^>]*name="og:videotime"[^>]*content="(.*?)""#,
    );

    let og_title = capture_single(
        &html,
        r#"(?is)<meta[^>]*name="og:title"[^>]*content="(.*?)""#,
    );
    let nickname = og_title.split(" - ").next().unwrap_or("").to_string();

    // semua og:image
    let images = capture_multi(
        &html,
        r#"(?is)<meta[^>]*name="og:image"[^>]*content="(.*?)""#,
    );

    // engagement
    let comments = capture_single(
        &html,
        r#"(?is)<meta[^>]*name="og:xhs:note_comment"[^>]*content="(.*?)""#,
    );
    let likes = capture_single(
        &html,
        r#"(?is)<meta[^>]*name="og:xhs:note_like"[^>]*content="(.*?)""#,
    );
    let collects = capture_single(
        &html,
        r#"(?is)<meta[^>]*name="og:xhs:note_collect"[^>]*content="(.*?)""#,
    );

    // downloads
    let downloads = if !video_url.is_empty() {
        vec![RednoteDownload {
            quality: "Original".to_string(),
            url: video_url,
        }]
    } else {
        vec![]
    };

    Ok(RednoteResult {
        note_id,
        nickname,
        title,
        desc,
        keywords,
        duration,
        engagement: RednoteEngagement {
            likes,
            comments,
            collects,
        },
        images,
        downloads,
    })
}

pub fn is_valid_rednote_url(url: &str) -> bool {
    Regex::new(r"(?i)^https?://(.*?)?(xhslink\.com|xiaohongshu\.com)/[^\s]+$")
        .unwrap()
        .is_match(url)
}
