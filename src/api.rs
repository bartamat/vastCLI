use crate::models::{Machine, MachinesResponse};
use crate::ui;
use reqwest::blocking::Client;
use serde::Serialize;

const BASE_URL: &str = "https://console.vast.ai/api/v0";

pub struct VastAiClient {
    client: Client,
}

impl VastAiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn send_maintenance_request<T: Serialize>(
        &self,
        request: &T,
        machine_id: u64,
        api_key: &str,
        verbose: bool,
    ) -> Result<String, String> {
        let url = format!("{}/machines/{}/dnotify", BASE_URL, machine_id);
        let json_body = serde_json::to_string(request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        if verbose {
            ui::print_executing();
            ui::print_request_details("PUT", &url, api_key, Some(&json_body));
        }

        match self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .body(json_body)
            .send()
        {
            Ok(response) => {
                let status = response.status().to_string();
                let body = response.text().ok();
                ui::print_response(&status, body.clone());
                Ok(body.unwrap_or_default())
            }
            Err(e) => {
                ui::print_error(&e.to_string());
                Err(e.to_string())
            }
        }
    }

    pub fn list_machines(&self, api_key: &str, verbose: bool) -> Result<Vec<Machine>, String> {
        let url = format!("{}/machines", BASE_URL);

        if verbose {
            ui::print_executing();
            ui::print_request_details("GET", &url, api_key, None);
        }

        match self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
        {
            Ok(response) => {
                let status = response.status();

                if status.is_success() {
                    // Get the raw text first
                    match response.text() {
                        Ok(body) => {
                            // Try to parse as JSON
                            match serde_json::from_str::<MachinesResponse>(&body) {
                                Ok(response) => Ok(response.machines),
                                Err(e) => {
                                    ui::print_error(&format!("Failed to parse response: {}", e));
                                    Err(e.to_string())
                                }
                            }
                        }
                        Err(e) => {
                            ui::print_error(&format!("Failed to read response body: {}", e));
                            Err(e.to_string())
                        }
                    }
                } else {
                    let error_msg = format!("Request failed with status: {}", status);
                    ui::print_error(&error_msg);
                    Err(error_msg)
                }
            }
            Err(e) => {
                ui::print_error(&e.to_string());
                Err(e.to_string())
            }
        }
    }

    pub fn unlist_instance<T: Serialize>(
        &self,
        request: &T,
        instance_id: u64,
        api_key: &str,
        verbose: bool,
    ) -> Result<String, String> {
        let url = format!("{}/instances/{}/unlist", BASE_URL, instance_id);
        let json_body = serde_json::to_string(request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        if verbose {
            ui::print_executing();
            ui::print_request_details("POST", &url, api_key, Some(&json_body));
        }

        match self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .body(json_body)
            .send()
        {
            Ok(response) => {
                let status = response.status().to_string();
                let body = response.text().ok();
                ui::print_response(&status, body.clone());
                Ok(body.unwrap_or_default())
            }
            Err(e) => {
                ui::print_error(&e.to_string());
                Err(e.to_string())
            }
        }
    }
}

impl Default for VastAiClient {
    fn default() -> Self {
        Self::new()
    }
}
