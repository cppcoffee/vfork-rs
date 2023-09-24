use std::fs::File;
use std::io::Error as IoError;
use std::io::Read;

use libc::pid_t;

use crate::error::Result;

#[derive(Debug)]
pub struct ExitCode(i32);

pub struct Child {
    pid: pid_t,
    output: File,
}

impl Child {
    pub(crate) fn new(pid: pid_t, output: File) -> Self {
        Child { pid, output }
    }

    /// Try to wait the child process exit. If the child process is still running, return `None`.
    pub fn try_wait(&self) -> Result<Option<ExitCode>> {
        let mut status = -1;

        unsafe {
            if libc::waitpid(self.pid, &mut status, libc::WNOHANG) == -1 {
                return Err(IoError::last_os_error().into());
            }
        }

        if libc::WIFEXITED(status) {
            Ok(Some(ExitCode(libc::WEXITSTATUS(status))))
        } else if libc::WIFSIGNALED(status) {
            Ok(Some(ExitCode(libc::WTERMSIG(status))))
        } else if libc::WIFSTOPPED(status) {
            Ok(Some(ExitCode(libc::WSTOPSIG(status))))
        } else {
            Ok(None)
        }
    }

    /// Wait until the child process exit
    pub fn wait(&self) -> Result<ExitCode> {
        let mut status = -1;

        unsafe {
            if libc::waitpid(self.pid, &mut status, 0) == -1 {
                return Err(IoError::last_os_error().into());
            }
        }

        if libc::WIFEXITED(status) {
            Ok(ExitCode(libc::WEXITSTATUS(status)))
        } else if libc::WIFSIGNALED(status) {
            Ok(ExitCode(libc::WTERMSIG(status)))
        } else {
            Err(IoError::last_os_error().into())
        }
    }

    /// Read the STDOUT of the child process
    pub fn output(&mut self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.output.read_to_end(&mut buf)?;
        Ok(buf)
    }
}
