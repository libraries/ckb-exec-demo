//! Generated by capsule
//!
//! `main.rs` is used to define rust lang items and modules.
//! See `entry.rs` for the `main` function.
//! See `error.rs` for the `Error` type.

#![no_std]
#![no_main]
#![feature(asm_sym)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

mod error;

use ckb_std::cstr_core::CStr;
use ckb_std::default_alloc;
use core::arch::asm;

ckb_std::entry!(program_entry);
default_alloc!();

/// program entry
///
///  Both `argc` and `argv` can be omitted.
fn program_entry(argc: u64, argv: *const *const u8) -> i8 {
    // This script will always return 0 if used alone.
    if argc == 0 {
        return 0;
    };

    // When calling the script by exec and passing in the arguments.
    let args = unsafe { core::slice::from_raw_parts(argv, argc as usize) };
    let arg1 = unsafe { CStr::from_ptr(args[0]) }.to_str().unwrap();
    let exit = arg1.parse::<i8>().unwrap();
    return exit;
}