use std::collections::HashMap;
use std::path::{Path, PathBuf};

use libproc::bsd_info::BSDInfo;
use libproc::proc_pid::{name, pidcwd, pidinfo};
use libproc::task_info::TaskAllInfo;
use libproc::processes;
use libproc::processes::ProcFilter;

pub struct ProcInfo {
    pid: i32,
    name: String,
    path: Option<PathBuf>,
    parent: Option<i32>,
}

impl ProcInfo {
    fn new(pid: i32, name: String) -> Self {
        let path = pidcwd(pid).ok();
        // Get the parent PID
        let process_infos = pidinfo::<BSDInfo>(pid, 0);
        let parent = process_infos.map(|p| p.pbi_ppid as i32).ok();
        Self { 
            pid, 
            name, 
            path,
            parent,
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

#[derive(Default)]
pub struct ProcList {
    procs: HashMap<i32, ProcInfo>,
    relations: HashMap<i32, Vec<i32>>,
}

impl ProcList {
    pub fn add_proc(&mut self, proc_info: ProcInfo) {
        let pid = proc_info.pid;
        if let Some(parent_pid) = proc_info.parent {
            self.add_relation(parent_pid, pid);
        }
        self.procs.insert(pid, proc_info);
    }

    fn add_relation(&mut self, parent: i32, child: i32) {
        self.relations.entry(parent).or_default().push(child);
    }
}

impl std::fmt::Display for ProcList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let root = self.procs.get(&1);
        if let Some(root) = root {
            writeln!(f, "Process: {}", root)?;
            // Plot the tree
            let mut visited = HashMap::new();
            visited.insert(root.pid, true);
            
            // Use a stack that includes both the process and its depth
            let mut stack = vec![(root, 0)];
            
            while let Some((proc_info, depth)) = stack.pop() {
                if let Some(children) = self.relations.get(&proc_info.pid) {
                    // Process children in reverse to maintain proper order when using a stack
                    for &child_pid in children.iter().rev() {
                        if let Some(child_proc) = self.procs.get(&child_pid) {
                            if let std::collections::hash_map::Entry::Vacant(e) = visited.entry(child_pid) {
                                // Create proper indentation based on depth
                                let indent = "|   ".repeat(depth + 1);
                                writeln!(f, "{}└── {}", indent, child_proc)?;
                                stack.push((child_proc, depth + 1));
                                e.insert(true);
                            }
                        }
                    }
                }
            }
        } else {
            writeln!(f, "No root process found")?
        }
        Ok(())
    }
}

fn main() {
    let mut proc_list = ProcList::default();
    let mut pids = processes::pids_by_type(ProcFilter::All).unwrap();
    pids.sort();
    for pid in pids {
        let name = name(pid.try_into().unwrap()).unwrap_or_else(|_| "Unknown".to_string());
        let proc_info = ProcInfo::new(pid as i32, name.clone());
        proc_list.add_proc(proc_info);
    }
    println!("{}", proc_list);
}
