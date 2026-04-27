use std::{
    env,
    error::Error,
    fs,
    process::{Command, Stdio},
};

use nix::{
    sys::signal::{Signal, kill},
    unistd::Pid,
};

use crate::profiles::Config;

pub fn spawn(profile: &str) -> Result<(), Box<dyn Error>> {
    let pid_path = Config::path()?.join(".pids");
    fs::create_dir_all(&pid_path)?;

    let pid_file = pid_path.join(format!("{}.pid", profile));

    if pid_file.exists() {
        eprintln!("autoclicker: {}, is already running", profile);
        return Ok(());
    }

    let command = Command::new(env::current_exe()?)
        .arg("start")
        .arg(&profile)
        .arg("--run")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    fs::write(&pid_file, command.id().to_string())?;
    println!("started autoclicker for: {}", profile);

    Ok(())
}

pub fn stop(profile: &str) -> Result<(), Box<dyn Error>> {
    let pid_file = Config::path()?
        .join(".pids")
        .join(format!("{}.pid", profile));

    if !pid_file.exists() {
        println!("no autoclicker running for: {}", profile);
        return Ok(());
    }

    let pid = fs::read_to_string(&pid_file)?.trim().parse::<u32>()?;

    let result = kill(Pid::from_raw(pid as i32), Signal::SIGTERM);
    match result {
        Ok(_) => {}
        Err(nix::Error::ESRCH) => {
            println!("no running process found for: {}", profile);
        }
        Err(e) => return Err(Box::new(e)),
    }

    fs::remove_file(pid_file)?;

    println!("stopped process: {}", profile);

    Ok(())
}
