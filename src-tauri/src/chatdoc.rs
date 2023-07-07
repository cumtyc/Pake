use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Debug, Default, PartialEq, Clone, Deserialize, Serialize)]
pub struct ChatDOCConfig {
    pub sites: Vec<String>,
    pub update_sites: Vec<String>,
}

fn site_avaliable(url: &str) -> bool {
    if let Ok(resp) = reqwest::blocking::get(url) {
        if resp.status().is_success() {
            return true
        }
    }
    return false
}

pub fn get_avaliable_site() -> String {
    let config_file = include_str!("../chatdoc.json");
    let config: ChatDOCConfig = serde_json::from_str(config_file).expect("failed to parse chatdoc config");
    for site in &config.sites {
        if site_avaliable(&site) {
            return site.clone()
        }
    }
    let mut update_site = "";
    for site in &config.update_sites {
        if site_avaliable(&site) {
            update_site = site.as_str();
        }
    }
    if update_site.len() > 0{
        let response = reqwest::blocking::get(update_site).expect("failed to get avaliable update site").text().expect("wrong sites format");
        let sites: Vec<&str> = response.split("\n").collect();
        for site in sites {
            if site_avaliable(&site) {
                return site.to_string()
            }
        }
    }
    return config.sites[0].clone()
}