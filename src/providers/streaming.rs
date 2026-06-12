use std::{
    io::Read,
    process::{Command, Stdio},
    thread,
};

use crate::{Result, StudyError};

/// Runs a command while optionally streaming child output live.
pub fn run_command(mut command: Command, full_output: bool) -> Result<String> {
    command.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = command.spawn()?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| StudyError::ProviderFailed("provider stdout was unavailable".to_owned()))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| StudyError::ProviderFailed("provider stderr was unavailable".to_owned()))?;

    let stdout_handle = thread::spawn(move || read_stream(stdout, full_output));
    let stderr_handle = thread::spawn(move || read_stream(stderr, full_output));
    let status = child.wait()?;
    let stdout = join_output(stdout_handle)?;
    let stderr = join_output(stderr_handle)?;

    status
        .success()
        .then(|| String::from_utf8_lossy(&stdout).to_string())
        .ok_or_else(|| StudyError::ProviderFailed(String::from_utf8_lossy(&stderr).to_string()))
}

fn read_stream(mut stream: impl Read, full_output: bool) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();
    let mut buffer = [0; 8192];
    loop {
        let count = stream.read(&mut buffer)?;
        if count == 0 {
            break;
        }

        let chunk = &buffer[..count];
        output.extend_from_slice(chunk);
        full_output.then(|| eprint!("{}", String::from_utf8_lossy(chunk)));
    }
    Ok(output)
}

fn join_output(handle: thread::JoinHandle<std::io::Result<Vec<u8>>>) -> Result<Vec<u8>> {
    handle
        .join()
        .map_err(|_| StudyError::ProviderFailed("provider output reader panicked".to_owned()))?
        .map_err(Into::into)
}
