//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
//! Placing a file named build.rs in the root of a package will cause Cargo to
//! compile that script and execute it just before building the package.

//? Just before a package is built, Cargo will compile a build script into an executable
//? It will then run a script, which may perform any number of tasks
//? The script may communicate with Cargo by printing specially formatted commands prefixed 
//? with cargo:: to stdout
//? Once the build script successfully finishes executing, the rest of the package will be compiled.
//? Scripts should exit with a non-zero exit code to halt the build if there is an error,
//? in which case the build script's output will be displayed on the terminal.

use std::fmt::format;

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    let your_command = format!(
        "rustc-env=TEST_FOO={}",
        timestamp
    );
    println!("cargo:{}", your_command);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    let your_command = "rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", your_command);
}
