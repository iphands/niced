use std::fs::File;
use std::{thread, fs};
use std::time::{Duration};

struct ProcInfo {
    comm: String,
    pid: i32,
}

struct NiceItem {
    comm: String,
    target: i32,
}

fn try_renice(proc_info: &ProcInfo, niceness: i32, name: &String) -> bool {
    if proc_info.comm.starts_with(name) {
        // println!("renicing {}", proc_info.comm);
        do_tasks(proc_info.pid, niceness);
        unsafe { libc::setpriority(libc::PRIO_PROCESS, proc_info.pid as u32, niceness); }
        return true;
    }
    return false;
}

fn do_renicing(map: &Vec<NiceItem>, procs: Vec<ProcInfo>) {
    for prc in procs {
        for i in map {
            if try_renice(&prc, i.target, &i.comm) { continue };
        }
    }
}

fn do_tasks(pid: i32, niceness: i32) {
    // println!("Doing tasks for pid: {}", pid);
    let pid_path = format!("/proc/{}/task", pid);
    // println!("  path is {}", pid_path);

    for dir_entry in fs::read_dir(pid_path).unwrap() {
        let path = dir_entry.unwrap().path().display().to_string();
        let task_pid = path.split('/').collect::<Vec<&str>>()[4];

        if task_pid != format!("{}", pid) {
            // println!("  renicing task_pid: {}", task_pid);
            unsafe { libc::setpriority(libc::PRIO_PROCESS, task_pid.parse::<u32>().unwrap(), niceness); }
        }
    }
}

fn get_procs() -> std::vec::Vec<ProcInfo> {
    let mut procs = Vec::new();

    for dir_entry in fs::read_dir("/proc").unwrap() {
        let path = dir_entry.unwrap().path().display().to_string();
        let pid = path.split('/').collect::<Vec<&str>>()[2];
        if pid.chars().nth(0).unwrap().is_ascii_digit() {
            let comm_path = String::from(&path) + "/comm";
            let comm = match fs::read_to_string(comm_path) {
                Ok(s)  => s,
                Err(_) => continue,
            };

            procs.push(ProcInfo {
                comm: String::from(comm.trim_end()),
                pid: pid.parse::<i32>().unwrap()
            });
        }
    }

    return procs;
}

fn do_config() -> std::vec::Vec<NiceItem> {
    let data: String = fs::read_to_string("/etc/niced.conf").unwrap();
    let mut map: Vec<NiceItem> = Vec::new();
    for line in data.lines() {
        if line.contains('=') {
            let items = line.split('=').collect::<Vec<&str>>();
            // println!("{:?}", items);
            if items.len() == 2 {
                map.push(NiceItem {
                    comm: String::from(items[0]),
                    target: items[1].parse::<i32>().unwrap(),
                });
            }
        }
    }

    return map;
}

fn main() {
    let delay = Duration::from_millis(10 * 1000);
    let map: Vec<NiceItem> = do_config();

    loop {
        do_renicing(&map, get_procs());
        thread::sleep(delay);
    }
}
