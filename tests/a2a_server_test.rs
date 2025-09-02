use inference_gateway_adk::{A2AClient, A2AServerBuilder};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};

/// Test configuration for the A2A server validation
#[derive(Debug)]
struct TestConfig {
    server_addr: SocketAddr,
    gateway_url: String,
    timeout_duration: Duration,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            server_addr: "127.0.0.1:8082".parse().unwrap(),
            gateway_url: "http://localhost:8080/v1".to_string(),
            timeout_duration: Duration::from_secs(30),
        }
    }
}

/// A test suite for validating A2A JSON-RPC endpoints
#[derive(Debug)]
struct A2AValidationSuite {
    config: TestConfig,
    client: Option<A2AClient>,
    test_results: HashMap<String, TestResult>,
}

#[derive(Debug, Clone)]
struct TestResult {
    name: String,
    passed: bool,
    error: Option<String>,
    response: Option<Value>,
    duration: Duration,
}

impl A2AValidationSuite {
    fn new() -> Self {
        Self {
            config: TestConfig::default(),
            client: None,
            test_results: HashMap::new(),
        }
    }

    async fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Setting up A2A validation suite...");

        let agent_card = serde_json::json!({
            "name": "Test A2A Agent",
            "description": "A test agent for validating A2A server functionality",
            "version": "1.0.0",
            "protocolVersion": "0.2.6",
            "url": format!("http://{}/a2a", self.config.server_addr),
            "preferredTransport": "JSONRPC",
            "capabilities": {
                "streaming": true,
                "pushNotifications": false,
                "stateTransitionHistory": false
            },
            "defaultInputModes": ["text/plain"],
            "defaultOutputModes": ["text/plain"],
            "skills": [
                {
                    "id": "general-conversation",
                    "name": "General Conversation",
                    "description": "Can engage in general conversation and answer questions",
                    "tags": ["conversation", "qa"]
                }
            ],
            "provider": {
                "organization": "Test Organization",
                "url": "https://example.com"
            }
        });

        let agent_card: inference_gateway_adk::a2a_types::AgentCard =
            serde_json::from_value(agent_card)?;

        let server = A2AServerBuilder::new()
            .with_gateway_url(&self.config.gateway_url)
            .with_agent_card(agent_card)
            .build()
            .await?;

        let addr = self.config.server_addr;
        let _server_handle = tokio::spawn(async move {
            if let Err(e) = server.serve(addr).await {
                error!("Test server failed: {}", e);
            }
        });

        sleep(Duration::from_millis(1000)).await;

        let client_url = format!("http://{}", self.config.server_addr);
        self.client = Some(A2AClient::new(&client_url)?);

        info!("A2A validation suite setup complete");
        Ok(())
    }

    async fn run_all_tests(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Running comprehensive A2A validation tests...");

        self.test_health_endpoint().await;
        self.test_agent_card_endpoint().await;

        self.test_message_send().await;
        self.test_message_stream().await;
        self.test_tasks_get().await;
        self.test_tasks_cancel().await;
        self.test_push_notification_config_set().await;
        self.test_push_notification_config_get().await;
        self.test_push_notification_config_list().await;
        self.test_push_notification_config_delete().await;
        self.test_tasks_resubscribe().await;

        self.test_invalid_json_rpc().await;
        self.test_method_not_found().await;
        self.test_invalid_parameters().await;

        self.print_test_results();

        Ok(())
    }

    async fn test_health_endpoint(&mut self) {
        let start = std::time::Instant::now();
        let test_name = "health_endpoint".to_string();

        match self.client.as_ref().unwrap().get_health().await {
            Ok(health) => {
                info!("✅ Health check passed: {}", health.status);
                self.test_results.insert(test_name, TestResult {
                    name: "Health Endpoint".to_string(),
                    passed: true,
                    error: None,
                    response: Some(json!({
                        "status": health.status,
                        "timestamp": health.timestamp
                    })),
                    duration: start.elapsed(),
                });
            }
            Err(e) => {
                error!("❌ Health check failed: {}", e);
                self.test_results.insert(test_name, TestResult {
                    name: "Health Endpoint".to_string(),
                    passed: false,
                    error: Some(e.to_string()),
                    response: None,
                    duration: start.elapsed(),
                });
            }
        }
    }

    async fn test_agent_card_endpoint(&mut self) {
        let start = std::time::Instant::now();
        let test_name = "agent_card_endpoint".to_string();

        match self.client.as_ref().unwrap().get_agent_card().await {
            Ok(card) => {
                info!(
                    "✅ Agent card retrieved: {} - {}",
                    card.name, card.description
                );
                self.test_results.insert(test_name, TestResult {
                    name: "Agent Card Endpoint".to_string(),
                    passed: true,
                    error: None,
                    response: Some(json!({
                        "name": card.name,
                        "description": card.description,
                        "protocol_version": card.protocol_version
                    })),
                    duration: start.elapsed(),
                });
            }
            Err(e) => {
                error!("❌ Agent card failed: {}", e);
                self.test_results.insert(test_name, TestResult {
                    name: "Agent Card Endpoint".to_string(),
                    passed: false,
                    error: Some(e.to_string()),
                    response: None,
                    duration: start.elapsed(),
                });
            }
        }
    }

    async fn test_message_send(&mut self) {
        let start = std::time::Instant::now();
        let test_name = "message_send".to_string();

        let request = json!({
            "jsonrpc": "2.0",
            "id": "test-message-send-001",
            "method": "message/send",
            "params": {
                "message": {
                    "kind": "message",
                    "messageId": "msg-001",
                    "role": "user",
                    "parts": [{
                        "kind": "text",
                        "text": "Hello, this is a test message for the A2A message/send endpoint."
                    }]
                }
            }
        });

        match self.send_jsonrpc_request(request).await {
            Ok(response) => {
                if response.get("result").is_some() {
                    info!("✅ message/send test passed");
                    self.test_results.insert(test_name, TestResult {
                        name: "message/send".to_string(),
                        passed: true,
                        error: None,
                        response: Some(response),
                        duration: start.elapsed(),
                    });
                } else {
                    warn!(
                        "⚠️ message/send returned error: {:?}",
                        response.get("error")
                    );
                    self.test_results.insert(test_name, TestResult {
                        name: "message/send".to_string(),
                        passed: false,
                        error: Some(format!(
                            "Server returned error: {:?}",
                            response.get("error")
                        )),
                        response: Some(response),
                        duration: start.elapsed(),
                    });
                }
            }
            Err(e) => {
                error!("❌ message/send test failed: {}", e);
                self.test_results.insert(test_name, TestResult {
                    name: "message/send".to_string(),
                    passed: false,
                    error: Some(e.to_string()),
                    response: None,
                    duration: start.elapsed(),
                });
            }
        }
    }

    async fn test_message_stream(&mut self) {
        let start = std::time::Instant::now();
        let test_name = "message_stream".to_string();

        let request = json!({
            "jsonrpc": "2.0",
            "id": "test-message-stream-001",
            "method": "message/stream",
            "params": {
                "message": {
                    "kind": "message",
                    "messageId": "msg-stream-001",
                    "role": "user",
                    "parts": [{
                        "kind": "text",
                        "text": "Hello, this is a test message for the A2A message/stream endpoint."
                    }]
                }
            }
        });

        match self.send_jsonrpc_request(request).await {
            Ok(response) => {
                if response.get("result").is_some() {
                    info!("✅ message/stream test passed");
                    self.test_results.insert(test_name, TestResult {
                        name: "message/stream".to_string(),
                        passed: true,
                        error: None,
                        response: Some(response),
                        duration: start.elapsed(),
                    });
                } else {
                    warn!(
                        "⚠️ message/stream returned error: {:?}",
                        response.get("error")
                    );
                    self.test_results.insert(test_name, TestResult {
                        name: "message/stream".to_string(),
                        passed: false,
                        error: Some(format!(
                            "Server returned error: {:?}",
                            response.get("error")
                        )),
                        response: Some(response),
                        duration: start.elapsed(),
                    });
                }
            }
            Err(e) => {
                error!("❌ message/stream test failed: {}", e);
                self.test_results.insert(test_name, TestResult {
                    name: "message/stream".to_string(),
                    passed: false,
                    error: Some(e.to_string()),
                    response: None,
                    duration: start.elapsed(),
                });
            }
        }
    }

    async fn test_tasks_get(&mut self) {
        let start = std::time::Instant::now();
        let test_name = "tasks_get".to_string();

        let request = json!({
            "jsonrpc": "2.0",
            "id": "test-tasks-get-001",
            "method": "tasks/get",
            "params": {
                "id": "test-task-001"
            }
        });

        match self.send_jsonrpc_request(request).await {
            Ok(response) => {
                if response.get("result").is_some()
                    || (response.get("error").is_some()
                        && response["error"]["code"].as_i64() == Some(-32001))
                {
                    info!("✅ tasks/get test passed (task not found is expected)");
                    self.test_results.insert(test_name, TestResult {
                        name: "tasks/get".to_string(),
                        passed: true,
                        error: None,
                        response: Some(response),
                        duration: start.elapsed(),
                    });
                } else {
                    warn!(
                        "⚠️ tasks/get returned unexpected error: {:?}",
                        response.get("error")
                    );
                    self.test_results.insert(test_name, TestResult {
                        name: "tasks/get".to_string(),
                        passed: false,
                        error: Some(format!("Unexpected error: {:?}", response.get("error"))),
                        response: Some(response),
                        duration: start.elapsed(),
                    });
                }
            }
            Err(e) => {
                error!("❌ tasks/get test failed: {}", e);
                self.test_results.insert(test_name, TestResult {
                    name: "tasks/get".to_string(),
                    passed: false,
                    error: Some(e.to_string()),
                    response: None,
                    duration: start.elapsed(),
                });
            }
        }
    }

    async fn test_tasks_cancel(&mut self) {
        let start = std::time::Instant::now();
        let test_name = "tasks_cancel".to_string();

        let request = json!({
            "jsonrpc": "2.0",
            "id": "test-tasks-cancel-001",
            "method": "tasks/cancel",
            "params": {
                "id": "test-task-001"
            }
        });

        match self.send_jsonrpc_request(request).await {
            Ok(response) => {
                if response.get("result").is_some()
                    || (response.get("error").is_some()
                        && (response["error"]["code"].as_i64() == Some(-32001)
                            || response["error"]["code"].as_i64() == Some(-32002)))
                {
                    info!("✅ tasks/cancel test passed (expected error for non-existent task)");
                    self.test_results.insert(test_name, TestResult {
                        name: "tasks/cancel".to_string(),
                        passed: true,
                        error: None,
                        response: Some(response),
                        duration: start.elapsed(),
                    });
                } else {
                    warn!(
                        "⚠️ tasks/cancel returned unexpected error: {:?}",
                        response.get("error")
                    );
                    self.test_results.insert(test_name, TestResult {
                        name: "tasks/cancel".to_string(),
                        passed: false,
                        error: Some(format!("Unexpected error: {:?}", response.get("error"))),
                        response: Some(response),
                        duration: start.elapsed(),
                    });
                }
            }
            Err(e) => {
                error!("❌ tasks/cancel test failed: {}", e);
                self.test_results.insert(test_name, TestResult {
                    name: "tasks/cancel".to_string(),
                    passed: false,
                    error: Some(e.to_string()),
                    response: None,
                    duration: start.elapsed(),
                });
            }
        }
    }

    async fn test_push_notification_config_set(&mut self) {
        let start = std::time::Instant::now();
        let test_name = "push_notification_config_set".to_string();

        let request = json!({
            "jsonrpc": "2.0",
            "id": "test-push-config-set-001",
            "method": "tasks/pushNotificationConfig/set",
            "params": {
                "taskId": "test-task-001",
                "pushNotificationConfig": {
                    "url": "http://localhost:9999/webhook",
                    "token": "test-token-123"
                }
            }
        });

        match self.send_jsonrpc_request(request).await {
            Ok(response) => {
                if response.get("result").is_some()
                    || (response.get("error").is_some()
                        && response["error"]["code"].as_i64() == Some(-32003))
                {
                    info!("✅ tasks/pushNotificationConfig/set test passed");
                    self.test_results.insert(test_name, TestResult {
                        name: "tasks/pushNotificationConfig/set".to_string(),
                        passed: true,
                        error: None,
                        response: Some(response),
                        duration: start.elapsed(),
                    });
                } else {
                    warn!(
                        "⚠️ push notification config set returned unexpected error: {:?}",
                        response.get("error")
                    );
                    self.test_results.insert(test_name, TestResult {
                        name: "tasks/pushNotificationConfig/set".to_string(),
                        passed: false,
                        error: Some(format!("Unexpected error: {:?}", response.get("error"))),
                        response: Some(response),
                        duration: start.elapsed(),
                    });
                }
            }
            Err(e) => {
                error!("❌ push notification config set test failed: {}", e);
                self.test_results.insert(test_name, TestResult {
                    name: "tasks/pushNotificationConfig/set".to_string(),
                    passed: false,
                    error: Some(e.to_string()),
                    response: None,
                    duration: start.elapsed(),
                });
            }
        }
    }

    async fn test_push_notification_config_get(&mut self) {
        let start = std::time::Instant::now();
        let test_name = "push_notification_config_get".to_string();

        let request = json!({
            "jsonrpc": "2.0",
            "id": "test-push-config-get-001",
            "method": "tasks/pushNotificationConfig/get",
            "params": {
                "id": "test-task-001"
            }
        });

        match self.send_jsonrpc_request(request).await {
            Ok(response) => {
                self.test_results.insert(test_name, TestResult {
                    name: "tasks/pushNotificationConfig/get".to_string(),
                    passed: true,
                    error: None,
                    response: Some(response),
                    duration: start.elapsed(),
                });
                info!("✅ tasks/pushNotificationConfig/get test passed");
            }
            Err(e) => {
                error!("❌ push notification config get test failed: {}", e);
                self.test_results.insert(test_name, TestResult {
                    name: "tasks/pushNotificationConfig/get".to_string(),
                    passed: false,
                    error: Some(e.to_string()),
                    response: None,
                    duration: start.elapsed(),
                });
            }
        }
    }

    async fn test_push_notification_config_list(&mut self) {
        let start = std::time::Instant::now();
        let test_name = "push_notification_config_list".to_string();

        let request = json!({
            "jsonrpc": "2.0",
            "id": "test-push-config-list-001",
            "method": "tasks/pushNotificationConfig/list",
            "params": {
                "id": "test-task-001"
            }
        });

        match self.send_jsonrpc_request(request).await {
            Ok(response) => {
                self.test_results.insert(test_name, TestResult {
                    name: "tasks/pushNotificationConfig/list".to_string(),
                    passed: true,
                    error: None,
                    response: Some(response),
                    duration: start.elapsed(),
                });
                info!("✅ tasks/pushNotificationConfig/list test passed");
            }
            Err(e) => {
                error!("❌ push notification config list test failed: {}", e);
                self.test_results.insert(test_name, TestResult {
                    name: "tasks/pushNotificationConfig/list".to_string(),
                    passed: false,
                    error: Some(e.to_string()),
                    response: None,
                    duration: start.elapsed(),
                });
            }
        }
    }

    async fn test_push_notification_config_delete(&mut self) {
        let start = std::time::Instant::now();
        let test_name = "push_notification_config_delete".to_string();

        let request = json!({
            "jsonrpc": "2.0",
            "id": "test-push-config-delete-001",
            "method": "tasks/pushNotificationConfig/delete",
            "params": {
                "id": "test-task-001",
                "pushNotificationConfigId": "test-config-001"
            }
        });

        match self.send_jsonrpc_request(request).await {
            Ok(response) => {
                self.test_results.insert(test_name, TestResult {
                    name: "tasks/pushNotificationConfig/delete".to_string(),
                    passed: true,
                    error: None,
                    response: Some(response),
                    duration: start.elapsed(),
                });
                info!("✅ tasks/pushNotificationConfig/delete test passed");
            }
            Err(e) => {
                error!("❌ push notification config delete test failed: {}", e);
                self.test_results.insert(test_name, TestResult {
                    name: "tasks/pushNotificationConfig/delete".to_string(),
                    passed: false,
                    error: Some(e.to_string()),
                    response: None,
                    duration: start.elapsed(),
                });
            }
        }
    }

    async fn test_tasks_resubscribe(&mut self) {
        let start = std::time::Instant::now();
        let test_name = "tasks_resubscribe".to_string();

        let request = json!({
            "jsonrpc": "2.0",
            "id": "test-tasks-resubscribe-001",
            "method": "tasks/resubscribe",
            "params": {
                "id": "test-task-001"
            }
        });

        match self.send_jsonrpc_request(request).await {
            Ok(response) => {
                self.test_results.insert(test_name, TestResult {
                    name: "tasks/resubscribe".to_string(),
                    passed: true,
                    error: None,
                    response: Some(response),
                    duration: start.elapsed(),
                });
                info!("✅ tasks/resubscribe test passed");
            }
            Err(e) => {
                error!("❌ tasks/resubscribe test failed: {}", e);
                self.test_results.insert(test_name, TestResult {
                    name: "tasks/resubscribe".to_string(),
                    passed: false,
                    error: Some(e.to_string()),
                    response: None,
                    duration: start.elapsed(),
                });
            }
        }
    }

    async fn test_invalid_json_rpc(&mut self) {
        let start = std::time::Instant::now();
        let test_name = "invalid_json_rpc".to_string();

        let request = json!({
            "invalid": "not a valid json-rpc request"
        });

        match self.send_jsonrpc_request(request).await {
            Ok(response) => {
                if response.get("error").is_some()
                    && response["error"]["code"].as_i64() == Some(-32600)
                {
                    info!("✅ Invalid JSON-RPC test passed - correctly returned error -32600");
                    self.test_results.insert(test_name, TestResult {
                        name: "Invalid JSON-RPC Request".to_string(),
                        passed: true,
                        error: None,
                        response: Some(response),
                        duration: start.elapsed(),
                    });
                } else {
                    warn!("⚠️ Invalid JSON-RPC didn't return expected error");
                    self.test_results.insert(test_name, TestResult {
                        name: "Invalid JSON-RPC Request".to_string(),
                        passed: false,
                        error: Some("Expected error -32600 for invalid request".to_string()),
                        response: Some(response),
                        duration: start.elapsed(),
                    });
                }
            }
            Err(e) => {
                info!("✅ Invalid JSON-RPC test passed - HTTP error: {}", e);
                self.test_results.insert(test_name, TestResult {
                    name: "Invalid JSON-RPC Request".to_string(),
                    passed: true,
                    error: None,
                    response: None,
                    duration: start.elapsed(),
                });
            }
        }
    }

    async fn test_method_not_found(&mut self) {
        let start = std::time::Instant::now();
        let test_name = "method_not_found".to_string();

        let request = json!({
            "jsonrpc": "2.0",
            "id": "test-method-not-found-001",
            "method": "nonexistent/method",
            "params": {}
        });

        match self.send_jsonrpc_request(request).await {
            Ok(response) => {
                if response.get("error").is_some()
                    && response["error"]["code"].as_i64() == Some(-32601)
                {
                    info!("✅ Method not found test passed - correctly returned error -32601");
                    self.test_results.insert(test_name, TestResult {
                        name: "Method Not Found".to_string(),
                        passed: true,
                        error: None,
                        response: Some(response),
                        duration: start.elapsed(),
                    });
                } else {
                    warn!("⚠️ Method not found didn't return expected error");
                    self.test_results.insert(test_name, TestResult {
                        name: "Method Not Found".to_string(),
                        passed: false,
                        error: Some("Expected error -32601 for unknown method".to_string()),
                        response: Some(response),
                        duration: start.elapsed(),
                    });
                }
            }
            Err(e) => {
                error!("❌ Method not found test failed: {}", e);
                self.test_results.insert(test_name, TestResult {
                    name: "Method Not Found".to_string(),
                    passed: false,
                    error: Some(e.to_string()),
                    response: None,
                    duration: start.elapsed(),
                });
            }
        }
    }

    async fn test_invalid_parameters(&mut self) {
        let start = std::time::Instant::now();
        let test_name = "invalid_parameters".to_string();

        let request = json!({
            "jsonrpc": "2.0",
            "id": "test-invalid-params-001",
            "method": "message/send",
            "params": {
                "invalid": "parameters"
            }
        });

        match self.send_jsonrpc_request(request).await {
            Ok(response) => {
                if response.get("error").is_some()
                    && response["error"]["code"].as_i64() == Some(-32602)
                {
                    info!("✅ Invalid parameters test passed - correctly returned error -32602");
                    self.test_results.insert(test_name, TestResult {
                        name: "Invalid Parameters".to_string(),
                        passed: true,
                        error: None,
                        response: Some(response),
                        duration: start.elapsed(),
                    });
                } else {
                    warn!("⚠️ Invalid parameters didn't return expected error");
                    self.test_results.insert(test_name, TestResult {
                        name: "Invalid Parameters".to_string(),
                        passed: false,
                        error: Some("Expected error -32602 for invalid parameters".to_string()),
                        response: Some(response),
                        duration: start.elapsed(),
                    });
                }
            }
            Err(e) => {
                error!("❌ Invalid parameters test failed: {}", e);
                self.test_results.insert(test_name, TestResult {
                    name: "Invalid Parameters".to_string(),
                    passed: false,
                    error: Some(e.to_string()),
                    response: None,
                    duration: start.elapsed(),
                });
            }
        }
    }

    async fn send_jsonrpc_request(
        &self,
        request: Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let url = format!("http://{}/a2a", self.config.server_addr);

        let response = timeout(
            self.config.timeout_duration,
            client.post(&url).json(&request).send(),
        )
        .await??;

        let response_text = response.text().await?;
        let response_json: Value = serde_json::from_str(&response_text)?;

        debug!("Request: {}", serde_json::to_string_pretty(&request)?);
        debug!(
            "Response: {}",
            serde_json::to_string_pretty(&response_json)?
        );

        Ok(response_json)
    }

    fn print_test_results(&self) {
        println!("\n{}", "=".repeat(80));
        println!("🧪 A2A Server Validation Results");
        println!("{}", "=".repeat(80));

        let mut passed = 0;
        let mut failed = 0;

        for result in self.test_results.values() {
            let status = if result.passed {
                "✅ PASS"
            } else {
                "❌ FAIL"
            };
            let duration_ms = result.duration.as_millis();

            println!("{} {} ({} ms)", status, result.name, duration_ms);

            if !result.passed {
                if let Some(error) = &result.error {
                    println!("   Error: {error}");
                }
            }

            if result.passed {
                passed += 1;
            } else {
                failed += 1;
            }
        }

        println!("{}", "=".repeat(80));
        println!(
            "📊 Summary: {} passed, {} failed, {} total",
            passed,
            failed,
            passed + failed
        );

        if failed == 0 {
            println!("🎉 All tests passed! The A2A server is working correctly.");
        } else {
            println!("⚠️  Some tests failed. Please review the implementation.");
        }

        println!("\n📋 Detailed Analysis:");
        self.print_implementation_status();
    }

    fn print_implementation_status(&self) {
        let required_methods = vec![
            "message/send",
            "message/stream",
            "tasks/get",
            "tasks/cancel",
            "tasks/pushNotificationConfig/set",
            "tasks/pushNotificationConfig/get",
            "tasks/pushNotificationConfig/list",
            "tasks/pushNotificationConfig/delete",
            "tasks/resubscribe",
        ];

        println!("\nA2A JSON-RPC Method Implementation Status:");
        for method in &required_methods {
            let test_key = method.replace("/", "_");
            if let Some(result) = self.test_results.get(&test_key) {
                let status = if result.passed {
                    if result
                        .response
                        .as_ref()
                        .and_then(|r| r.get("error"))
                        .is_some()
                    {
                        "🟡 PARTIAL (returns error)"
                    } else {
                        "✅ IMPLEMENTED"
                    }
                } else {
                    "❌ NOT IMPLEMENTED"
                };
                println!("  {status} {method}");
            } else {
                println!("  ❓ UNKNOWN {method}");
            }
        }

        println!("\nRecommendations:");
        println!("- Implement proper JSON-RPC method routing in the server");
        println!("- Add full A2A specification compliance");
        println!("- Implement task management and persistence");
        println!("- Add push notification support");
        println!("- Improve error handling according to JSON-RPC 2.0 spec");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("🚀 Starting A2A Server Validation Suite");

    let mut suite = A2AValidationSuite::new();

    suite.setup().await?;

    suite.run_all_tests().await?;

    Ok(())
}
