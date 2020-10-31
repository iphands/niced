extern crate procfs;
use std::{fs, thread};
use std::time::{Instant, Duration};

struct ProcInfo {
    comm: String,
    pid: i32,
}

fn get_procs() -> std::vec::Vec<ProcInfo> {
    let mut procs = Vec::new();

    for path in fs::read_dir("/proc").unwrap() {
        let path = path.unwrap().path().display().to_string();
        let pid = path.split('/').collect::<Vec<&str>>()[2];

        // if pid.as_bytes()[0].is_ascii_digit() {
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

fn get_procs_procfs() -> std::vec::Vec<ProcInfo> {
    let mut procs = Vec::new();
    for prc in procfs::process::all_processes().unwrap() {
        procs.push(ProcInfo {
            comm: prc.stat.comm,
            pid: prc.stat.pid
        });
    }

    return procs;
}

fn warning_killer() {
    for p in get_procs() {
        println!("{} {}", p.pid, p.comm);
        assert!(p.comm.len() > 0);
        assert!(p.pid > 0);
    }

    for p in get_procs_procfs() {
        assert!(p.comm.len() > 0);
        assert!(p.pid > 0);
    }
}

fn main() {
    let delay = Duration::from_millis(250);
    warning_killer();
    loop {
        let now = Instant::now();
        for _ in 0..100 { get_procs(); }
        println!("mine:   {}", now.elapsed().as_nanos());
        thread::sleep(delay);

        let now_two = Instant::now();
        for _ in 0..100 { get_procs_procfs(); }
        println!("extern: {}", now_two.elapsed().as_nanos());
        thread::sleep(delay);
    }
}
