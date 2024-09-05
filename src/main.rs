#![allow(unused)]
use libc::c_char;
use std::arch::asm;
use std::ffi::CStr;

unsafe fn syscall_write(message: &str) {
    asm!(
        "mov rax, 1",
        "mov rdi, 1",
        "syscall",
        in("rsi") message.as_ptr(),
        in("rdx") message.len(),
    );
}

unsafe fn syscall_uname(utsname: &Utsname) {
    asm!(
        "mov rax, 63",
        "syscall",
        in("rdi") utsname as *const _,
    );
}

#[repr(C)]
pub struct Utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
    pub domainname: [c_char; 65],
}

impl Utsname {
    fn new() -> Self {
        Self {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
            domainname: [0; 65],
        }
    }
}

#[derive(Debug)]
pub struct UnameInfo {
    pub sysname: String,
    pub nodename: String,
    pub release: String,
    pub version: String,
    pub machine: String,
    pub domainname: String,
}

// Inspired by uname crate.
impl From<Utsname> for UnameInfo {
    fn from(utsname: Utsname) -> Self {
        Self {
            sysname: { unsafe { CStr::from_ptr(utsname.sysname.as_ptr()) } }
                .to_string_lossy()
                .to_string(),
            nodename: { unsafe { CStr::from_ptr(utsname.nodename.as_ptr()) } }
                .to_string_lossy()
                .to_string(),
            release: { unsafe { CStr::from_ptr(utsname.release.as_ptr()) } }
                .to_string_lossy()
                .to_string(),
            version: { unsafe { CStr::from_ptr(utsname.version.as_ptr()) } }
                .to_string_lossy()
                .to_string(),
            machine: { unsafe { CStr::from_ptr(utsname.machine.as_ptr()) } }
                .to_string_lossy()
                .to_string(),
            domainname: { unsafe { CStr::from_ptr(utsname.domainname.as_ptr()) } }
                .to_string_lossy()
                .to_string(),
        }
    }
}

#[derive(Debug)]
enum Error {}

fn main() -> Result<(), Error> {
    let message1 = String::from("Hello from: ");
    let message2 = String::from("🦉 Uggla !!!\n");
    let mut utsname = Utsname::new();

    unsafe {
        syscall_write(&message1);
        syscall_write(&message2);
        syscall_uname(&utsname);
    }

    let uname = UnameInfo::from(utsname);
    println!("{:#?}", uname);
    Ok(())
}
