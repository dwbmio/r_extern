use std::{process::Command, env};

fn main() {
    let cur = env::current_dir().unwrap();
    println!("cur path is {}", cur.display());
    let core = Command::new("example/bin/luajit")
        .arg("example/lua/core.lua").output().expect("--[luatest]--\nfailed!");
    println!("{}\n--[luatest]-- suc!", String::from_utf8(core.stdout).unwrap());
    
}