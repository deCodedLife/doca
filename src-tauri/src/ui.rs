use crate::configs;

pub struct ConfigState(pub configs::Config);

#[tauri::command]
pub fn get_crm_url(cfg: tauri::State<'_, ConfigState>) -> String {
    std::format!("{}", cfg.0.crm_url.to_string())
}
