use std::default::Default;
use std::io::{self, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn create_listening_socket(port: u16) -> Result<libc::c_int> {
    unsafe {
        let sock = libc::socket(libc::PF_INET, libc::SOCK_STREAM, 0);
        if sock < 0 {
            return Err(io::Error::last_os_error().into());
        }

        let len = std::mem::size_of::<libc::sockaddr_in>();

        // This sockaddr is only constructed on macOS.
        #[cfg(target_os = "macos")]
        let sockaddr = libc::sockaddr_in {
            sin_len: len as u8, // macOS, but not Linux
            sin_family: libc::AF_INET as u8,
            sin_port: port.to_be(),
            sin_addr: libc::in_addr {
                s_addr: libc::INADDR_ANY,
            },
            sin_zero: Default::default(),
        };

        // This sockaddr is only constructed on Linux.
        #[cfg(target_os = "linux")]
        let sockaddr = libc::sockaddr_in {
            sin_family: libc::AF_INET as u16,
            sin_port: port.to_be(),
            sin_addr: libc::in_addr {
                s_addr: libc::INADDR_ANY,
            },
            sin_zero: Default::default(),
        };

        let sockaddr_ptr = (&sockaddr as *const libc::sockaddr_in).cast::<libc::sockaddr>();
        if libc::bind(sock, sockaddr_ptr, len as u32) < 0 {
            let err = io::Error::last_os_error();
            libc::close(sock);
            return Err(err.into());
        }

        if libc::listen(sock, 10) < 0 {
            let err = io::Error::last_os_error();
            libc::close(sock);
            return Err(err.into());
        }
        Ok(sock)
    }
}

fn handle_connection(sock: libc::c_int) -> Result<()> {
    let mut data = vec![0; 4096];

    loop {
        unsafe {
            let ptr: *mut libc::c_void = data.as_mut_ptr().cast();
            let len: isize = libc::recv(sock, ptr, data.len(), 0);
            if len < 0 {
                return Err(io::Error::last_os_error().into());
            }
            if len == 0 {
                libc::close(sock);
                return Ok(());
            }
            let mut stdout = io::stdout();
            stdout.write_all(&data[..len as usize])?;
            stdout.flush()?;
        }
    }
}

fn main() -> Result<()> {
    let listen_sock = create_listening_socket(8765)?;

    loop {
        unsafe {
            let mut remote_addr: libc::sockaddr = std::mem::zeroed();
            let mut addr_len = 0;
            let sock = libc::accept(listen_sock, &mut remote_addr, &mut addr_len);
            if sock < 0 {
                return Err(io::Error::last_os_error().into());
            }

            // Ignore errors.
            let _ = handle_connection(sock);
        }
    }
}
