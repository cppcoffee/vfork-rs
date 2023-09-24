use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::Error as IoError;
use std::os::fd::FromRawFd;
use std::os::unix::ffi::OsStrExt;

use crate::child::Child;
use crate::error::Result;

pub struct Command {
    program: OsString,
    argv: Vec<OsString>,
}

impl Command {
    /// Constructs a new Command for launching the program at path program
    pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
        Command {
            program: program.as_ref().to_os_string(),
            argv: Vec::new(),
        }
    }

    /// Adds an argument to pass to the program.
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.argv.push(arg.as_ref().to_os_string());
        self
    }

    /// Adds multiple arguments to pass to the program.
    pub fn args<S: AsRef<OsStr>>(&mut self, args: &[S]) -> &mut Self {
        for arg in args {
            self.argv.push(arg.as_ref().to_os_string());
        }
        self
    }

    /// Executes the command as a child process
    pub fn spawn(&self) -> Result<Child> {
        let mut fds = [0; 2];

        let mut items = Vec::with_capacity(self.argv.len() + 1);
        items.push(self.program.clone());
        items.extend(self.argv.iter().cloned());

        let mut argv = items
            .iter()
            .map(|arg| arg.as_bytes().as_ptr() as *const libc::c_char)
            .collect::<Vec<_>>();

        // null pointer at last
        argv.push(std::ptr::null());

        unsafe {
            if libc::pipe(fds.as_mut_ptr()) == -1 {
                return Err(IoError::last_os_error().into());
            }

            let pid = libc::vfork();
            match pid {
                -1 => Err(IoError::last_os_error().into()),
                0 => {
                    // child
                    libc::close(fds[0]);

                    if libc::dup2(fds[1], libc::STDOUT_FILENO) == -1 {
                        libc::_exit(1);
                    }

                    libc::execv(
                        self.program.as_bytes().as_ptr() as *const libc::c_char,
                        argv.as_ptr() as *const *const libc::c_char,
                    );

                    libc::_exit(1);
                }
                _ => {
                    libc::close(fds[1]);

                    let file = File::from_raw_fd(fds[0]);

                    Ok(Child::new(pid, file))
                }
            }
        }
    }
}
