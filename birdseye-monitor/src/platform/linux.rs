use birdseye_common::User;
use std::collections::HashMap;
use std::os::unix::prelude::*;
use sysinfo::{System, SystemExt, Uid, UserExt};
use tracing::debug;
use walkdir::WalkDir;

fn user_to_be_user(usr: users::User) -> User {
    User::new(usr.name().to_str().unwrap_or("Unable to get username"))
}

/// Black magic fuckery to find the current GUI user in linux
pub fn get_current_user() -> Option<User> {
    // Find all the files in the /dev/pts dir, idk, who and finger mentioned them when say which
    // user was logged in where so ¯\_(ツ)_/¯, it seems to work
    let pts = WalkDir::new("/dev/pts");

    let mut users: HashMap<u32, i32> = HashMap::new();

    for entry in pts.into_iter().flatten() {
        // Get the owner of each file, this *should* be the logged in user, and add to hashmap of
        // logged in users, increase count for every user found, except root, because we don't care
        // if they are currently root, and it can mess up stuff
        if let Ok(meta) = entry.metadata() {
            if users.get_mut(&meta.uid()).map(|f| *f += 1).is_none() && meta.uid() != 0 {
                users.insert(meta.uid(), 1);
            }
        }
    }

    debug!("{:?}", users);

    // Find the "most logged in user"
    let mut users = users.iter().collect::<Vec<_>>();
    users.sort_by(|a, b| a.1.cmp(b.1));

    // Return it 👍
    users::get_user_by_uid(*users.last()?.0).map(user_to_be_user)
}
