use std::ffi::CString;
use std::path::Path;
use std::process::{Command, Stdio};
use termcolor::Color;
use tokio::fs;
use bungee_script::{backup, web, config, cli, server, util::{java, logger, runner}};

#[cfg(target_os = "windows")]
fn windows() {
    use std::ptr;
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::SetWindowTextA;

    let window = unsafe { GetConsoleWindow() };
    if window != ptr::null_mut() {
        unsafe {
            let cstr = CString::new("Server Script").unwrap();
            SetWindowTextA(window, cstr.as_ptr());
            
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn windows() {

}

fn convert_string(str: &str) -> CString {
    CString::new(str).unwrap()
}

#[cfg(target_os = "windows")]
fn spawn(artifact: &String, script: &String, directory: &String) {
    use winapi::um::{shellapi::ShellExecuteA, winuser::SW_NORMAL};
    let runas = convert_string("open");
    let parameters = convert_string(script.split(" ").skip(1).collect::<Vec<&str>>().join(" ").as_str());
    let program = convert_string(artifact.as_str());
    let dir = convert_string(&directory);

    unsafe {
        ShellExecuteA(
            std::ptr::null_mut(), 
            runas.as_ptr(), 
            program.as_ptr(), 
            parameters.as_ptr(), 
            dir.as_ptr(), 
            SW_NORMAL);
    };
}

#[cfg(not(target_os = "windows"))]
fn spawn() {
    panic!("unimplemented")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    windows();

    print!("[Logger] ");
    logger::log("Running bungee-script v1.0.0", Some(Color::Cyan), None);
    print!("[Logger] ");
    logger::log("Report bugs here: https://github.com/dolphin2410/bungee-script", Some(Color::Cyan), None);

    let cli = cli::parse();

    // Loads the config
    let mut configuration = config::load_config().await?;

    configuration.apply(&cli);

    let jarfile = "waterfall.jar".to_string();

    let jar_path = Path::new(&jarfile);

    if !jar_path.exists() || !configuration.no_update {
        // Download the jar
        web::download_server(&configuration, &jarfile).await.unwrap();
    }

    let executable = java::find_executable();

    let args = runner::default_args(&jarfile, &configuration);

    loop {
        let plugins_path = Path::new("plugins");
        if !plugins_path.exists() {
            fs::create_dir(plugins_path).await?;
        }

        // Download plugins
        for plugin in configuration.plugins.to_vec() {
            let file_name = plugin.split("/").last().unwrap();
            web::download(&plugin,&format!("plugins/{}", file_name)).await?;
        }

        for (key, script) in configuration.servers.clone() {
            let artifact = server::parse_artifact(&script).await;
            server::prepare(&key, &artifact).await?;
            spawn(&server::artifact_of(&artifact, &key), &script, &server::directory_of(&key));
        }

        // Execute the program
        Command::new(&executable)
            .args(&args)
            .stdout(Stdio::inherit())
            .spawn()
            .unwrap().wait().unwrap();

        if configuration.backup {
            logger::log("Starting Backup...", None, None);
            backup::backup().await?;
        }

        if !&configuration.restart {
            break;
        }

        logger::log("Restarting...", None, None);
    }

    logger::log("Exiting...", None, None);

    Ok(())
}