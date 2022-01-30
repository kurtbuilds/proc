use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};
use users::{get_current_uid, get_user_by_uid, User};

#[derive(Debug)]
pub struct ProcessInfo {
    pub pid: Pid,
    pub name: String,
    pub owner: User,
    pub is_current_user: bool,
    pub command: Vec<String>,
}

impl From<i32> for ProcessInfo {
    fn from(pid: i32) -> Self {
        let empty = String::from("");
        let sys = System::new_all();
        let process = sys.process(Pid::from_u32(pid as u32)).unwrap();

        Self {
            pid: process.pid(),
            name: process.name().to_string(),
            command: process.cmd().to_vec(),
            owner: get_user_by_uid(process.uid).unwrap(),
            is_current_user: get_user_by_uid(process.uid).unwrap().uid() == get_current_uid(),
        }
    }
}

/// get process information for pids
pub fn get_process_info(pids: Vec<u32>) -> Vec<ProcessInfo> {
    pids.into_iter()
        .map(|pid| ProcessInfo::from(pid as i32))
        .collect()
}
