use crate::commands::plans::mark_next_step;
use crate::settings::Hotkeys;
use crate::state::AppState;
use std::sync::atomic::Ordering;
use tauri::{AppHandle, Emitter, Manager, State, WebviewWindow};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

pub const OVERLAY: &str = "overlay";
const MODE_CHANGED: &str = "overlay-mode";

type Action = fn(&AppHandle);

/// Registers each hotkey; one that fails (taken by another app) is reported and skipped.
pub fn register_hotkeys(app: &AppHandle, keys: &Hotkeys) -> Vec<String> {
    let actions: [(&str, Action); 3] = [
        (&keys.toggle_mode, toggle_mode),
        (&keys.toggle_visible, toggle_visible),
        (&keys.mark_next, |app| {
            let _ = mark_next_step(app.clone());
        }),
    ];
    actions
        .into_iter()
        .filter_map(|(key, action)| {
            app.global_shortcut()
                .on_shortcut(key, move |app, _, event| {
                    if event.state == ShortcutState::Pressed {
                        action(app);
                    }
                })
                .err()
                .map(|e| format!("Hotkey {key} not registered: {e}"))
        })
        .collect()
}

fn overlay_window(app: &AppHandle) -> Option<WebviewWindow> {
    app.get_webview_window(OVERLAY)
}

/// Switches between click-through (passive) and clickable (interactive).
fn toggle_mode(app: &AppHandle) {
    let state = app.state::<AppState>();
    let interactive = !state.overlay_interactive.fetch_xor(true, Ordering::SeqCst);
    if let Some(window) = overlay_window(app) {
        let _ = window.set_ignore_cursor_events(!interactive);
        let _ = window.emit(MODE_CHANGED, interactive);
    }
}

fn toggle_visible(app: &AppHandle) {
    let Some(window) = overlay_window(app) else { return };
    match window.is_visible() {
        Ok(true) => window.hide(),
        _ => window.show(),
    }
    .unwrap_or_default();
}

#[tauri::command]
pub fn overlay_interactive(state: State<AppState>) -> bool {
    state.overlay_interactive.load(Ordering::SeqCst)
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HotkeyStatus {
    keys: Hotkeys,
    errors: Vec<String>,
}

#[tauri::command]
pub fn hotkey_status(state: State<AppState>) -> HotkeyStatus {
    HotkeyStatus {
        keys: crate::state::lock(&state.config).hotkeys.clone(),
        errors: crate::state::lock(&state.notices).clone(),
    }
}
