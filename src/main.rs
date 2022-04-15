#![allow(unused)]
use libc::c_char;
use std::arch::asm;
use std::ffi::CStr;

unsafe fn syscall_write(message: String) {
    asm!(
        "mov rax, 1",
        "mov rdi, 1",
        "syscall",
        in("rsi") message.as_ptr(),
        in("rdx") message.len(),
    );
}

// pub type c_char = i8;

#[repr(C)]
pub struct utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
    pub domainname: [c_char; 65],
}

unsafe fn syscall_uname(utsname: &utsname) {
    asm!(
        "mov rax, 63",
        "syscall",
        in("rdi") utsname as *const _,
    );
}

fn main() {
    let message1 = String::from("Hello from: ");
    let message2 = String::from("🦉 Uggla !!!\n");
    let mut utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    unsafe {
        syscall_write(message1);
        syscall_write(message2);
        syscall_uname(&utsname);
    }
    let sysname = unsafe { CStr::from_ptr(utsname.sysname.as_ptr()) };
    let nodename = unsafe { CStr::from_ptr(utsname.nodename.as_ptr()) };
    let domainename = unsafe { CStr::from_ptr(utsname.domainname.as_ptr()) };
    println!("{}", sysname.to_string_lossy());
    println!("{}", nodename.to_string_lossy());
    println!("{}", domainename.to_string_lossy());
}
