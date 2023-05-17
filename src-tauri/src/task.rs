use std::{os::windows::process::CommandExt, process::Command};

use serde::Serialize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize)]
pub enum SessionName {
    Services,
    Console,
    Others,
}

#[derive(Debug, Clone, Serialize)]
pub struct Task {
    pub process_name: String,
    pub pid: u32,
    pub session_name: SessionName,
    pub memory_usage: String,
}

impl Task {
    fn new(info: Vec<&str>) -> Self {
        let length = info.len();

        Task {
            process_name: info[..length - 5].join(" "),
            pid: match info[length - 5].parse() {
                Ok(id) => id,
                Err(e) => {
                    println!("{} | {}", info[length - 5], e);
                    0
                }
            },
            session_name: match info[length - 4] {
                "Services" => SessionName::Services,
                "Console" => SessionName::Console,
                _ => SessionName::Others,
            },
            memory_usage: match info[length - 2].replace(",", "").parse::<f64>() {
                Ok(n) => format!("{:.2} MB", n / 1024_f64),
                Err(e) => {
                    println!("{} | {}", info[length - 2], e);
                    String::from("N/A")
                }
            },
        }
    }
}

#[derive(Serialize)]
pub struct TasksList {
    pub all: Vec<Task>,
    pub services: Vec<Task>,
    pub console: Vec<Task>,
    pub other: Vec<Task>,
}

impl TasksList {
    pub fn new(filter: &str) -> Self {
        let output = Command::new("tasklist")
            .creation_flags(0x08000000)
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().filter(|s| !s.is_empty()).collect();

        let mut all = Vec::with_capacity(lines.len());
        let mut services = Vec::new();
        let mut console = Vec::new();
        let mut other = Vec::new();

        for line in lines[2..].iter() {
            let columns: Vec<&str> = line.split_whitespace().collect();
            let task = Task::new(columns);

            if task.process_name == "tasklist.exe"
                || task.process_name == format!("{}.exe", env!("CARGO_PKG_NAME"))
                || task.pid == std::process::id()
            {
                continue;
            }

            if filter.is_empty() {
                all.push(task.clone());

                if task.session_name == SessionName::Console {
                    console.push(task)
                } else if task.session_name == SessionName::Services {
                    services.push(task)
                } else {
                    other.push(task);
                };

                continue;
            }
            
            if task.process_name.to_lowercase().contains(&filter.to_lowercase())
            {
                all.push(task.clone());

                if task.session_name == SessionName::Console {
                    console.push(task)
                } else if task.session_name == SessionName::Services {
                    services.push(task)
                } else {
                    other.push(task);
                };

                continue;
            }

            if task.pid.to_string().to_lowercase().contains(&filter) {
                all.push(task.clone());

                if task.session_name == SessionName::Console {
                    console.push(task)
                } else if task.session_name == SessionName::Services {
                    services.push(task)
                } else {
                    other.push(task);
                };

                continue;
            }
        }

        TasksList {
            all,
            services,
            console,
            other,
        }
    }
}
