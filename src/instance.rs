#[cfg(target_os = "linux")]
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver};
use std::thread;

const ACTIVATION_ADDRESS: &str = "127.0.0.1:37481";

pub struct ManagerInstanceLock {
    #[cfg(target_os = "linux")]
    _file: std::fs::File,
    #[cfg(target_os = "windows")]
    _handle: windows_sys::Win32::Foundation::HANDLE,
}

impl ManagerInstanceLock {
    pub fn acquire() -> Result<Self, String> {
        #[cfg(target_os = "linux")]
        {
            use std::fs::OpenOptions;
            use std::os::fd::AsRawFd;

            let path = crate::config::Config::get_runtime_dir().join("manager.lock");
            let file = OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .open(&path)
                .map_err(|error| format!("Failed to open manager lock: {error}"))?;

            let result = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
            if result != 0 {
                let error = io::Error::last_os_error();
                return Err(if error.raw_os_error() == Some(libc::EWOULDBLOCK)
                    || error.raw_os_error() == Some(libc::EAGAIN)
                {
                    "Another WebFlow Runtime Manager instance is already running".to_string()
                } else {
                    format!("Failed to lock manager instance: {error}")
                });
            }

            return Ok(Self { _file: file });
        }

        #[cfg(target_os = "windows")]
        {
            use std::ptr::null;
            use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS};
            use windows_sys::Win32::System::Threading::CreateMutexW;

            let name: Vec<u16> = "Local\\WebFlowRuntimeManager\0".encode_utf16().collect();
            let handle = unsafe { CreateMutexW(null(), 0, name.as_ptr()) };
            if handle.is_null() {
                return Err("Failed to create manager instance mutex".to_string());
            }
            if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
                unsafe { CloseHandle(handle) };
                return Err("Another WebFlow Runtime Manager instance is already running".to_string());
            }

            return Ok(Self { _handle: handle });
        }

        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        {
            Ok(Self {})
        }
    }
}

pub fn request_manager_activation() -> bool {
    let Ok(mut stream) = TcpStream::connect(ACTIVATION_ADDRESS) else {
        return false;
    };
    let _ = stream.write_all(b"show");
    true
}

pub fn start_activation_listener() -> Option<Receiver<()>> {
    let listener = TcpListener::bind(ACTIVATION_ADDRESS).ok()?;
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        for mut stream in listener.incoming().flatten() {
            let mut message = [0u8; 4];
            if stream.read_exact(&mut message).is_ok() && &message == b"show" {
                let _ = sender.send(());
            }
        }
    });

    Some(receiver)
}

#[cfg(target_os = "windows")]
impl Drop for ManagerInstanceLock {
    fn drop(&mut self) {
        unsafe { windows_sys::Win32::Foundation::CloseHandle(self._handle) };
    }
}
