use crate::color_space::{ColorSpace, ColorSpaceEntry, ColorSpacesNames, get_color_space, get_color_spaces};
use crate::input_output::{InputOutput, InputOutputs, get_input_output, INPUT_OUTPUTS};
use crate::cameras::{Cameras, get_cameras};
use std::sync::Mutex;
use tauri::State;
use std::collections::HashSet;
use tauri::Manager;

#[derive(Default)]
pub struct AppState {
    pub color_spaces_names: ColorSpacesNames,
    pub color_space: ColorSpace,
    pub input_output: InputOutput,
    pub cameras: Cameras,
}
pub type AppStateMutex = Mutex<AppState>;

#[derive(Default, Clone, serde::Serialize)]
pub struct Updates {
    pub current_color_space: Option<ColorSpace>,
    pub current_input_output: Option<InputOutput>,
    pub cameras: Option<Cameras>,
}

#[derive(Default, Clone, serde::Serialize)]
pub struct InitState {
    pub color_spaces: Vec<ColorSpaceEntry>,
    pub color_space: ColorSpace,
    pub input_outputs: InputOutputs,
    pub input_output: InputOutput,
    pub cameras: Cameras,
}

#[tauri::command]
pub fn get_init_state(app_state_mutex: State<'_, AppStateMutex>) -> InitState {
    let mut app_state = app_state_mutex.lock().unwrap();
    let color_spaces = get_color_spaces();
    app_state.color_spaces_names =
        HashSet::from_iter(color_spaces.iter().map(|space| space.name.clone()));
    app_state.color_space = get_color_space();
    app_state.input_output = get_input_output();
    app_state.cameras = get_cameras();
    InitState {
        color_spaces: color_spaces,
        color_space: app_state.color_space.clone(),
        input_outputs: INPUT_OUTPUTS.clone(),
        input_output: app_state.input_output.clone(),
        cameras: app_state.cameras.clone(),
    }
}

pub fn get_current_state() -> (ColorSpace, InputOutput, Cameras) {
    let color_space = get_color_space();
    let input_output = get_input_output();
    let cameras = get_cameras();
    (color_space, input_output, cameras)
}

pub fn merge_state(app: tauri::AppHandle, new_state: (ColorSpace, InputOutput, Cameras)) -> Updates {
    let state = app.state::<AppStateMutex>();
    let app_state = state.lock().unwrap();
    let mut new_color_space = None;
    if new_state.0 != app_state.color_space {
        new_color_space = Some(new_state.0.clone());
    }
    let mut new_input_output = None;
    if new_state.1 != app_state.input_output {
        new_input_output = Some(new_state.1.clone());
    }
    let mut new_cameras = None;
    if new_state.2 != app_state.cameras {
        new_cameras = Some(new_state.2.clone());
    }
    Updates {
        current_color_space: new_color_space,
        current_input_output: new_input_output,
        cameras: new_cameras,
    }
}