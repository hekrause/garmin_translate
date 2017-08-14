
use nix::unistd::{fork, execvp, ForkResult};
use nix::sys::wait::waitpid;

use std;
use std::ffi::CString;

pub fn generate_csv_from_fit(src_filename: &str) {
    match fork() {
        Ok(ForkResult::Parent { child }) => {
            let _wait = waitpid(child, None);
        }
        Ok(ForkResult::Child) => {
            let mut argv: Vec<CString> = Vec::new();

            argv.push(CString::new("java").unwrap());
            argv.push(CString::new("-jar").unwrap());
            argv.push(CString::new("/home/henning/rust-programs/garmin_translator/src/converter/FitSDKRelease_20.41.00/java/FitCSVTool.jar").unwrap());
            argv.push(CString::new("-b").unwrap());
            argv.push(CString::new(src_filename).unwrap());
            argv.push(CString::new(src_filename.replace(".FIT", ".csv")).unwrap());

            let _result = execvp(&&argv[..][0], &argv[..]);
        }
        Err(_e) => {
            println!("Fork failed");
            std::process::exit(1);
        }
    }
}
