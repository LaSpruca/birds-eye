use birdseye_common::{Process, User};
use std::collections::HashMap;
use sysinfo::{PidExt, ProcessExt, System, SystemExt, UserExt};
use tokio::sync::mpsc;

/// Function to convert sysinfo process to my process
fn sysinfo_to_be_process(process: &sysinfo::Process, sys: &System) -> Process {
    // Get the current user
    let usr = process
        .user_id()
        .map(|id| {
            sys.get_user_by_id(id)
                .map(|user| user.name())
                .unwrap_or("Unknown user")
        })
        .unwrap_or_else(|| "Unknown user");

    // Convert sysinfo's process to my process
    Process::new(process.pid().as_u32(), process.name(), &User::new(usr))
}

/// Struct to represent weather or not a process has started or stopped
/// Stores name and process id
#[derive(Debug)]
pub enum ProcessStatus {
    Start(Process),
    Stop(Process),
}

/// Start a process to monitor the running processes on the system and notify over a tokio mpsc channel
pub fn monitor_processes() -> mpsc::Receiver<ProcessStatus> {
    let (tx, rx) = mpsc::channel(5);

    tokio::spawn(async move {
        let mut sys = System::default();
        let mut processes = HashMap::new();

        loop {
            // Refresh sys struct to make sure all the things and stuff are up-to-date
            sys.refresh_processes();
            sys.refresh_users_list();

            let running_processes = sys.processes();
            let processes2 = processes.clone();

            // Find all process that have just started, store them and report back to system
            for (pid, process) in running_processes
                .iter()
                .filter(|(key, _)| !processes2.contains_key(*key))
            {
                let process = sysinfo_to_be_process(process, &sys);

                // Record information and send result
                processes.insert(*pid, process.clone());
                tx.send(ProcessStatus::Start(process)).await.unwrap();
            }

            let processes2 = processes.clone();

            // Find all process that have just stoped, store them and report back to system
            for (pid, process) in processes2
                .iter()
                .filter(|(key, _)| !running_processes.contains_key(*key))
            {
                // Record information and send result
                processes.remove(pid);
                tx.send(ProcessStatus::Stop(process.clone())).await.unwrap();
            }
        }
    });

    rx
}

/// Get all the processes running on the current system
#[allow(dead_code)]
pub fn get_all_processes() -> Vec<Process> {
    let mut sys = System::default();
    sys.refresh_processes();
    sys.refresh_users_list();
    sys.processes()
        .iter()
        .map(|(_, x)| sysinfo_to_be_process(x, &sys))
        .collect()
}
