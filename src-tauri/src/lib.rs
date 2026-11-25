mod commands;
pub mod game_data;
mod overlay;
pub mod planner;
mod plans;
mod settings;
mod state;

use commands::{data, planning, plans as plan_cmds};
use state::{lock, AppState};
use tauri::{App, Manager, WindowEvent};
use tauri_plugin_window_state::StateFlags;

const PLANNER: &str = "planner";

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let paths = app.path();
    let state = AppState::load(&paths.app_config_dir()?, &paths.app_data_dir()?)?;
    let hotkeys = lock(&state.config).hotkeys.clone();
    app.manage(state);
    let errors = overlay::register_hotkeys(app.handle(), &hotkeys);
    *lock(&app.state::<AppState>().hotkey_errors) = errors;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_state_flags(StateFlags::SIZE | StateFlags::POSITION)
                .build(),
        )
        .setup(setup)
        .on_window_event(|window, event| {
            if window.label() == PLANNER && matches!(event, WindowEvent::CloseRequested { .. }) {
                window.app_handle().exit(0);
            }
        })
        .invoke_handler(tauri::generate_handler![
            data::load_game_data,
            data::set_docs_path,
            data::list_targets,
            data::list_raw_resources,
            data::list_alternates,
            data::set_alternate,
            data::extractor_options,
            planning::calculate,
            plan_cmds::list_plans,
            plan_cmds::save_plan,
            plan_cmds::delete_plan,
            plan_cmds::set_active_plan,
            plan_cmds::adjust_step,
            plan_cmds::mark_next_step,
            overlay::overlay_interactive,
            overlay::hotkey_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
