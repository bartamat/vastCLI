use crate::api::VastAiClient;
use crate::models::{MaintRequest, UnlistRequest};
use crate::ui;

pub fn handle_maint(id: Option<u64>, api_key: &str, verbose: bool) {
    ui::print_header("VastAI Maintenance Mode");

    let machine_id = ui::prompt_machine_id_with_saved(id);
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

    let client = VastAiClient::new();
    let _ = client.send_maintenance_request(&request, machine_id, api_key, verbose);
}

pub fn handle_list(api_key: &str, verbose: bool) {
    ui::print_header("VastAI List Machines");

    let client = VastAiClient::new();
    match client.list_machines(api_key, verbose) {
        Ok(machines) => {
            ui::display_machines(&machines);
            let _ = ui::save_machines_to_file(&machines);
        }
        Err(e) => {
            ui::print_error(&format!("Failed to list machines: {}", e));
        }
    }
}

pub fn handle_unlist(id: Option<u64>, api_key: &str, verbose: bool) {
    ui::print_header("VastAI Unlist Instance");

    let instance_id = ui::prompt_machine_id(id);
    let request = UnlistRequest::new(instance_id);

    ui::show_preview("Unlist Request", &request, instance_id);

    if !ui::confirm_action() {
        ui::print_cancelled();
        return;
    }

    let client = VastAiClient::new();
    let _ = client.unlist_instance(&request, instance_id, api_key, verbose);
}
