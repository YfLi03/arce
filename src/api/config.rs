use std::path::PathBuf;

pub struct GlobalConfig {
    pub title: String,
    pub subtitle: String,
    pub foot: String,
    pub bei_an: Option<String>,

    pub pic_cloud_prefix: String,
    pub pic_local: PathBuf,
    pub pic_compress_threshold: usize,

    pub scp_server: String,
    pub scp_pic_path: String,
    pub scp_web_path: String,

    pub deploy_auto: bool,
    pub deploy_interval: Option<usize>,
}
