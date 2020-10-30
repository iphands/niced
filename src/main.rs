use std::{thread, fs};
use std::time::{Duration};

struct ProcInfo {
    comm: String,
    pid: i32,
}

fn try_renice(proc_info: &ProcInfo, niceness: i32, name: &str) -> bool {
    if proc_info.comm.starts_with(name) {
        // println!("renicing {}", proc_info.comm);
        unsafe { libc::setpriority(libc::PRIO_PROCESS, proc_info.pid as u32, niceness); }
        return true;
    }
    return false;
}

fn do_renicing(procs: std::vec::Vec<ProcInfo>) {
    for prc in procs {
        if try_renice(&prc, -19, "qemu") { continue };

        if try_renice(&prc, -10, "X") { continue };
        if try_renice(&prc, -10, "compton") { continue };
        if try_renice(&prc, -10, "fluxbox") { continue };

        if try_renice(&prc, -5,  "chome") { continue };
        if try_renice(&prc, -5,  "nacl") { continue };
        if try_renice(&prc, -5,  "pulseaudio") { continue };
        if try_renice(&prc, -5,  "term") { continue };
        if try_renice(&prc, -5,  "emacs") { continue };

        if try_renice(&prc, 1,  "slack") { continue };
        if try_renice(&prc, 20, "conky") { continue };
        if try_renice(&prc, 20, "xosview") { continue };
    }
}

fn get_procs() -> std::vec::Vec<ProcInfo> {
    let mut procs = Vec::new();

    for path in fs::read_dir("/proc").unwrap() {
        let path = path.unwrap().path().display().to_string();
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

fn main() {
    let delay = Duration::from_millis(10 * 1000);
    loop {
        do_renicing(get_procs());
        thread::sleep(delay);
    }
}
