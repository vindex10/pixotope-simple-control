use crate::common::{get_pixotope_property, set_pixotope_property};
use std::collections::HashMap;
use std::sync::LazyLock;

pub type InputOutput = String;
pub type InputOutputs = HashMap<&'static str, &'static str>;

pub static INPUT_OUTPUTS: LazyLock<InputOutputs> = LazyLock::new(|| {
    HashMap::from([
        ("AJA", "AJA"),
        ("BMD", "BMD"),
        ("NDI", "NDI"),
        ("SRT", "SRT"),
        ("Webcam", "Webcam"),
        ("File", "File (Experimental)"),
    ])
});

#[tauri::command]
pub fn set_input_output(input_output: InputOutput) -> Result<(), String> {
    if !INPUT_OUTPUTS.contains_key(&input_output.as_str()) {
        return Err("Input output not found".to_string());
    }
    set_pixotope_property("State.Defaults.Type", &input_output, "Store");
    Ok(())
}


pub fn get_input_output() -> InputOutput {
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