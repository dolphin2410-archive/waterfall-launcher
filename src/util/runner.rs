use crate::config::Configuration;

/// Generates the default running arguments
pub fn default_args<'a>(jarfile: &'a str, config: &'a Configuration) -> Vec<String> {
    let mut default = vec![
        format!("-Xmx{}M", config.memory).as_str(),
        format!("-Xms{}M", config.memory).as_str(),
        "-XX:+UseG1GC", 
        "-XX:G1HeapRegionSize=4M", 
        "-XX:+UnlockExperimentalVMOptions", 
        "-XX:+ParallelRefProcEnabled", 
        "-XX:+AlwaysPreTouch"
    ].into_iter().map(String::from).collect::<Vec<String>>();

    println!("Launching Server...");

    default.append(&mut vec![
        "-jar",
        jarfile
    ].into_iter().map(String::from).collect::<Vec<String>>());

    default
}