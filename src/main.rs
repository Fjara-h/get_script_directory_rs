#![allow(unused_parens)]

// exitcode values to indicate issue
// Unsafe unwraps?
// add check IS_SUPPORTED_SYSTEM

use clap::Parser;
use std::{
    path::PathBuf,
    process::{ExitCode, id},
};
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System, UpdateKind};

// Replace `SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )`

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Canonicalize the final path before outputting
    #[arg(short = 'c', long = "canonical", default_value_t = false)]
    canonicalize: bool,

    /// Path to file or directory
    #[arg(required = false)]
    path: Option<PathBuf>,
}

fn main() -> ExitCode {
    let args: Args = Args::parse();
    let mut input_path: PathBuf = PathBuf::new();
    match args.path {
        None => {
            if let Ok(caller_path) = get_caller_path() {
                input_path = caller_path;
            }
        }
        Some(path) => {
            if let Ok(_) = path.try_exists() {
                input_path = path;
            } else {
                return ExitCode::FAILURE;
            }
        }
    };
    if let Ok(path) = get_parent_dir(input_path, args.canonicalize) {
        println!("{:?}", path);
        return ExitCode::SUCCESS;
    }

    return ExitCode::FAILURE;
}

fn get_caller_path() -> Result<PathBuf, &'static str> {
    // .with_cwd(UpdateKind::Always)
    let sys: System = System::new_with_specifics(
        RefreshKind::nothing().with_processes(
            ProcessRefreshKind::nothing()
                .with_environ(UpdateKind::Always)
                .with_exe(UpdateKind::Always),
        ),
    );
    if let Some(proc) = sys.process(Pid::from(id() as usize)) {
        if let Some(pproc_pid) = proc.parent() {
            if let Some(pproc) = sys.process(pproc_pid) {
                if let Some(asfs) = pproc
                    .environ()
                    .iter()
                    .map(|i| i.as_os_str().to_str().unwrap())
                    .filter(|i| i.starts_with("_="))
                    .next()
                {
                    return Ok(PathBuf::from(asfs.split("=").last().unwrap()));
                }

                if let Some(executable_path) = pproc.exe() {
                    return Ok(executable_path.to_path_buf());
                }
            };
        };
    };
    return Err("No caller process found");
}

fn get_parent_dir(path: PathBuf, canonicalize: bool) -> Result<PathBuf, &'static str> {
    if (path.is_empty()) {
        return Err("No parent path.");
    }
    let mut parent_path: PathBuf = path;
    if (canonicalize) {
        if let Ok(canonical_parent) = parent_path.canonicalize() {
            parent_path = canonical_parent;
        }
    }

    while let Some(parent_dir) = parent_path.parent() {
        if (parent_dir.ends_with("..") || parent_dir.ends_with(".")) {
            match parent_path.parent() {
                None => {
                    return Err("Parent path invalid.");
                }
                Some(new_parent) => {
                    return Ok(new_parent.to_path_buf());
                }
            }
        } else {
            return Ok(parent_dir.to_path_buf());
        }
    }

    return Err("Parent does not exist.");
}
