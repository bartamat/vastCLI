use colored::Colorize;
use dialoguer::{Input, Select};
use serde::Serialize;

pub fn print_header(title: &str) {
    println!("{}", format!("=== {} ===", title).cyan().bold());
    println!();
}

pub fn prompt_machine_id(id: Option<u64>) -> u64 {
    id.unwrap_or_else(|| {
        Input::<u64>::new()
            .with_prompt("Machine ID")
            .interact()
            .unwrap()
    })
}

pub fn prompt_start_date() -> String {
    Input::new()
        .with_prompt("Start date (ISO 8601 format)")
        .default("2023-10-30T14:00:00Z".to_string())
        .interact()
        .unwrap()
}

pub fn prompt_duration() -> u32 {
    Input::new()
        .with_prompt("Duration (hours)")
        .default(2)
        .interact()
        .unwrap()
}

pub fn prompt_maintenance_reason() -> String {
    Input::new()
        .with_prompt("Maintenance reason")
        .default("Routine hardware check".to_string())
        .interact()
        .unwrap()
}

pub fn prompt_maintenance_category() -> String {
    let categories = vec!["software", "hardware", "network", "other"];
    let selection = Select::new()
        .with_prompt("Maintenance category")
        .items(&categories)
        .default(0)
        .interact()
        .unwrap();

    categories[selection].to_string()
}

pub fn show_preview<T: Serialize>(title: &str, request: &T, machine_id: u64) {
    println!();
    println!("{}", format!("{}:", title).yellow().bold());
    println!("  {}: {}", "Machine ID".bold(), machine_id);
    println!();

    let json_preview = serde_json::to_string_pretty(request).unwrap();
    for line in json_preview.lines() {
        println!("  {}", line);
    }
    println!();
}

pub fn show_list_preview(api_key: &str) {
    println!();
    println!("{}", "Preview:".yellow().bold());
    println!("  {}: {}", "Endpoint".bold(), "GET /instances");
    println!("  {}: ********{}", "API Key".bold(), &api_key[api_key.len().saturating_sub(4)..]);
    println!();
}

pub fn confirm_action() -> bool {
    loop {
        let confirmation: String = Input::new()
            .with_prompt("Type 'accept' to confirm or 'cancel' to abort")
            .interact()
            .unwrap();

        match confirmation.to_lowercase().as_str() {
            "accept" => return true,
            "cancel" => return false,
            _ => {
                println!("{}", "Invalid input. Please type 'accept' or 'cancel'.".red());
                continue;
            }
        }
    }
}

pub fn print_cancelled() {
    println!("{}", "Request cancelled.".red());
}

pub fn print_executing() {
    println!();
    println!("{}", "Executing request...".green());
}

pub fn print_request_details(method: &str, url: &str, api_key: &str, body: Option<&str>) {
    println!();
    println!("{}", "Request details:".cyan());
    println!("  {}: {}", "Method".bold(), method);
    println!("  {}: {}", "URL".bold(), url);
    println!("  {}: Bearer ********{}", "Authorization".bold(), &api_key[api_key.len().saturating_sub(4)..]);

    if let Some(json_body) = body {
        println!("  {}:", "Body".bold());
        for line in json_body.lines() {
            println!("    {}", line);
        }
    }
    println!();
}

pub fn print_response(status: &str, body: Option<String>) {
    println!("{}", "Response:".green().bold());
    println!("  {}: {}", "Status".bold(), status);

    if let Some(response_body) = body {
        if !response_body.is_empty() {
            println!("  {}:", "Body".bold());
            for line in response_body.lines() {
                println!("    {}", line);
            }
        }
    }
}

pub fn print_error(error: &str) {
    println!("{}: {}", "Request failed".red().bold(), error);
}
