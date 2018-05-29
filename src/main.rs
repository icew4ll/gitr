#[macro_use]
extern crate dotenv_codegen;

#[macro_use] extern crate duct;

fn main() {
    println!("Hello, world!");
    let msg = cmd!("sh", "-c", "cd /home/fish/m/gitr && ls").stderr_to_stdout().read().unwrap();
    println!("{}", msg);
    getvar("test".to_string());
}

fn getvar(var: String) {
    // environment variables {{{
    println!("{}", var);
    let home = dotenv!("HOME");
    println!("{}", home);
    // }}}
}
