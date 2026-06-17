use std::process::Command;

fn main() {
    /* re-run if any commits happen */
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs");
    
    /* get the most recent commit hash */
    let commit = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .map(|o| String::from_utf8(o.stdout).unwrap_or_default())
        .unwrap_or_else(|_| String::from("unknown"));
    
    /* set the environment variable with our commit hash in it */
    println!("cargo:rustc-env=GIT_COMMIT={}", commit);
}
