use nix::fcntl::{open, OFlag};
use nix::pty::{grantpt, posix_openpt, ptsname_r, unlockpt};
use nix::sys::stat::Mode;
use nix::unistd::{close, dup2, execvp, fork, read, setsid, write, ForkResult};
use std::ffi::CString;
use std::io::{self, Read, Write};
use std::os::fd::AsRawFd;
use std::thread;

fn main() {
    let master = posix_openpt(OFlag::O_RDWR | OFlag::O_NOCTTY).expect("posix_openpt failed");
    grantpt(&master).expect("grantpt failed");
    unlockpt(&master).expect("unlockpt failed");
    let slave_path = ptsname_r(&master).expect("ptsname_r failed");
    let master_fd = master.as_raw_fd();

    match unsafe { fork() }.expect("fork failed") {
        ForkResult::Child => {
            let _ = setsid();
            let slave_fd = open(slave_path.as_str(), OFlag::O_RDWR, Mode::empty())
                .expect("open slave failed");
            unsafe { libc::ioctl(slave_fd, libc::TIOCSCTTY as _, 0) };
            let _ = dup2(slave_fd, 0);
            let _ = dup2(slave_fd, 1);
            let _ = dup2(slave_fd, 2);
            if slave_fd > 2 {
                let _ = close(slave_fd);
            }
            let _ = close(master_fd);

            let shell = CString::new("sh").unwrap();
            let _ = execvp(&shell, &[shell.clone()]);
            std::process::exit(1);
        }
        ForkResult::Parent { child } => {
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
