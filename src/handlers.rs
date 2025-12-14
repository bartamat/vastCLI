use crate::api::VastAiClient;
use crate::config::Config;
use crate::models::{MaintRequest, UnlistRequest};
use crate::ui;

pub fn handle_maint(id: Option<u64>) {
    ui::print_header("VastAI Maintenance Mode");

    let machine_id = ui::prompt_machine_id(id);
    let start_date = ui::prompt_start_date();
    let duration = ui::prompt_duration();
    let maintenance_reason = ui::prompt_maintenance_reason();
    let maintenance_category = ui::prompt_maintenance_category();

    let request = MaintRequest::new(
        start_date,
        duration,
        maintenance_reason,
        maintenance_category,
    );

    ui::show_preview("Maintenance Request", &request, machine_id);

    if !ui::confirm_action() {
        ui::print_cancelled();
        return;
    }

    let config = Config::new();
    let api_key = match config.get_api_key() {
        Ok(key) => key,
        Err(e) => {
            ui::print_error(&format!("Failed to get API key: {}", e));
            return;
        }
    };

    let client = VastAiClient::new();
    let _ = client.send_maintenance_request(&request, machine_id, &api_key);
}

pub fn handle_list() {
    ui::print_header("VastAI List Instances");

    let config = Config::new();
    let api_key = match config.get_api_key() {
        Ok(key) => key,
        Err(e) => {
            ui::print_error(&format!("Failed to get API key: {}", e));
            return;
        }
    };

    ui::show_list_preview(&api_key);

    if !ui::confirm_action() {
        ui::print_cancelled();
        return;
    }

    let client = VastAiClient::new();
    let _ = client.list_instances(&api_key);
}

pub fn handle_unlist(id: Option<u64>) {
    ui::print_header("VastAI Unlist Instance");

    let instance_id = ui::prompt_machine_id(id);
    let request = UnlistRequest::new(instance_id);

    ui::show_preview("Unlist Request", &request, instance_id);

    if !ui::confirm_action() {
        ui::print_cancelled();
        return;
    }

    let config = Config::new();
    let api_key = match config.get_api_key() {
        Ok(key) => key,
        Err(e) => {
            ui::print_error(&format!("Failed to get API key: {}", e));
            return;
        }
    };

    let client = VastAiClient::new();
    let _ = client.unlist_instance(&request, instance_id, &api_key);
}
