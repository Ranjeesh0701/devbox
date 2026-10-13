use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::process::Child;

use super::operations::{is_same_process, start_process, stop_process};
use super::session::{ProcessSession, ProcessStatus};
use crate::error::DevboxError;
use crate::storage::{get_log_file, get_sessions, save_sessions};

struct ManagedProcess {
    child: Option<Child>,
    session: ProcessSession,
}

pub struct ProcessSupervisor {
    processes: Vec<ManagedProcess>,
}

impl ProcessSupervisor {
    pub fn new() -> Result<Self, DevboxError> {
        let sessions = get_sessions()?;

        let processes = sessions
            .into_iter()
            .map(|session| ManagedProcess {
                child: None,
                session,
            })
            .collect();

        Ok(Self { processes })
    }

    fn find_process(&self, id: &str) -> Option<&ManagedProcess> {
        self.processes.iter().find(|p| p.session.id == id)
    }

    fn remove_process(&mut self, id: &str) {
        self.processes.retain(|p| p.session.id != id);
    }

    pub fn list(&mut self) -> Result<Vec<ProcessSession>, DevboxError> {
        for process in &mut self.processes {
            let child_result = match process.child.as_mut() {
                Some(child) => Some(child.try_wait()),
                None => None,
            };

            match child_result {
                Some(Ok(Some(exit_status))) => {
                    process.session.status = ProcessStatus::Exited;
                    process.session.exit_code = exit_status.code();
                    process.child = None;
                }
                Some(Ok(None)) => {
                    process.session.status = ProcessStatus::Running;
                }
                Some(Err(error)) => {
                    eprintln!(
                        "Failed to check child process {}: {error}",
                        process.session.pid
                    );
                }
                None => match is_same_process(&process.session) {
                    Ok(true) => {
                        process.session.status = ProcessStatus::Running;
                    }
                    Ok(false) => {
                        process.session.status = ProcessStatus::Exited;
                    }
                    Err(error) => {
                        eprintln!("Failed to check process {}: {error}", process.session.pid);
                    }
                },
            }
        }

        self.save_sessions()?;

        Ok(self
            .processes
            .iter()
            .map(|process| process.session.clone())
            .collect())
    }

    pub fn start(
        &mut self,
        name: String,
        program: String,
        args: Vec<String>,
        cwd: PathBuf,
        env: HashMap<String, String>,
    ) -> Result<&ProcessSession, DevboxError> {
        let (child, session) = start_process(name, program, args, cwd, env)?;
        let session_id = session.id.clone();
        self.processes.push(ManagedProcess {
            child: Some(child),
            session,
        });

        if let Err(save_error) = self.save_sessions() {
            if let Err(stop_error) = self.stop(&session_id) {
                eprintln!("Failed to stop process after save failure: {stop_error:?}");
            }

            return Err(save_error);
        }

        Ok(&self.processes.last().unwrap().session)
    }

    pub fn stop(&mut self, id: &str) -> Result<(), DevboxError> {
        let Some(process) = self.find_process(id) else {
            return Err(DevboxError::ProcessNotFound(io::Error::new(
                io::ErrorKind::NotFound,
                "Process not found",
            )));
        };

        if !is_same_process(&process.session).map_err(DevboxError::ProcessCheck)? {
            return Err(DevboxError::ProcessNotFound(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Process with PID {} has changed since the session was created",
                    process.session.pid
                ),
            )));
        }

        let pid = process.session.pid;
        let name = process.session.name.clone();

        match stop_process(pid) {
            Ok(true) => {
                println!("Sent termination signal to {} (PID {})", name, pid);
            }
            Ok(false) => {
                println!("Process {} (PID {}) is already gone", name, pid);
            }
            Err(e) => return Err(DevboxError::ProcessCheck(e)),
        }
        self.remove_process(id);
        self.save_sessions()?;

        Ok(())
    }

    pub fn log_file(&self, id: &str) -> Result<std::path::PathBuf, DevboxError> {
        self.find_process(id)
            .map(|process| get_log_file(&process.session.id))
            .ok_or_else(|| {
                DevboxError::ProcessNotFound(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Session not found",
                ))
            })?
    }

    fn save_sessions(&self) -> Result<(), DevboxError> {
        let sessions: Vec<ProcessSession> =
            self.processes.iter().map(|p| p.session.clone()).collect();

        save_sessions(&sessions)?;

        Ok(())
    }
}
