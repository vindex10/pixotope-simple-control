use crate::common::get_pixotope_property;
use std::collections::HashMap;

pub type Cameras = Vec<String>;

pub fn get_cameras() -> Cameras {
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

