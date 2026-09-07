use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../web/src");
    println!("cargo:rerun-if-changed=../web/index.html");
    println!("cargo:rerun-if-changed=../web/vite.config.ts");
    println!("cargo:rerun-if-changed=../web/package.json");

    let status = Command::new("pnpm")
        .args(["--filter", "@paugeran/web", "build"])
        .current_dir("../..")
        .status()
        .expect("pnpm diperlukan untuk membangun frontend embedded");
    if !status.success() {
        panic!("frontend build gagal; binary tidak dapat dibuat");
    }
}
