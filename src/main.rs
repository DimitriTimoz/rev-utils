use std::path::{Path, PathBuf};

use libproc::proc_pid::{name, pidcwd, pidinfo};
use libproc::task_info::TaskAllInfo;
use libproc::processes;
use libproc::processes::ProcFilter;

struct ProcInfo {
    pid: i32,
    name: String,
    path: Option<PathBuf>,
    parent: Option<i32>,
    children: Vec<i32>,
}
impl ProcInfo {
    fn new(pid: i32, name: String) -> Self {
        let path = pidcwd(pid).ok();

        Self { 
            pid, 
            name, 
            path,
            children: Vec::new(),
            parent: None,
        }
    }

    fn get_pid_info(&self) -> Option<TaskAllInfo> {
        pidinfo::<TaskAllInfo>(self.pid, 0)
            .ok()
    }
}

impl std::fmt::Display for ProcInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PID: {:<}", self.pid)
            .and_then(|_| write!(f, ", Name: {}", self.name))
            .and_then(|_| {
                // Be sure to check if the path is not empty
                if let Some(path) = &self.path {
                    if path != Path::new("") {
                        write!(f, ", Path: {}", path.display())
                    } else {
                        Ok(())
                    }
                } else {
                    Ok(())
                }
            })
    }
}


fn main() {
    let mut pids = processes::pids_by_type(ProcFilter::All).unwrap();
    pids.sort();
    for pid in pids {
        let name = name(pid.try_into().unwrap()).unwrap_or_else(|_| "Unknown".to_string());
        let proc_info = ProcInfo::new(pid as i32, name.clone());
        println!("{}", proc_info);
        proc_info.get_pid_info();
    }
}
