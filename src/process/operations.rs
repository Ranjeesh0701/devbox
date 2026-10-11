use std::process::{Child, Command};
use std::{io, mem};
use uuid::Uuid;

use super::session::ProcessSession;
use crate::error::DevboxError;

pub fn start_process(
    name: String,
    program: String,
    args: Vec<String>,
) -> Result<(Child, ProcessSession), DevboxError> {
    let child = Command::new(&program)
        .args(args)
        .spawn()
        .map_err(DevboxError::ProcessLaunch)?;

    let pid = child.id();
    let (start_time_seconds, start_time_useconds) = match get_process_start_time(pid) {
        Ok(val) => val,
        Err(e) => return Err(DevboxError::ProcessCheck(e)),
    };
    let session = ProcessSession {
        id: Uuid::new_v4().to_string(),
        pid: pid,
        name: name,
        command: program,
        start_time_seconds: start_time_seconds,
        start_time_useconds: start_time_useconds,
    };

    println!("Started {} with PID {}", session.name, session.pid);
    Ok((child, session))
}

pub fn stop_process(pid: u32) -> Result<bool, std::io::Error> {
    let pid: i32 = pid_to_i32(pid)?;

    let result = unsafe { libc::kill(pid, libc::SIGTERM) };

    if result == 0 {
        return Ok(true);
    }

    let err = std::io::Error::last_os_error();
    match err.raw_os_error() {
        Some(libc::ESRCH) => Ok(false),
        _ => Err(err),
    }
}

fn is_process_active(pid: u32) -> Result<bool, std::io::Error> {
    let pid = pid_to_i32(pid)?;

    let result = unsafe { libc::kill(pid, 0) };

    if result == 0 {
        return Ok(true);
    }

    let err = std::io::Error::last_os_error();
    match err.raw_os_error() {
        Some(libc::ESRCH) => Ok(false),
        Some(libc::EPERM) => Ok(true),
        _ => Err(err),
    }
}

fn get_process_start_time(pid: u32) -> Result<(u64, u64), std::io::Error> {
    let pid = pid_to_i32(pid)?;
    let mut info: libc::proc_bsdinfo = unsafe { mem::zeroed() };
    let size = mem::size_of::<libc::proc_bsdinfo>() as i32;

    let result = unsafe {
        libc::proc_pidinfo(
            pid,
            libc::PROC_PIDTBSDINFO,
            0,
            &mut info as *mut _ as *mut libc::c_void,
            size,
        )
    };

    if result <= 0 {
        return Err(io::Error::last_os_error());
    }
    if result < size {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "short proc_pidinfo result",
        ));
    }

    Ok((info.pbi_start_tvsec, info.pbi_start_tvusec))
}

pub fn is_same_process(session: &ProcessSession) -> Result<bool, std::io::Error> {
    if !is_process_active(session.pid)? {
        return Ok(false);
    }

    let (current_start_time_seconds, current_start_time_useconds) =
        get_process_start_time(session.pid)?;

    Ok(current_start_time_seconds == session.start_time_seconds
        && current_start_time_useconds == session.start_time_useconds)
}

pub fn pid_to_i32(pid: u32) -> Result<i32, std::io::Error> {
    i32::try_from(pid)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "pid out of range"))
}
