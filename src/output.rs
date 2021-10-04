use std::io;
use std::io::Error;
use std::fs::OpenOptions;
use std::io::{Write, stdout};
use crossterm::{QueueableCommand, cursor};
use chrono::{DateTime, Local};
use std::path::Path;
use crossterm::style::Stylize;
use std::{sync, thread, time};
use std::sync::atomic::{AtomicBool, Ordering};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;

lazy_static! {
    pub static ref OUTPUT: Mutex<HashMap<String, String>> = {
        let mut m = HashMap::new();
        m.insert("logfile".to_string(), "/tmp/".to_string()+&get_cmd()+".log");
        m.insert("log".to_string(), "true".to_string());
        m.insert("log_enabled".to_string(), "false".to_string());
        Mutex::new(m)
    };
}

impl OUTPUT {
    pub fn replace(key: &str, value: &str) {
        let mut data = OUTPUT.lock().expect("Could not lock mutex");
        data.insert(key.to_string(), value.to_string());
    }

    pub fn get(key: &str) -> String {
        let data = OUTPUT.lock().expect("Could not lock mutex");
        data.get(key).unwrap().to_string()
    }
    pub fn display(key: &str) {
        let data = OUTPUT.lock().expect("Could not lock mutex");
        let mut text: String = "[OUTPUT] Key: ".to_owned();
        text.push_str(&yellow(&format!("{}", key)));
        text.push_str(&" - Value: ");
        text.push_str(&yellow(&format!("{}", data.get(key).unwrap())));
        let s: String = text.to_string();
        drop(data);
        ok(&s);
    }
}


pub fn display_error(message: &str, logfile: &str, err: Error) {
    let command = std::env::args().nth(0).unwrap();
    let cmd = Path::new(&command).file_name().unwrap().to_str().unwrap();
    let cmd_string = format!("{}", cmd.red());
    let now: DateTime<Local> = Local::now();
    let mut status_str: String = "[".to_string();
    status_str.push_str(format!("{}", "ERROR".red()).as_str());
    status_str.push_str("]");
    let file_str = format!("{}", logfile.cyan());
    println!(
        "{cmd} - [{date}] {message} {file}. {err_str} {status}",
        cmd = cmd_string,
        date = now.format("%y-%m-%d %T"),
        message = message,
        file = file_str,
        err_str = err,
        status = status_str
    );
    std::process::exit(0);
}

fn log(message: &str, status: &str) {
    let log_enabled = OUTPUT::get("log_enabled");
    let log: bool = log_enabled.parse().unwrap();
    if !log {
        return
    };
    let log_active = OUTPUT::get("log");
    let log: bool = log_active.parse().unwrap();
    if !log {
        return
    };

    // let logfile = OUTPUT::get("logfile");
    // OUTPUT::display("logfile");
    let command = std::env::args().nth(0).unwrap();
    let cmd = Path::new(&command).file_name().unwrap().to_str().unwrap();
    let cmd_string = format!("{}", cmd.red());
    let now: DateTime<Local> = Local::now();
    let mut status_str: String = "[".to_string();
    match status {
        "OK" => status_str.push_str(format!("{}", "OK".green()).as_str()),
        "ERROR" => status_str.push_str(format!("{}", "ERROR".red()).as_str()),
        "DEBUG" => status_str.push_str(format!("{}", "DEBUG".cyan()).as_str()),
        "INFO" => status_str.push_str(format!("{}", "INFO".yellow()).as_str()),
        "WARN" => status_str.push_str(format!("{}", "WARN".magenta()).as_str()),
        _ => status_str.push_str(format!("{}", "INFO".red()).as_str()),
    };

    status_str.push_str("]");
    let tmp = format!(
        "{cmd} - [{date}] {message} {status}",
        cmd = cmd_string,
        date = now.format("%y-%m-%d %T"),
        status = status_str,
        message = message
    );
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(OUTPUT::get("logfile"))
        .unwrap();

     if let Err(e) = writeln!(file,"{}", tmp) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn print(message: &str, status: &str) {
    let command = std::env::args().nth(0).unwrap();
    let cmd = Path::new(&command).file_name().unwrap().to_str().unwrap();
    let cmd_string = format!("{}", cmd.red());
    let now: DateTime<Local> = Local::now();
    let mut status_str: String = "[".to_string();
    match status {
        "OK" => status_str.push_str(format!("{}", "OK".green()).as_str()),
        "ERROR" => status_str.push_str(format!("{}", "ERROR".red()).as_str()),
        "DEBUG" => status_str.push_str(format!("{}", "DEBUG".cyan()).as_str()),
        "INFO" => status_str.push_str(format!("{}", "INFO".yellow()).as_str()),
        "WARN" => status_str.push_str(format!("{}", "WARN".magenta()).as_str()),
        _ => status_str.push_str(format!("{}", "INFO".red()).as_str()),
    };

    status_str.push_str("]");
    println!(
        "{cmd} - [{date}] {message} {status}",
        cmd = cmd_string,
        date = now.format("%y-%m-%d %T"),
        status = status_str,
        message = message
    );
}


pub fn info(message: &str) {
    print(&message, "INFO");
    log(&message, "INFO");
}

pub fn ok(message: &str) {
    log(&message, "OK");
    print(&message, "OK");
}

pub fn error(message: &str) {
    print(&message, "ERROR");
    log(&message, "ERROR");
}

pub fn debug(message: &str) {
    print(&message, "DEBUG");
    log(&message, "DEBUG");
}

pub fn warn(message: &str) {
    print(&message, "WARN");
    log(&message, "WARN");
}

pub fn green(text: &str) -> String {
    return format!("{}", text.green())
}

pub fn red(text: &str) -> String {
    return format!("{}", text.red())
}

pub fn yellow(text: &str) -> String {
    return format!("{}", text.yellow())
}

pub fn cyan(text: &str) -> String {
    return format!("{}", text.cyan())
}

pub fn magenta(text: &str) -> String {
    return format!("{}", text.magenta())
}

pub fn grey(text: &str) -> String {
    return format!("{}", text.grey())
}

pub fn b_cyan(text: &str) -> String {
    return format!("{}", text.black().on_cyan())
}



pub struct Rotor {
    handle: Option<thread::JoinHandle<()>>,
    alive: sync::Arc<AtomicBool>,
}

impl Rotor {
    pub fn new() -> Rotor {
        Rotor {
            handle: None,
            alive: sync::Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&mut self)
    {
        let r_chars: [char; 4] = ['|', '/', '-', '\\'];
        let mut stdout = stdout();
        let _ = stdout.queue(cursor::Hide);

        self.alive.store(true, Ordering::SeqCst);

        let alive = self.alive.clone();
        let wait = 100;

        self.handle = Some(thread::spawn(move || {
            while alive.load(Ordering::SeqCst) {
                for x in r_chars {
                    print!("{}", x.black().on_green());
                    let _ = io::stdout().flush();
                    thread::sleep(time::Duration::from_millis(wait));
                    let _ = stdout.queue(cursor::MoveLeft(1));
                    let _ = io::stdout().flush();
                }
            }
        }));
    }

    pub fn ok(&mut self, message: &str) {
        self.stop();
        print!("\r");
        let _ = io::stdout().flush();
        print(&message, "OK");
    }

    pub fn error(&mut self, message: &str) {
        self.stop();
        print!("\r");
        let _ = io::stdout().flush();
        print(&message, "ERROR");
    }

    pub fn stop(&mut self) {
        self.alive.store(false, Ordering::SeqCst);
        self.handle
            .take().expect("Called stop on non-running thread")
            .join().expect("Could not join spawned thread");
        let mut stdout = stdout();
        let _ = stdout.queue(cursor::Show);
    }
}

pub fn start(message: &str) -> Rotor {
    let command = std::env::args().nth(0).unwrap();
    let cmd = Path::new(&command).file_name().unwrap().to_str().unwrap();
    let cmd_string = format!("{}", cmd.red());
    let now: DateTime<Local> = Local::now();
    print!(
        "{cmd} - [{date}] {message}",
        cmd = cmd_string,
        date = now.format("%y-%m-%d %T"),
        message = message
    );
    let _ = io::stdout().flush();
    let mut rotor = Rotor::new();
    rotor.start();
    return rotor;
}


pub fn get_cmd() -> String {
    let command = std::env::args().nth(0).unwrap();
    let cmd = Path::new(&command).file_name().unwrap().to_str().unwrap();
    cmd.to_string()
}
