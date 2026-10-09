use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jint, jstring};
use nix::fcntl::{fcntl, FcntlArg, OFlag};
use nix::pty::{forkpty, Winsize};
use nix::unistd::{execvp, read, write, ForkResult};
use std::ffi::CString;
use std::os::fd::AsRawFd;

#[no_mangle]
pub extern "system" fn Java_com_phantom_aos_core_bridge_PtyBridge_nativePing(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let output = env.new_string("PhantomAOS Rust core is alive").unwrap();
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_phantom_aos_core_bridge_PtyBridge_nativeStartShell(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    let winsize = Winsize { ws_row: 30, ws_col: 90, ws_xpixel: 0, ws_ypixel: 0 };
    let result = unsafe { forkpty(Some(&winsize), None) };
    match result {
        Ok(r) => match r.fork_result {
            ForkResult::Child => {
                let shell = CString::new("sh").unwrap();
                let _ = execvp(&shell, &[shell.clone()]);
                std::process::exit(1);
            }
            ForkResult::Parent { .. } => {
                let master_fd = r.master.as_raw_fd();
                if let Ok(flags) = fcntl(master_fd, FcntlArg::F_GETFL) {
                    let mut new_flags = OFlag::from_bits_truncate(flags);
                    new_flags.insert(OFlag::O_NONBLOCK);
                    let _ = fcntl(master_fd, FcntlArg::F_SETFL(new_flags));
                }
                std::mem::forget(r.master);
                master_fd as jint
            }
        },
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_phantom_aos_core_bridge_PtyBridge_nativeWrite(
    mut env: JNIEnv,
    _class: JClass,
    fd: jint,
    input: JString,
) {
    if let Ok(text) = env.get_string(&input) {
        let text: String = text.into();
        let _ = write(fd, text.as_bytes());
    }
}

#[no_mangle]
pub extern "system" fn Java_com_phantom_aos_core_bridge_PtyBridge_nativeRead(
    mut env: JNIEnv,
    _class: JClass,
    fd: jint,
) -> jstring {
    let mut buf = [0u8; 4096];
    let output = match read(fd, &mut buf) {
        Ok(n) if n > 0 => String::from_utf8_lossy(&buf[..n]).to_string(),
        _ => String::new(),
    };
    env.new_string(output).unwrap().into_raw()
}
