use serde;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::LazyLock;
use std::sync::Mutex;
use tauri::{Emitter, Manager, State};
use tauri_plugin_http::reqwest;

static PIXOTOPE_ENDPOINT: &str = "http://127.0.0.1:16208/gateway/25.1.1";
static PIXOTOPE_INSTALLATION: &str = "C:\\Pixotope\\25.1.1.13725";

#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
struct ColorSpaceEntry {
    family: String,
    name: String,
}

type ColorSpacesNames = HashSet<String>;
type ColorSpace = String;
type InputOutput = String;
type InputOutputs = HashMap<&'static str, &'static str>;
type Cameras = Vec<String>;

static INPUT_OUTPUTS: LazyLock<InputOutputs> = LazyLock::new(|| {
    HashMap::from([
        ("AJA", "AJA"),
        ("BMD", "BMD"),
        ("NDI", "NDI"),
        ("SRT", "SRT"),
        ("Webcam", "Webcam"),
        ("File", "File (Experimental)"),
    ])
});

#[derive(Default)]
struct AppState {
    color_spaces_names: ColorSpacesNames,
    color_space: ColorSpace,
    input_output: InputOutput,
    cameras: Cameras,
}
type AppStateMutex = Mutex<AppState>;

#[derive(Default, Clone, serde::Serialize)]
struct Updates {
    current_color_space: Option<ColorSpace>,
    current_input_output: Option<InputOutput>,
    cameras: Option<Cameras>,
}

#[derive(Default, Clone, serde::Serialize)]
struct InitState {
    color_spaces: Vec<ColorSpaceEntry>,
    color_space: ColorSpace,
    input_outputs: InputOutputs,
    input_output: InputOutput,
    cameras: Cameras,
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            get_init_state,
            set_input_output,
            set_color_space
        ])
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            listen_current_state(app.get_webview_window("main").unwrap());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_init_state(app_state_mutex: State<'_, AppStateMutex>) -> InitState {
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

#[tauri::command]
fn set_color_space(
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

#[tauri::command]
fn set_input_output(input_output: InputOutput) -> Result<(), String> {
    if !INPUT_OUTPUTS.contains_key(&input_output.as_str()) {
        return Err("Input output not found".to_string());
    }
    set_pixotope_property("State.Defaults.Type", &input_output, "Store");
    Ok(())
}

fn listen_current_state(window: tauri::WebviewWindow) {
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        let new_state = get_current_state();
        let updates = merge_state(window.app_handle().clone(), new_state);
        window.emit("state-update", updates).unwrap();
    });
}

fn get_current_state() -> (ColorSpace, InputOutput, Cameras) {
    let color_space = get_color_space();
    let input_output = get_input_output();
    let cameras = get_cameras();
    (color_space, input_output, cameras)
}

fn get_color_space() -> ColorSpace {
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

fn get_input_output() -> InputOutput {
    #[derive(serde::Deserialize)]
    struct InputOutputResponse {
        Message: InputOutputMessageResponse,
    }
    #[derive(serde::Deserialize)]
    struct InputOutputMessageResponse {
        Value: String,
    }
    let response = get_pixotope_property("State.Defaults.Type", "Store");
    // response is a one element array, unpack by removing the brackets
    let input_output: InputOutputResponse =
        serde_json::from_str(&response[1..response.len() - 1]).unwrap();
    input_output.Message.Value
}

fn get_cameras() -> Cameras {
    #[derive(serde::Deserialize)]
    struct CamerasResponse {
        Message: CamerasMessageResponse,
    }
    #[derive(serde::Deserialize)]
    struct CamerasMessageResponse {
        Value: HashMap<String, Camera>,
    }
    #[derive(serde::Deserialize)]
    struct Camera {
        Name: String,
    }
    let response = get_pixotope_property("State.Cameras", "Store");
    // response is a one element array, unpack by removing the brackets
    let cameras: CamerasResponse = serde_json::from_str(&response[1..response.len() - 1]).unwrap();
    cameras
        .Message
        .Value
        .values()
        .map(|k| k.Name.clone())
        .collect()
}

fn merge_state(app: tauri::AppHandle, new_state: (ColorSpace, InputOutput, Cameras)) -> Updates {
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

fn get_pixotope_property(property: &str, target: &str) -> String {
    let url = format!(
        "{}/publish?Type=Get&Name={}&Target={}",
        PIXOTOPE_ENDPOINT, property, target
    );
    let response = reqwest::blocking::get(url).unwrap();
    response.text().unwrap()
}

fn set_pixotope_property(property: &str, value: &str, target: &str) {
    let url = format!(
        "{}/publish?Type=Set&Name={}&Value={}&Target={}",
        PIXOTOPE_ENDPOINT, property, value, target
    );
    reqwest::blocking::get(url).unwrap();
}

fn get_color_spaces() -> Vec<ColorSpaceEntry> {
    let path = format!(
        "{}\\Services\\VideoIO\\ocio-configs\\aces_1.1\\config.ocio",
        PIXOTOPE_INSTALLATION
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
