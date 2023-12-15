fn main() {

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-search=/opt/homebrew/opt/ncurses/lib");
    
    println!("cargo:rustc-link-lib=ncursesw");

}