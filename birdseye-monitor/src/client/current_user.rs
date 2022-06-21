use users::get_current_username;

#[cfg(target_os = "windows")]
fn monitor_current_user() {}

#[cfg(target_os = "linux")]
fn monitor_current_user() {}
