use vfork::Command;

fn main() {
    let mut cmd = Command::new("/bin/echo")
        .arg("Hello, world!")
        .spawn()
        .expect("failed to execute process");

    cmd.wait().expect("failed to wait process");

    let output = cmd.output().expect("failed to get output");
    println!("output: {:?}", String::from_utf8_lossy(&output));
}
