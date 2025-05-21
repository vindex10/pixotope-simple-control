
use crate::common::{get_pixotope_property, set_pixotope_property, PIXOTOPE_INSTALLATION};
use std::collections::HashSet;
use tauri::State;
use crate::state::AppStateMutex;

#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ColorSpaceEntry {
    pub family: String,
    pub name: String,
}
pub type ColorSpacesNames = HashSet<String>;
pub type ColorSpace = String;

#[tauri::command]
pub fn set_color_space(
    app_state_mutex: State<'_, AppStateMutex>,
    color_space: ColorSpace,
) -> Result<(), String> {
    let app_state = app_state_mutex.lock().unwrap();
    if !app_state.color_spaces_names.contains(&color_space) {
        return Err("Color space not found".to_string());
    }
    set_pixotope_property("State.Defaults.ColorSpace", &color_space, "Store");
    Ok(())
}

pub fn get_color_space() -> ColorSpace {
    #[derive(serde::Deserialize)]
    struct ColorSpaceResponse {
        Message: ColorSpaceMessageResponse,
    }
    #[derive(serde::Deserialize)]
    struct ColorSpaceMessageResponse {
        Value: String,
    }
    let response = get_pixotope_property("State.Defaults.ColorSpace", "Store");
    // response is a one element array, unpack by removing the brackets
    let color_space: ColorSpaceResponse =
        serde_json::from_str(&response[1..response.len() - 1]).unwrap();
    color_space.Message.Value
}

pub fn get_color_spaces() -> Vec<ColorSpaceEntry> {
    let path = format!(
        "{}\\Services\\VideoIO\\ocio-configs\\aces_1.1\\config.ocio",
        PIXOTOPE_INSTALLATION.as_str()
    );
    let config_str = std::fs::read_to_string(path).unwrap();
    let mut color_spaces = Vec::<ColorSpaceEntry>::new();
    let mut current_space = None;
    for line in config_str.lines() {
        let line = line.trim();
        if line.starts_with("- !<ColorSpace>") {
            if let Some(space) = current_space {
                color_spaces.push(space);
            }
            current_space = Some(ColorSpaceEntry {
                name: String::new(),
                family: String::new(),
            });
        } else if let Some(space) = &mut current_space {
            if line.starts_with("name:") {
                space.name = line.split(':').nth(1).unwrap().trim().to_string();
            } else if line.starts_with("family:") {
                space.family = line.split(':').nth(1).unwrap().trim().to_string();
            }
        }
    }
    if let Some(space) = current_space {
        color_spaces.push(space);
    }
    color_spaces
}