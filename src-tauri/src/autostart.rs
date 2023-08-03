use std::io::Write;
use std::process::Output;

#[cfg(target_os = "macos")]
const AUTOSTART_MAC: &str = " \n\
<?xml version=\"1.0\" encoding=\"UTF-8\"?> \n\
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\"> \n\
<plist version=\"1.0\"> \n\
\t  <dict> \n\
\t\t    <key>Label</key> \n\
\t\t    <string>org.oxbox.docacrm</string> \n\
\t\t    <key>ProgramArguments</key> \n\
\t\t    <array> \n\
\t\t\t       <string>/Applications/DocaCRM.app/Contents/MacOS/DocaCRM</string> \n\
\t\t    </array> \n\
\t\t    <key>RunAtLoad</key> \n\
\t\t    <true/> \n\
\t  </dict> \n\
</plist>";
#[cfg(target_os = "windows")]
const AUTOSTART_WIN: &str = r#"HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Run"#;

#[cfg(target_os = "windows")]
pub fn add_to_autostart(path: String) -> std::io::Result<()> {
    Ok(())
}


#[cfg(target_os = "windows")]
pub fn remove_from_autostart() -> std::io::Result<()> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn is_configured() -> std::io::Result<bool> {
    Ok(true);
}



#[cfg(target_os = "macos")]
pub fn add_to_autostart() -> std::io::Result<()> {
    let path = dirs::home_dir().unwrap()
        .join("Library")
        .join("LaunchAgents")
        .join("DocaCRM.plist");
    let mut file = std::fs::File::create(path)?;
    file.write(AUTOSTART_MAC.as_bytes())?;
    let properties = std::format!(
        "{{name:\"{}\",path:\"{}\",hidden:{}}}",
        "DocaCRM",
        "/Applications/DocaCRM.app",
        false
    );
    let command = std::format!("make login item at end with properties {}", properties);
    exec_script(&command).expect("[-] Can't add to autorun");
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn remove_from_autostart() -> std::io::Result<()> {
let path = dirs::home_dir().unwrap()
        .join("Library")
        .join("LaunchAgents")
        .join("DocaCRM.plist");
    std::fs::remove_file(path)?;
    let command = format!("delete login item \"{}\"", "DocaCRM");
    exec_script(&command)?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn is_configured() -> std::io::Result<bool> {
    let mut is_configured = false;
    let command = "get the name of every login item";
    let output  = exec_script(command).expect("[-] ");
    if output.status.success() {
        let result = std::str::from_utf8(&output.stdout).unwrap_or("");
        let mut result = result.split(",").map(|x| x.trim());
        is_configured = result.find(|x| x == &"DocaCRM").is_some();
    }
    Ok(is_configured)
}

#[cfg(target_os = "macos")]
fn exec_script(suffix: &str) -> std::io::Result<Output> {
    let preffix = "tell application \"System Events\" to";
    let command = std::format!("{} {}", preffix, suffix);
    let out = std::process::Command::new("osascript")
        .arg("-e")
        .arg(&command)
        .output()?;
    Ok(out)
}
