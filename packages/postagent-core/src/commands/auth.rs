use crate::token;
use std::io::{self, Write};

pub fn run(project: &str) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = read_secret(&format!("Enter API key for \"{}\": ", project))?;

    if api_key.is_empty() {
        eprintln!("Error: API key cannot be empty.");
        std::process::exit(1);
    }

    match token::save_token(project, &api_key) {
        Ok(()) => {
            println!("Auth saved for \"{}\".", project.to_lowercase());
            Ok(())
        }
        Err(e) => {
            let err_str = e.to_string();
            if err_str.contains("Permission denied") || err_str.contains("permission denied") {
                eprintln!("Error: Permission denied. Check directory permissions.");
            } else {
                eprintln!("Error: {}", err_str);
            }
            std::process::exit(1);
        }
    }
}

fn read_secret(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    eprint!("{}", prompt);
    io::stderr().flush()?;

    if atty_check() {
        read_secret_tty()
    } else {
        read_secret_pipe()
    }
}

fn atty_check() -> bool {
    unsafe { libc_isatty(0) }
}

#[cfg(unix)]
unsafe fn libc_isatty(fd: i32) -> bool {
    extern "C" {
        fn isatty(fd: i32) -> i32;
    }
    unsafe { isatty(fd) != 0 }
}

#[cfg(windows)]
unsafe fn libc_isatty(fd: i32) -> bool {
    extern "C" {
        fn _isatty(fd: i32) -> i32;
    }
    unsafe { _isatty(fd) != 0 }
}

fn read_secret_pipe() -> Result<String, Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}

#[cfg(unix)]
fn read_secret_tty() -> Result<String, Box<dyn std::error::Error>> {
    use std::os::unix::io::AsRawFd;

    let stdin = io::stdin();
    let fd = stdin.as_raw_fd();

    // Save original terminal settings
    let original = unsafe {
        let mut t: libc::termios = std::mem::zeroed();
        libc::tcgetattr(fd, &mut t);
        t
    };

    // Disable echo
    unsafe {
        let mut t = original;
        t.c_lflag &= !(libc::ECHO);
        libc::tcsetattr(fd, libc::TCSANOW, &t);
    }

    let mut input = String::new();
    let result = io::stdin().read_line(&mut input);

    // Restore original terminal settings
    unsafe {
        libc::tcsetattr(fd, libc::TCSANOW, &original);
    }
    eprintln!(); // newline after hidden input

    result?;
    Ok(input.trim().to_string())
}

#[cfg(windows)]
fn read_secret_tty() -> Result<String, Box<dyn std::error::Error>> {
    read_secret_pipe()
}
