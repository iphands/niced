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

        let pid = proc_info.pid as u32;
        unsafe {
            if niceness != libc::getpriority(libc::PRIO_PROCESS, pid) {
                libc::setpriority(libc::PRIO_PROCESS, pid, niceness);
            }
        }

        do_tasks(proc_info.pid, niceness);
        return true;
    }

    return false;
}

fn do_renicing(map: &Vec<NiceItem>, procs: Vec<ProcInfo>) {
    procs.iter().for_each(|prc| {
        map.iter().find(|i| {
            // Note find breaks this loop if the return is true
            try_renice(&prc, i.target, &i.comm)
        });
    });
}

fn do_tasks(pid: i32, niceness: i32) {
    // println!("Doing tasks for pid: {}", pid);
    let pid_path = format!("/proc/{}/task", pid);
    // println!("  path is {}", pid_path);

    let dir_reader: fs::ReadDir = match fs::read_dir(&pid_path) {
        Ok(r)  => r,
        Err(_) => return,
    };

    dir_reader.for_each(|dir_entry| {
        let entry: fs::DirEntry = match dir_entry {
            Ok(r)  => r,
            Err(_) => return,
        };

        let path = entry.path().into_os_string().into_string().unwrap();
        let task_pid = path.split('/').collect::<Vec<&str>>()[4];

        if task_pid != format!("{}", pid) {
            // println!("  renicing task_pid: {}", task_pid);
            let pid = task_pid.parse::<u32>().unwrap();
            unsafe {
                if niceness != libc::getpriority(libc::PRIO_PROCESS, pid) {
                    libc::setpriority(libc::PRIO_PROCESS, pid, niceness);
                }
            }
        }
    });
}

fn get_procs() -> std::vec::Vec<ProcInfo> {
    let mut procs = Vec::new();

    fs::read_dir("/proc").unwrap().for_each(|dir_entry| {
        let entry: fs::DirEntry = match dir_entry {
            Ok(r)  => r,
            Err(_) => return,
        };

        let path = entry.path().into_os_string().into_string().unwrap();

        if path.bytes().nth(6).unwrap().is_ascii_digit() {
            let pid = &path[6..];
            let comm_path = format!("{}/comm", &path);
            let comm = match fs::read_to_string(comm_path) {
                Ok(s)  => s,
                Err(_) => return,
            };

            procs.push(ProcInfo {
                comm: String::from(comm.trim_end()),
                pid: pid.parse::<i32>().unwrap()
            });
        }
    });

    return procs;
}

fn do_config() -> std::vec::Vec<NiceItem> {
    let data: String = fs::read_to_string("/etc/niced.conf").unwrap();
    let mut map: Vec<NiceItem> = Vec::new();

    data.lines().for_each(|line| {
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
    });

    return map;
}

fn main() {
    let delay = Duration::from_millis(15 * 1000);
    let map: Vec<NiceItem> = do_config();

    loop {
        // let now = Instant::now();
        // for _ in 0..100 {
        do_renicing(&map, get_procs());
        // }
        // println!("nanos: {}", now.elapsed().as_nanos());
        thread::sleep(delay);
    }
}
