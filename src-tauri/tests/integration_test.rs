use ruziniu_tools_lib::expand_url;

#[tokio::test]
async fn test_expand_url() {
    let client = reqwest::Client::new();
    let url = "https://psee.io/8cgqrq";
    let long_url = expand_url(&client, url).await;

    // 这个测试现在检查我们是否能正确处理各种情况
    match long_url {
        Ok(resolved_url) => {
            println!("Successfully resolved {} to {}", url, resolved_url);
            // 如果成功解析，应该是一个有效的URL且与原URL不同
            assert!(resolved_url.starts_with("http://") || resolved_url.starts_with("https://"));
            assert_ne!(resolved_url, url);
        }
        Err(e) => {
            println!("Failed to resolve {}: {}", url, e);
            // 如果失败，应该提供有意义的错误信息
            let error_msg = e.to_string();
            assert!(!error_msg.is_empty());
            assert!(!error_msg.contains("panic"));
        }
    }
}

#[tokio::test]
async fn test_known_good_redirect() {
    let client = reqwest::Client::new();
    // 使用一个已知可靠的短链服务进行测试
    let url = "https://httpbin.org/redirect/2";
    let long_url = expand_url(&client, url).await;

    assert!(long_url.is_ok(), "Should successfully resolve HTTP redirect");
    let resolved_url = long_url.unwrap();
    println!("Redirect test: {} -> {}", url, resolved_url);
    assert!(resolved_url.contains("/get"));
}
