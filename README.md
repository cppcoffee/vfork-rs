# vfork-rs

`vfork-rs` is used in embedded low memory to run an external program and read the stdout output.

Just like the name, the `vfork-rs` uses the linux `vfork` syscall. the `vfork` syscall is used to create new processes without copying the page tables of the parent process. 


## Notice

Used in linux only.

## Usage

```rust
use vfork::Command;

fn main() {
    let s = "hello, world!";
    let mut cmd = Command::new("/bin/echo")
        .arg(s)
        .spawn()
        .expect("failed to execute process");

    let status_code = cmd.wait().expect("failed to wait process");
    assert_eq!(0, status_code.code());

    let output = cmd.output().expect("failed to get output");
    assert_eq!(String::from_utf8_lossy(&output), s);
}
```

## Reference

[https://man7.org/linux/man-pages/man2/vfork.2.html](https://man7.org/linux/man-pages/man2/vfork.2.html)

