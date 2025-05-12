use std::path::{Path, PathBuf};

use libproc::proc_pid::{name, pidcwd, ListThreads};
use libproc::{proc_pid, processes};
use libproc::processes::ProcFilter;

struct ProcInfo {
    pid: i32,
    name: String,
    path: Option<PathBuf>,
}
impl ProcInfo {
    fn new(pid: i32, name: String) -> Self {
        let path = pidcwd(pid).ok();

        Self { pid, name, path }
    }
}

impl std::fmt::Display for ProcInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PID: {:<}, Name: {}", self.pid, self.name)
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
        let proc_info = proc_pid::listpidinfo::<ListThreads>(pid as i32, 128);
    }
}
