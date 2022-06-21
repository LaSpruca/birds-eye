use std::collections::HashMap;
use tracing::debug;
use users::{get_user_by_uid, User};
use walkdir::WalkDir;

#[cfg(target_os = "windows")]
fn monitor_current_user() {}

/// Black magic fuckery to find the current GUI user in linux
#[cfg(target_os = "linux")]
pub fn get_current_user() -> Option<User> {
    use std::os::unix::prelude::*;

    // Find all the files in the /dev/pts dir, idk, who and finger mentioned them when say which
    // user was logged in where so ¯\_(ツ)_/¯, it seems to work
    let pts = WalkDir::new("/dev/pts");

    let mut users: HashMap<u32, i32> = HashMap::new();

    for entry in pts.into_iter().flatten() {
        // Get the owner of each file, this *should* be the logged in user, and add to hashmap of
        // logged in users, increase count for every user found
        if let Ok(meta) = entry.metadata() {
            if users.get_mut(&meta.uid()).map(|f| *f += 1).is_none() {
                users.insert(meta.uid(), 1);
            }
        }
    }

    debug!("{:?}", users);

    // Find the "most logged in user"
    let mut users = users.iter().collect::<Vec<_>>();
    users.sort_by(|a, b| a.1.cmp(b.1));

    // Return it 👍
    get_user_by_uid(*users.last()?.0)
}

#[cfg(target_os = "mac_os")]
pub fn get_current_user() -> Option<User> {
    todo!("fuck you mac user.");
}
