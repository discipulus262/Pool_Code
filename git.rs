use std::process::Command;
pub fn push() {
    let _ga = Command::new("git").args(["add", "status.txt"]).output();
    let _result = Command::new("git")
        .args(["commit", "-m Status Changed"])
        .output()
        .expect("oops");
    let _gp = Command::new("git").args(["push"]).output().expect("oops");
    
}
pub fn pull() {
    let _result = Command::new("git").args(["pull"]).output().expect("oops");
}

