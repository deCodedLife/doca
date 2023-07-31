const CASHBOX_ID: &str = "1_yashlek";

#[tauri::command]
pub fn get_cashbox_id() -> String {
    format!( "{}", CASHBOX_ID )
}