use nix::pty::{forkpty, Winsize};
use nix::unistd::{execvp, read, write, ForkResult};
use std::ffi::CString;
use std::io::{self, Read, Write};
use std::os::fd::AsRawFd;
use std::thread;

fn main() {
    let winsize = Winsize { ws_row: 30, ws_col: 90, ws_xpixel: 0, ws_ypixel: 0 };
    let result = unsafe { forkpty(Some(&winsize), None) }.expect("forkpty failed");

    match result.fork_result {
        ForkResult::Child => {
            let shell = CString::new("sh").unwrap();
            let _ = execvp(&shell, &[shell.clone()]);
            std::process::exit(1);
        }
        ForkResult::Parent { child } => {
            let master_fd = result.master.as_raw_fd();
            let reader = thread::spawn(move || {
                let mut buf = [0u8; 4096];
                loop {
                    match read(master_fd, &mut buf) {
                        Ok(0) | Err(_) => break,
                        Ok(n) => {
                            if io::stdout().write_all(&buf[..n]).is_err() { break; }
                            let _ = io::stdout().flush();
                        }
                    }
                }
            });

            let mut buf = [0u8; 4096];
            loop {
                match io::stdin().read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => { if write(master_fd, &buf[..n]).is_err() { break; } }
                }
            }
            let _ = reader.join();
            let _ = nix::sys::wait::waitpid(child, None);
        }
    }
}
