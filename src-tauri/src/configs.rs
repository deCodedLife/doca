use std::io::Write;


const CONFIGS_FILE: &str = "configs.json";
const AUTOSTART_MAC: &str = " \n\
<?xml version=\"1.0\" encoding=\"UTF-8\"?> \n\
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\"> \n\
<plist version=\"1.0\"> \n\
  <dict> \n\
    <key>Label</key> \n\
    <string>org.oxbox.docacrm</string> \n\
    <key>ProgramArguments</key> \n\
    <array> \n\
       <string>/Applications/DocaCRM.app/Contents/MacOS/DocaCRM</string> \n\
    </array> \n\
    <key>RunAtLoad</key> \n\
    <true/> \n\
  </dict> \n\
</plist>";
const AUTOSTART_WIN: &str = r#"HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Run"#;


// App configuration.
#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Config {
    pub cashbox_id: String,
    pub api_url: String,
    pub atol_url: String,
    pub crm_url: String,
    pub jwt_token: String,
    pub jwt_username: String,
    pub jwt_password: String,
    pub app_path: String,
    pub terminal_path: String,
    pub autorun: bool
}


// Returns default app configuration
fn get_default() -> Config {
    Config {
        cashbox_id: "1_yashlek".to_string(),
        api_url: "https://dev.docacrm.com".to_string(),
        atol_url: "http://localhost:16732".to_string(),
        crm_url: "https://dev.docacrm.ru".to_string(),
        jwt_token: "".to_string(),
        jwt_username: "support@oxbox.ru".to_string(),
        jwt_password: "IDEqRe1X4tPQDubh".to_string(),
        app_path: std::env::current_exe().unwrap().display().to_string(),
        terminal_path: "C:/sc552".to_string(),
        autorun: false
    }
}


#[cfg(target_os = "windows")]
pub fn add_to_autostart(path: String) {

}

#[cfg(target_os = "macos")]
pub fn add_to_autostart() -> std::io::Result<()> {
    let file = std::fs::File::create("~/Library/LaunchAgents/org.oxbox.docacrm.plist");
    file.unwrap().write_all(AUTOSTART_MAC.as_bytes())?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn remove_from_autostart() -> std::io::Result<()> {
    std::fs::remove_file("~/Library/LaunchAgents/org/oxbox.oxbox.docacrm.plist")
}


fn write_config(conf: Config, path: String) -> std::io::Result<()> {
    let file = std::fs::File::create(path)?;
    let mut writer = std::io::BufWriter::new(file);
    serde_json::to_writer(&mut writer, &conf)?;
    writer.flush()?;
    Ok(())
}

fn read_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(path)?;
    Ok(data)
}

fn read_config(path: &str) -> Config {
    let read_result = read_file(path);
    let config_content: String = match read_result {
        Ok(res) =>res,
        Err(e) => {
            eprintln!("[-] {:?}", e);
            serde_json::to_string(&get_config()).unwrap()
        }
    };
    serde_json::from_str(&*config_content).unwrap()
}

pub fn get_config() -> Config {
    let mut configs_path = std::env::current_exe().unwrap();
    configs_path.pop();
    configs_path.push(CONFIGS_FILE);
    if configs_path.exists() {
        return read_config(configs_path.display().to_string().as_str())
    }
    let write_result = write_config(get_default(), configs_path.display().to_string());
    match write_result {
        Err(e) => eprintln!("[-] {:?}", e),
        _ => ()
    }
    get_default()
}
