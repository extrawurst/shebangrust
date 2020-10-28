use std::process::Command;

fn main() {
    let output = Command::new("sh")
            .arg("-c")
            .arg("./test")
            .output()
            .expect("failed to execute process");

    println!("out: {:?}",String::from_utf8_lossy(output.stdout.as_ref()));
    println!("err: {:?}",String::from_utf8_lossy(output.stderr.as_ref()));
}
