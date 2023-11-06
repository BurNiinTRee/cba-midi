fn main() {
    let mut path = std::env::current_exe().expect("Couldn't locate self");
    path.pop();
    path.push("bin");
    path.push(String::from("cba-midi") + std::env::consts::EXE_SUFFIX);
    std::process::Command::new(path)
        .args(std::env::args().skip(1))
        .spawn()
        .expect("Couldn't launch cba-midi")
        .wait()
        .expect("cba-midi failed");
}
