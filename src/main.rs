use std::path::PathBuf;

use clap::Parser;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::GetLastError,
        Storage::FileSystem::{CreateSymbolicLinkA, SYMBOLIC_LINK_FLAGS},
    },
};

/// mklink but more stupid and in rust!
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    name: PathBuf,
    target: PathBuf,
}

fn main() {
    let args = Args::parse();

    let name = args.name.to_str().unwrap().as_ptr();
    let name = PCSTR::from_raw(name);

    let path = args.target.to_str().unwrap().as_ptr();
    let path = PCSTR::from_raw(path);

    let ok = unsafe { CreateSymbolicLinkA(name, path, SYMBOLIC_LINK_FLAGS(0x2)) };

    if ok.into() {
        println!("Successfully created symlink!");
    } else {
        unsafe {
            let err = GetLastError();
            println!("{:#?}", err);
        }
    }
}
