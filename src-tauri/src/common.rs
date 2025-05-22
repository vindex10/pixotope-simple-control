use std::collections::HashSet;
use tauri_plugin_http::reqwest;
use std::sync::LazyLock;

pub static PIXOTOPE_ENDPOINT: LazyLock<String> = LazyLock::new(|| {
    match std::env::var("PIXOTOPE_ENDPOINT") {
        Ok(endpoint) => {println!("{}", &endpoint.as_str()); endpoint},
        Err(_) => "http://127.0.0.1:16208/gateway/25.1.1".to_string(),
    }
});
pub static PIXOTOPE_INSTALLATION: LazyLock<String> = LazyLock::new(|| {
    match std::env::var("PIXOTOPE_INSTALLATION") {
        Ok(installation) => installation,
        Err(_) => "C:\\Pixotope\\25.1.1.13725".to_string(),
    }
});
pub static POLLING_INTERVAL: u64 = 100;


pub fn get_pixotope_property(property: &str, target: &str) -> String {
    let url = format!(
        "{}/publish?Type=Get&Name={}&Target={}",
        PIXOTOPE_ENDPOINT.as_str(), property, target
    );
    let response = reqwest::blocking::get(url).unwrap();
    response.text().unwrap()
}

pub fn set_pixotope_property(property: &str, value: &str, target: &str) {
    let url = format!(
        "{}/publish?Type=Set&Name={}&Value={}&Target={}",
        PIXOTOPE_ENDPOINT.as_str(), property, value, target
    );
    reqwest::blocking::get(url).unwrap();
}

pub fn set_if_changed(orig: &str, new: &str) -> Option<String> {
    if orig == new {
        return None;
    }
    Some(new.to_string())
}

pub fn set_vec_if_changed(orig: Vec<String>, new: Vec<String>) -> Option<Vec<String>> {
    if HashSet::<String>::from_iter(orig).symmetric_difference(&HashSet::<String>::from_iter(new.clone())).count() == 0 {
        return None;
    }
    Some(new)
}