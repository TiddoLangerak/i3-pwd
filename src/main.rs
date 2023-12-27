use i3_ipc::{Connect, I3};
use std::{env, io};

fn main() -> io::Result<()> {
    let home = env::var("HOME").expect("Can't find home folder");
    let mut i3 = I3::connect()?;
    let workspaces = i3.get_workspaces()?;
    let pwd = workspaces
        .iter()
        .find(|workspace| workspace.focused)
        .and_then(|workspace| -> Option<String> {
            let ws_pwd = workspace.name.split_once(":")?.1;
            let resolved = ws_pwd.replacen("~", &home, 1);
            Some(resolved)
        })
        .unwrap_or(home);
    println!("{}", pwd);

    Ok(())
}
