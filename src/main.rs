use std::{fs::{self, File}, io::{BufReader, Read}, path::Path};


struct Process {
    pid: i32,
    ppid: i32,
    name: String,
}


fn print_procs(procs: &[Process], pid: i32, level: usize) {
    for proc in procs.iter().filter(|p| p.pid == pid) {
        for _ in 1..level {
            print!("|  ");
        }
        print!("+-");
        println!("{} {}", pid, proc.name);
    }

    for proc in procs.iter().filter(|p| p.ppid == pid) {
        print_procs(procs, proc.pid, level + 1);
    }
}


fn snapshot_procs() -> Result<Vec<Process>, std::io::Error> {
    let proc_path = Path::new("/proc");
    let mut procs: Vec<Process> = Vec::new();

    let res = fs::read_dir(proc_path).and_then(|entries| {
        for entry_res in entries {
            if let Ok(filename) = entry_res?.file_name().into_string() {
                match filename.parse::<i32>() {
                    Ok(pid) => {
                        let stat_path = proc_path.join(filename).join("stat");
                        let parse_path_res = File::open(stat_path).and_then(|stat_file| {
                            let mut contents = String::new();
                            let mut buf_reader = BufReader::new(stat_file);
                            let _ = buf_reader.read_to_string(&mut contents);
                            let left_bracket = contents.find('(').unwrap();
                            let right_bracket = contents.rfind(')').unwrap();
                            let contents_split: Vec<String> = contents[right_bracket + 2..].split(" ").map(|s| s.to_string()).collect();
                            let ppid = contents_split[1].parse::<i32>().unwrap();
                            let name = &contents[left_bracket..=right_bracket];
                            procs.push(Process{pid: pid, ppid: ppid, name: name.to_string()});
                            Ok(())
                        });
                        if let Err(e) = parse_path_res {
                            eprintln!("Parse /proc/{}/stat failed, Error: {}\n", pid, e);
                        }

                    },
                    Err(_) => {},
                }
            }
        }
        Ok(())
    });

    if let Err(e) = res {
        eprintln!("Parse /proc/[pid] failed, Error:{}", e);
        return Err(e);
    }
    return Ok(procs);

}

fn main() {
    if let Ok(procs) = snapshot_procs() {
        print_procs(&procs, 1, 0);
    }

}
