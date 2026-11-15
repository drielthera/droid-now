#[cfg(windows)]
use process_wrap::std::JobObject;
#[cfg(unix)]
use process_wrap::std::ProcessGroup;
use process_wrap::std::{ChildWrapper, CommandWrap};
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProcessOutput {
    pub stdout: String,
    pub stderr: String,
    pub code: Option<i32>,
}

/// Injectable boundary around an external process. Implementations must honor the timeout.
pub(crate) trait ProcessRunner: Send + Sync {
    fn run(
        &self,
        program: &Path,
        args: &[String],
        timeout: Duration,
    ) -> Result<ProcessOutput, String>;
}

#[derive(Debug, Default)]
pub(crate) struct SystemProcessRunner;

impl ProcessRunner for SystemProcessRunner {
    fn run(
        &self,
        program: &Path,
        args: &[String],
        timeout: Duration,
    ) -> Result<ProcessOutput, String> {
        // Execute the binary directly; never interpolate arguments through a shell.
        let mut command = CommandWrap::from(Command::new(program));
        command
            .command_mut()
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        #[cfg(unix)]
        command.wrap(ProcessGroup::leader());
        #[cfg(windows)]
        command.wrap(JobObject);

        let mut child = command
            .spawn()
            .map_err(|error| format!("Gagal menjalankan {}: {error}", program.display()))?;

        let mut stdout_pipe = child.stdout().take().expect("piped stdout");
        let mut stderr_pipe = child.stderr().take().expect("piped stderr");
        let (stdout_sender, stdout_receiver) = mpsc::channel();
        let (stderr_sender, stderr_receiver) = mpsc::channel();
        let stdout_reader = thread::spawn(move || {
            let mut bytes = Vec::new();
            let _ = stdout_pipe.read_to_end(&mut bytes);
            let _ = stdout_sender.send(String::from_utf8_lossy(&bytes).into_owned());
        });
        let stderr_reader = thread::spawn(move || {
            let mut bytes = Vec::new();
            let _ = stderr_pipe.read_to_end(&mut bytes);
            let _ = stderr_sender.send(String::from_utf8_lossy(&bytes).into_owned());
        });

        let deadline = Instant::now() + timeout;
        let mut status = None;
        let mut stdout = None;
        let mut stderr = None;
        loop {
            if status.is_none() {
                match child.try_wait() {
                    Ok(exit_status) => status = exit_status,
                    Err(error) => {
                        kill_and_reap_in_background(child);
                        drop((stdout_reader, stderr_reader));
                        return Err(format!(
                            "Tidak dapat menunggu {}: {error}",
                            program.display()
                        ));
                    }
                }
            }
            if stdout.is_none() {
                match stdout_receiver.try_recv() {
                    Ok(output) => stdout = Some(output),
                    Err(TryRecvError::Disconnected) => stdout = Some(String::new()),
                    Err(TryRecvError::Empty) => {}
                }
            }
            if stderr.is_none() {
                match stderr_receiver.try_recv() {
                    Ok(output) => stderr = Some(output),
                    Err(TryRecvError::Disconnected) => stderr = Some(String::new()),
                    Err(TryRecvError::Empty) => {}
                }
            }
            if status.is_some() && stdout.is_some() && stderr.is_some() {
                let status = status.take().expect("status was checked");
                let stdout = stdout.take().expect("stdout was checked");
                let stderr = stderr.take().expect("stderr was checked");
                let _ = stdout_reader.join();
                let _ = stderr_reader.join();
                return Ok(ProcessOutput {
                    stdout,
                    stderr,
                    code: status.code(),
                });
            }
            let now = Instant::now();
            if now >= deadline {
                // A descendant may keep the inherited pipes open after the direct
                // child exits. Kill the entire group and never join a pipe reader
                // unless its result arrived before this deadline.
                kill_and_reap_in_background(child);
                drop((stdout_reader, stderr_reader));
                return Err(format!(
                    "{} tidak merespons (timeout {} detik).",
                    program.display(),
                    timeout.as_secs()
                ));
            }
            thread::sleep(Duration::from_millis(5).min(deadline - now));
        }
    }
}

fn kill_and_reap_in_background(mut child: Box<dyn ChildWrapper>) {
    let _ = child.start_kill();
    let _ = thread::Builder::new()
        .name("droidnow-process-reaper".into())
        .spawn(move || {
            let _ = child.wait();
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn system_runner_kills_process_after_timeout() {
        let result = SystemProcessRunner.run(
            Path::new("/bin/sleep"),
            &["2".into()],
            Duration::from_millis(30),
        );
        let error = result.expect_err("sleep must be interrupted by the timeout");
        assert!(error.contains("timeout"), "unexpected error: {error}");
    }

    #[cfg(unix)]
    #[test]
    fn timeout_kills_descendants_that_keep_pipes_open_after_parent_exits() {
        use std::time::Instant;

        let marker = std::env::temp_dir().join(format!(
            "droidnow-descendant-survived-{}.txt",
            std::process::id()
        ));
        let script = format!("(sleep 1; printf leaked > '{}') & exit 0", marker.display());
        let started = Instant::now();
        let result = SystemProcessRunner.run(
            Path::new("/bin/sh"),
            &["-c".into(), script],
            Duration::from_millis(150),
        );
        let elapsed = started.elapsed();
        assert!(result.unwrap_err().contains("timeout"));
        assert!(
            elapsed < Duration::from_millis(800),
            "runner took {elapsed:?}"
        );

        thread::sleep(Duration::from_millis(1100));
        assert!(
            !marker.exists(),
            "a descendant escaped group termination and wrote its marker"
        );
    }
}
