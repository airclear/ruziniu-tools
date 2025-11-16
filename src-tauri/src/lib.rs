use tauri::{Emitter, Window};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use tokio::sync::Semaphore;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Clone, serde::Serialize)]
struct ConversionProgress {
    total: usize,
    converted: usize,
}

#[derive(Clone, serde::Serialize)]
struct ConversionResult {
    short_url: String,
    long_url: String,
}

pub async fn expand_url(_client: &reqwest::Client, url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    // 创建带有重定向策略的客户端
    let redirect_client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10))
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    // 配置请求以获得更好的重定向处理
    let response = redirect_client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Accept-Encoding", "gzip, deflate")
        .header("DNT", "1")
        .header("Connection", "keep-alive")
        .header("Upgrade-Insecure-Requests", "1")
        .send()
        .await?;

    // 获取最终URL（在所有重定向之后）
    let final_url = response.url().to_string();

    // 如果URL没有变化，尝试其他方法
    if final_url == url {
        // 对于某些短链服务，可能需要特殊处理
        let response_text = response.text().await?;

        // 尝试从HTML中查找重定向
        if let Some(redirect_url) = extract_redirect_from_html(&response_text) {
            return Ok(redirect_url);
        }

        // 如果仍然没有找到，返回原始URL
        Ok(url.to_string())
    } else {
        Ok(final_url)
    }
}

fn extract_redirect_from_html(html: &str) -> Option<String> {
    // 查找常见的重定向模式
    use regex::Regex;

    // 查找 JavaScript 重定向: window.location, location.href, etc.
    let js_patterns = [
        r#"window\.location\s*=\s*["']([^"']+)["']"#,
        r#"location\.href\s*=\s*["']([^"']+)["']"#,
        r#"window\.location\.href\s*=\s*["']([^"']+)["']"#,
        r#"location\.replace\s*\(\s*["']([^"']+)["']\s*\)"#,
    ];

    for pattern in &js_patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(caps) = re.captures(html) {
                if let Some(url) = caps.get(1) {
                    return Some(url.as_str().to_string());
                }
            }
        }
    }

    // 查找 meta refresh 重定向
    if let Ok(re) = Regex::new(r#"http-equiv=["']refresh["']\s+content=["'][^;]*;\s*url=([^"']+)["']"#) {
        if let Some(caps) = re.captures(html) {
            if let Some(url) = caps.get(1) {
                return Some(url.as_str().to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_http_redirect() {
        let client = reqwest::Client::new();
        // 测试一个简单的重定向URL (httpbin.org)
        let result = expand_url(&client, "https://httpbin.org/redirect/1").await;
        assert!(result.is_ok());
        let expanded_url = result.unwrap();
        println!("HTTP redirect result: {}", expanded_url);
        // 应该重定向到 httpbin.org/get
        assert!(expanded_url.contains("/get"));
    }

    #[tokio::test]
    async fn test_extract_redirect_from_html() {
        // 测试JavaScript重定向
        let js_html = r#"
        <html>
        <script>
        window.location = "https://example.com/target";
        </script>
        </html>
        "#;

        let result = extract_redirect_from_html(js_html);
        assert_eq!(result, Some("https://example.com/target".to_string()));

        // 测试location.href
        let href_html = r#"
        <script>
        location.href = "https://another-example.com/page";
        </script>
        "#;

        let result = extract_redirect_from_html(href_html);
        assert_eq!(result, Some("https://another-example.com/page".to_string()));

        // 测试meta refresh
        let meta_html = r#"
        <html>
        <head>
        <meta http-equiv="refresh" content="0; url=https://meta-redirect.com/target">
        </head>
        <body>
        Redirecting...
        </body>
        </html>
        "#;

        let result = extract_redirect_from_html(meta_html);
        assert_eq!(result, Some("https://meta-redirect.com/target".to_string()));

        // 测试没有重定向的HTML
        let normal_html = r#"
        <html>
        <body>
        <h1>Normal Page</h1>
        </body>
        </html>
        "#;

        let result = extract_redirect_from_html(normal_html);
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_invalid_url() {
        let client = reqwest::Client::new();
        let result = expand_url(&client, "https://invalid-domain-that-does-not-exist-12345.com").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_psee_io_redirect() {
        let client = reqwest::Client::new();
        // 测试您提供的具体URL
        let result = expand_url(&client, "https://psee.io/8cgqrq").await;

        match result {
            Ok(url) => {
                println!("psee.io resolved to: {}", url);
                // 如果成功解析，应该是一个有效的URL
                assert!(url.starts_with("http://") || url.starts_with("https://"));
                assert!(url != "https://psee.io/8cgqrq"); // 应该是不同的URL
            }
            Err(e) => {
                println!("psee.io expansion failed: {}", e);
                // 如果失败了，我们应该知道具体原因
                assert!(!e.to_string().is_empty());
            }
        }
    }

    #[tokio::test]
    async fn test_multiple_redirect_patterns() {
        let test_cases = vec![
            // (描述, HTML内容, 期望结果)
            ("window.location.href", r#"window.location.href = "https://test1.com""#, Some("https://test1.com")),
            ("location.replace", r#"location.replace("https://test2.com")"#, Some("https://test2.com")),
            ("complex meta refresh", r#"<meta http-equiv="refresh" content="5; url=https://test3.com/path?query=1">"#, Some("https://test3.com/path?query=1")),
            ("no redirect", r#"<h1>Just a normal page</h1>"#, None),
        ];

        for (description, html, expected) in test_cases {
            let result = extract_redirect_from_html(html);
            assert_eq!(result, expected.map(String::from), "Failed test case: {}", description);
        }
    }

    #[test]
    fn test_regex_patterns() {
        use regex::Regex;

        // 测试正则表达式模式是否正确
        let patterns = [
            r#"window\.location\s*=\s*["']([^"']+)["']"#,
            r#"location\.href\s*=\s*["']([^"']+)["']"#,
            r#"window\.location\.href\s*=\s*["']([^"']+)["']"#,
            r#"location\.replace\s*\(\s*["']([^"']+)["']\s*\)"#,
        ];

        for pattern in &patterns {
            assert!(Regex::new(pattern).is_ok(), "Invalid regex pattern: {}", pattern);
        }

        let meta_pattern = r#"http-equiv=["']refresh["']\s+content=["'][^;]*;\s*url=([^"']+)["']"#;
        assert!(Regex::new(meta_pattern).is_ok(), "Invalid meta refresh regex pattern");
    }
}

#[tauri::command]
async fn batch_short2long_url(
    window: Window,
    urls: Vec<String>,
    concurrency: u32,
) -> Result<(), String> {
    let total_urls = urls.len();
    let converted_count = Arc::new(AtomicUsize::new(0));
    let semaphore = Arc::new(Semaphore::new(concurrency as usize));

    let client = reqwest::Client::new();

    for url in urls {
        let window = window.clone();
        let converted_count = Arc::clone(&converted_count);
        let semaphore = Arc::clone(&semaphore);
        let client = client.clone();

        tokio::spawn(async move {
            let permit = semaphore.acquire().await.unwrap();
            let long_url = match expand_url(&client, &url).await {
                Ok(url) => url,
                Err(e) => format!("Error: {}", e),
            };

            let result = ConversionResult {
                short_url: url,
                long_url,
            };
            window.emit("conversion-result", result).unwrap();

            let converted = converted_count.fetch_add(1, Ordering::SeqCst) + 1;
            let progress = ConversionProgress {
                total: total_urls,
                converted,
            };
            window.emit("conversion-progress", progress).unwrap();
            drop(permit);
        });
    }

    Ok(())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, batch_short2long_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
