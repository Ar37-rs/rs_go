use std::process::Command;

fn main() {
    let go_filename = "rs_go";
    let path = go_filename;
    let rsbuildmode = "debug";
    let ext = "dll";
    println!("cargo:rustc-link-search=target/{0}", rsbuildmode);
    println!("cargo:rustc-link-lib={0}", go_filename);
    Command::new("go")
        .args(&[
            "build",
            "-buildmode=c-shared",
            "-o",
            &format!("target/{0}/{1}.{2}", rsbuildmode, go_filename, ext),
            &format!("src/{0}/{1}.go", path, go_filename),
        ])
        .spawn()
        .expect("failed to execute process");

    // release command args
    // Command::new("go")
    //     .args(&[
    //         "build",
    //         "-trimpath",
    //         "-buildmode=c-shared",
    //         "-ldflags=-s",
    //         "-o",
    //         &format!("target/{0}/{1}.{2}", rsbuildmode, go_filename, ext),
    //         &format!("src/{0}/{1}.go", path, go_filename),
    //     ])
    //     .spawn()
    //     .expect("failed to execute process");
}
