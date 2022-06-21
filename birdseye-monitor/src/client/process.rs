use futures::{stream, StreamExt};
use serde::Serialize;
use std::collections::HashMap;
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt, UserExt};
use tokio::sync::mpsc;
use tracing::debug;

/// Serializable process struct to describe process used inside of crate
#[derive(Clone, Serialize, Debug)]
pub struct Process {
    pid: u32,
    name: String,
    user: String,
}

impl From<(&sysinfo::Process, &sysinfo::System)> for Process {
    fn from((info, sys): (&sysinfo::Process, &sysinfo::System)) -> Self {
        Self {
            name: info.name().to_string(),
            pid: info.pid().as_u32(),
            user: info
                .user_id()
                .map(|uid| {
                    sys.get_user_by_id(uid)
                        .map(|f| f.name().to_string())
                        .unwrap_or_else(|| uid.to_string())
                })
                .unwrap_or_else(|| "No username".to_string()),
        }
    }
}

/// Struct to represent weather or not a process has started or stopped
/// Stores name and process id
#[derive(Debug)]
pub enum ProcessStatus {
    Start(Process),
    Stop(Process),
}

impl ProcessStatus {
    pub fn process(&self) -> &Process {
        match self {
            Self::Stop(a) => a,
            Self::Start(a) => a,
        }
    }
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
                // Convert sysinfo's process to my process
                let process = Process::from((process, &sys));

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

pub fn get_all_processes() -> Vec<Process> {
    let mut sys = System::default();
    sys.refresh_processes();
    sys.processes()
        .iter()
        .map(|(_, x)| Process::from((x, &sys)))
        .collect()
}
