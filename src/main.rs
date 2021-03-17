/* Perf tests:
hyperfine --warmup 5 "defaults read $HOME/Library/Preferences/com.apple.HIToolbox.plist AppleSelectedInputSources" 'brishz.dash input-lang-get-icon' input_lang_get_icon
*/
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use cmd_lib::{proc_env_set, run_cmd, run_fun, CmdResult, FunResult};
use std::env;
use std::process::Command;

fn main() -> CmdResult {
    // let HOME = env::var("HOME").unwrap();
    let HOME = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
    
    // let mut lang: String = run_fun!(defaults read $HOME/Library/Preferences/com.apple.HIToolbox.plist AppleSelectedInputSources | /usr/local/bin/rg -e r#""KeyboardLayout Name" = "([^"]*)""# --replace r"$1" --only-matching --color never)?;

    let mut lang: String = run_fun!($HOME/bin/input_lang_get_objc)?;
    // let mut lang: String = run_fun!(/usr/local/bin/hs -A -c "hs.keycodes.currentSourceID()")?;
    lang = lang.to_lowercase();
    if lang == "u.s." || lang == "us" || lang == "com.apple.keylayout.us" {
        println!("🇺🇸" )
    } else if lang.contains("persian") {
        println!("🇮🇷" )
    } else {
        println!("{}", lang)
    }

    /* is not updated instantaneously
    // let exit_status = Exec::cmd("rg").args(&["--quiet", "-F", "LayoutTU.S.", format!("{}/Library/Preferences/com.apple.HIToolbox.plist", HOME)]).join()?;
    let exit_status = Command::new("rg").args(&["--quiet", "-F", "LayoutTU.S.", &format!("{}/Library/Preferences/com.apple.HIToolbox.plist", HOME)]).status().unwrap();
    if exit_status.success() {
        println!("🇺🇸" )
    } else {
        println!("🇮🇷" )
    }
     */

    Ok(())
}
