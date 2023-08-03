use std::io::Write;

static CONFIGS_FILE: &str = "configs.json";

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
