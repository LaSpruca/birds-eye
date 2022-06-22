//! Windows specific implementatinos for common activities
use birdseye_common::User;
use wmi::{COMLibrary, WMIConnection};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename = "Win32_ComputerSystem")]
#[serde(rename_all = "PascalCase")]
struct UserQuery {
    user_name: String,
}

impl Into<User> for &UserQuery {
    fn into(self) -> User {
        User::new(&self.user_name)
    }
}

pub fn get_current_user() -> Option<User> {
    let com_con = COMLibrary::new().ok()?;
    let wmi_con = WMIConnection::new(com_con.into()).ok()?;

    let users_query: Vec<UserQuery> = wmi_con.query().ok()?;

    Some(users_query.first()?.into())
}
