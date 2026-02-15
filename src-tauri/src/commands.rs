use crate::hardware;

#[tauri::command]
pub fn get_hardware_info() -> Result<hardware::HardwareInfo, String> {
    Ok(hardware::collect_hardware_info())
}

#[tauri::command]
pub fn get_hardware_live() -> Result<hardware::LiveInfo, String> {
    Ok(hardware::collect_live_info())
}
