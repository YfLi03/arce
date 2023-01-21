use crate::api::config::GlobalConfig;
use log::{info, warn};
use std::process::Command;

/// deploy the site by copying the files to the server
pub fn deploy() {
    info!("Deploying");
    let config = GlobalConfig::global();
    let dst = config.scp_server.clone() + ":" + &config.scp_web_path;
    match Command::new("scp")
        .arg("-r")
        .arg("public/")
        .arg(&dst)
        .output()
    {
        Err(e) => {
            warn!("DEPLOY FAILED due to {:?}", e);
        }
        _ => {}
    }
    info!("Deployed");
}
