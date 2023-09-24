use vfork::Command;

fn main() {
    let mut cmd = Command::new("/bin/echo")
        .arg("Hello, world!")
        .spawn()
        .expect("failed to execute process");

    let status_code = cmd.wait().expect("failed to wait process");
    println!("status code: {:?}", status_code);

    let output = cmd.output().expect("failed to get output");
    println!("output: {:?}", String::from_utf8_lossy(&output));
}
