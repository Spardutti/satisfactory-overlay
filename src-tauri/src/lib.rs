mod commands;
pub mod game_data;
mod layout;
pub mod planner;
mod plans;
mod settings;
mod state;

use commands::{data, layout as layout_cmds, planning, plans as plan_cmds, unlocks};
use state::AppState;
use tauri::{App, Manager, WebviewWindowBuilder};
use tauri_plugin_window_state::StateFlags;

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let paths = app.path();
    app.manage(AppState::load(&paths.app_config_dir()?, &paths.app_data_dir()?));
    open_windows(app)
}

/// Windows open only after the state is managed; Tauri would open them before setup runs.
fn open_windows(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    for config in &app.config().app.windows {
        WebviewWindowBuilder::from_config(app.handle(), config)?.build()?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_state_flags(StateFlags::SIZE | StateFlags::POSITION)
                .build(),
        )
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            data::load_game_data,
            data::set_docs_path,
            data::list_targets,
            data::list_raw_resources,
            data::list_alternates,
            data::set_alternate,
            unlocks::list_unlocks,
            unlocks::set_unlock,
            unlocks::set_tier,
            data::extractor_options,
            planning::calculate,
            layout_cmds::plan_layout,
            layout_cmds::move_block,
            layout_cmds::reset_layout,
            plan_cmds::set_nodes,
            plan_cmds::fill_miners,
            plan_cmds::set_depot,
            plan_cmds::list_plans,
            plan_cmds::save_plan,
            plan_cmds::delete_plan,
            plan_cmds::toggle_built,
            plan_cmds::clear_built,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
