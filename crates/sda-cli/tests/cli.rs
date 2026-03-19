use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

fn sda_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_sda"))
}

fn unique_temp_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();
    std::env::temp_dir().join(format!("axiom-sda-{name}-{}-{nanos}", std::process::id()))
}

#[test]
fn eval_reads_stdin_json() {
    let mut child = sda_bin()
        .args(["eval", "-e", "values(input)"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn sda");

    use std::io::Write;
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(br#"{"b":2,"a":1}"#)
        .expect("write stdin");
    drop(child.stdin.take());

    let output = child.wait_with_output().expect("wait output");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "[\n  1,\n  2\n]\n");
}

#[test]
fn eval_reads_source_and_input_files() {
    let source_path = unique_temp_path("program.sda");
    let input_path = unique_temp_path("input.json");
    fs::write(&source_path, "values(input)").expect("write source file");
    fs::write(&input_path, r#"{"z":3,"a":1,"m":2}"#).expect("write input file");

    let output = sda_bin()
        .args([
            "eval",
            "-f",
            source_path.to_str().expect("source path str"),
            "-i",
            input_path.to_str().expect("input path str"),
            "--compact",
        ])
        .output()
        .expect("run sda eval");

    let _ = fs::remove_file(&source_path);
    let _ = fs::remove_file(&input_path);

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "[1,2,3]\n");
}

#[test]
fn check_reports_ok_for_valid_source() {
    let output = sda_bin()
        .args(["check", "-e", "values(input)"])
        .output()
        .expect("run sda check");

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "ok\n");
}

#[test]
fn fmt_echoes_valid_source_stub() {
    let output = sda_bin()
        .args(["fmt", "-e", "  values(input)  "])
        .output()
        .expect("run sda fmt");

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "values(input)\n");
}

#[test]
fn check_exits_nonzero_for_invalid_source() {
    let output = sda_bin()
        .args(["check", "-e", "let x = ;"])
        .output()
        .expect("run invalid sda check");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("Error:"));
}