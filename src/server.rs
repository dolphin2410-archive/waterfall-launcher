pub async fn prepare(directory: &String, artifact: &String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = std::env::current_exe().unwrap().join(format!("../../{}", directory));

    if !path.exists() {
        tokio::fs::create_dir_all(path.clone()).await?;
    }

    let executable = path.join(artifact);

    if !executable.exists() {
        crate::web::download_progress(
            &"https://github.com/dolphin2410/server-script/releases/download/2.0.1/server-script-windows.exe".to_string(), 
            &executable.to_str().unwrap().to_string()).await?;
        println!("Download Complete!!");
    }

    Ok(())
}

pub async fn parse_artifact(shell: &String) -> String {
    let executable = shell.split(" ").nth(0).unwrap().chars().filter(|c| { !"\\/".contains(c.to_owned()) }).collect::<String>();
    
    executable.trim_start_matches('.').to_string()
}

pub fn artifact_of(artifact: &String, server: &String) -> String {
    let path = std::env::current_exe().unwrap().join(format!("../../{}", server)).join(artifact);

    path.to_str().unwrap().to_string()
}

pub fn directory_of(server: &String) -> String {
    let path = std::env::current_exe().unwrap().join(format!("../../{}", server));

    path.to_str().unwrap().to_string()
}