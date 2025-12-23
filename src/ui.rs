use crate::models::{Machine, SavedMachine};
use chrono::{Local, TimeZone};
use colored::Colorize;
use dialoguer::{Input, Select};
use serde::Serialize;
use std::fs;
use std::io::Write;

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
    let now = Local::now();
    let default_display = now.format("%Y-%m-%d_%H:%M").to_string();

    loop {
        let user_input: String = Input::new()
            .with_prompt("Start date (YYYY-MM-DD HH:MM)")
            .default(default_display.clone())
            .interact()
            .unwrap();

        // Parse the user input and add timezone
        match chrono::NaiveDateTime::parse_from_str(&user_input.replace(' ', "T"), "%Y-%m-%dT%H:%M")
        {
            Ok(parsed) => {
                // Convert to local timezone and format as ISO 8601
                return Local.from_local_datetime(&parsed).unwrap().to_rfc3339();
            }
            Err(_) => {
                println!(
                    "{}",
                    "Invalid date format! Please use YYYY-MM-DD HH:MM (e.g., 2025-12-23 14:30)"
                        .red()
                );
                continue;
            }
        }
    }
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
                println!(
                    "{}",
                    "Invalid input. Please type 'accept' or 'cancel'.".red()
                );
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
    println!(
        "  {}: Bearer ********{}",
        "Authorization".bold(),
        &api_key[api_key.len().saturating_sub(4)..]
    );

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

pub fn display_machines(machines: &[Machine]) {
    println!("{}", "=".repeat(120));
    println!(
        "{:<12} {:<20} {:<12} {:<12} {:<15} {:<12} {:<15}",
        "Machine ID".bold(),
        "Hostname".bold(),
        "GPU Temp".bold(),
        "Reliability".bold(),
        "GPU Occupancy".bold(),
        "Earning".bold(),
        "Driver Ver".bold()
    );
    println!("{}", "=".repeat(120));

    for machine in machines {
        // Format GPU occupancy as "rented/total"
        let gpu_occupancy_display = machine
            .gpu_occupancy
            .as_deref()
            .map(|occ| {
                let gpus: Vec<&str> = occ.split_whitespace().collect();
                let total = gpus.len();
                let rented = gpus
                    .iter()
                    .filter(|&&g| g.to_uppercase() == "D" || g.to_uppercase() == "I")
                    .count();
                format!("{}/{}", rented, total)
            })
            .unwrap_or_else(|| "N/A".to_string());

        println!(
            "{:<12} {:<20} {:<12} {:<12} {:<15} {:<12} {:<15}",
            machine.machine_id,
            machine.hostname,
            machine
                .gpu_max_cur_temp
                .map_or("N/A".to_string(), |t| format!("{:.1}°C", t)),
            machine
                .reliability2
                .map_or("N/A".to_string(), |r| format!("{:.2}%", r * 100.0)),
            gpu_occupancy_display,
            machine
                .earn_hour
                .map_or("N/A".to_string(), |e| format!("${:.4}/h", e)),
            machine.driver_version.as_deref().unwrap_or("N/A")
        );
    }
    println!("{}", "=".repeat(120));
    println!();
}

pub fn save_machines_to_file(machines: &[Machine]) -> std::io::Result<()> {
    let saved_machines: Vec<SavedMachine> = machines
        .iter()
        .map(|m| SavedMachine {
            machine_id: m.machine_id,
            hostname: m.hostname.clone(),
        })
        .collect();

    let json = serde_json::to_string_pretty(&saved_machines)?;
    let mut file = fs::File::create(".machines")?;
    file.write_all(json.as_bytes())?;

    println!(
        "{} Saved {} machines to {}",
        "Success:".green(),
        machines.len(),
        ".machines".cyan()
    );
    Ok(())
}

pub fn load_machines_from_file() -> Option<Vec<SavedMachine>> {
    match fs::read_to_string(".machines") {
        Ok(content) => serde_json::from_str(&content).ok(),
        Err(_) => None,
    }
}

pub fn prompt_machine_id_with_saved(id: Option<u64>) -> u64 {
    if let Some(id) = id {
        return id;
    }

    // Try to load saved machines
    if let Some(machines) = load_machines_from_file() {
        if !machines.is_empty() {
            let mut options: Vec<String> = machines
                .iter()
                .map(|m| format!("{} - {}", m.machine_id, m.hostname))
                .collect();
            options.push("Enter custom machine ID".to_string());

            let selection = Select::new()
                .with_prompt("Select a machine")
                .items(&options)
                .default(0)
                .interact()
                .unwrap();

            // If user selected a saved machine
            if selection < machines.len() {
                return machines[selection].machine_id;
            }
        }
    }

    // Fall back to manual input
    Input::<u64>::new()
        .with_prompt("Machine ID")
        .interact()
        .unwrap()
}
