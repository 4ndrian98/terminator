//! Cross-platform browser script execution using terminator SDK
//!
//! Uses terminator SDK selectors for cross-platform browser automation.
//! Finds console tab and prompt using proper selectors, runs JavaScript, extracts results.

use crate::{AutomationError, Desktop};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::info;

/// Execute JavaScript in browser using local server for result communication
pub async fn execute_script(
    browser_element: &crate::UIElement,
    script: &str,
) -> Result<String, AutomationError> {
    info!("🚀 Executing JavaScript using local server approach");

    // Step 1: Start a local server to receive results
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|e| AutomationError::PlatformError(format!("Failed to bind server: {e}")))?;
    let port = listener
        .local_addr()
        .map_err(|e| AutomationError::PlatformError(format!("Failed to get port: {e}")))?
        .port();

    info!("📡 Local server listening on port {}", port);

    let result = Arc::new(Mutex::new(None));
    let result_clone = result.clone();

    // Spawn server task
    let _server_handle = tokio::spawn(async move {
        info!("🔌 Server waiting for connection...");
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                info!("📡 Connection from: {}", addr);
                let mut buf = vec![0; 65536];
                match socket.read(&mut buf).await {
                    Ok(n) => {
                        let data = String::from_utf8_lossy(&buf[..n]);
                        info!(
                            "📨 Received {} bytes, first 500 chars: {}",
                            n,
                            &data[..data.len().min(500)]
                        );

                        // Parse GET request with query params
                        if data.starts_with("GET ") {
                            if let Some(line_end) = data.find('\r') {
                                let request_line = &data[4..line_end];
                                info!("📦 Request line: {}", request_line);

                                // Extract query params
                                if request_line.contains("?result=") {
                                    if let Some(query_start) = request_line.find("?result=") {
                                        let result_encoded = &request_line[query_start + 8..];
                                        let result_end = result_encoded
                                            .find(' ')
                                            .unwrap_or(result_encoded.len());
                                        let result_encoded = &result_encoded[..result_end];

                                        info!("📦 Encoded result: {}", result_encoded);

                                        // Simple URL decode (just handle %20 for spaces and basic chars)
                                        let decoded = result_encoded
                                            .replace("%20", " ")
                                            .replace("%22", "\"")
                                            .replace("%2C", ",");
                                        info!("📦 Decoded result: {}", decoded);
                                        *result_clone.lock().await = Some(decoded.to_string());
                                    }
                                } else if let Some(query_start) = request_line.find("?error=") {
                                    let error_encoded = &request_line[query_start + 7..];
                                    let error_end =
                                        error_encoded.find(' ').unwrap_or(error_encoded.len());
                                    let error_encoded = &error_encoded[..error_end];

                                    let decoded = error_encoded
                                        .replace("%20", " ")
                                        .replace("%22", "\"")
                                        .replace("%2C", ",");
                                    info!("📦 Decoded error: {}", decoded);
                                    *result_clone.lock().await =
                                        Some(format!("ERROR: {decoded}"));
                                }
                            }
                        }

                        // Send HTTP response
                        let response = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: 2\r\n\r\nOK";
                        let _ =
                            tokio::io::AsyncWriteExt::write_all(&mut socket, response.as_bytes())
                                .await;
                    }
                    Err(e) => info!("❌ Failed to read from socket: {}", e),
                }
            }
            Err(e) => info!("❌ Failed to accept connection: {}", e),
        }
    });

    // Wait a moment for server to be ready
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Step 2: Focus browser
    browser_element.focus()?;
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Step 3: Wrap script to send result to our server
    let wrapped_script = format!(
        r#"
        (function() {{
            try {{
                const result = eval(`{script}`);
                const resultStr = typeof result === 'object' ? JSON.stringify(result) : String(result);
                
                // Send result to local server - use simple image trick to bypass mixed content
                const img = new Image();
                img.src = 'http://127.0.0.1:{port}/?result=' + encodeURIComponent(resultStr);
                console.log('Result:', resultStr);
                
                return result;
            }} catch (e) {{
                const img = new Image();
                img.src = 'http://127.0.0.1:{port}/?error=' + encodeURIComponent(e.message);
                console.error('Error:', e.message);
                throw e;
            }}
        }})()
        "#
    );

    // Step 3: Open dev tools if not already open (Ctrl+Shift+J)
    info!("⚙️ Opening dev tools (Ctrl+Shift+J)");
    browser_element.press_key("{Ctrl}{Shift}J")?;
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Step 4: Clear console using Ctrl + L
    // info!("🧹 Clearing console using Ctrl + L");
    // browser_element.press_key("{Ctrl}L")?;
    // tokio::time::sleep(Duration::from_millis(500)).await;

    let desktop = Desktop::new(true, false)?;

    // Step 5: Find console prompt using terminator selector
    info!("🔍 Finding console prompt using name:Console prompt");
    let console_prompt = desktop
        .locator("role:document|name:DevTools >> name:Console prompt")
        .first(None)
        .await?;

    info!("⌨️ Typing wrapped JavaScript into console prompt");
    console_prompt.type_text(&wrapped_script, true)?;
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Step 6: Execute the script (Enter)
    info!("🚀 Executing script with Enter");
    console_prompt.press_key("{ENTER}")?;

    // Step 7: Wait for result from server
    info!("📄 Waiting for result from browser...");
    let mut attempts = 0;
    let max_attempts = 30;

    loop {
        tokio::time::sleep(Duration::from_millis(500)).await;

        if let Some(res) = result.lock().await.as_ref() {
            let final_result = res.clone();

            // Close dev tools
            info!("🚪 Closing dev tools");
            browser_element.press_key("{F12}")?;

            info!("✅ Script execution completed: {}", final_result);
            return Ok(final_result);
        }

        attempts += 1;
        if attempts >= max_attempts {
            break;
        }

        if attempts % 5 == 0 {
            info!(
                "⏳ Still waiting for result... ({}/{})",
                attempts, max_attempts
            );
        }
    }

    // Timeout - close dev tools and return error
    browser_element.press_key("{F12}")?;
    Err(AutomationError::Timeout(
        "Script execution timed out after 15 seconds".to_string(),
    ))
}

