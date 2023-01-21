use api::{
    config::{GlobalConfig, CONFIG},
    folders::{ArticleFolder, PictureFolder},
    sync::{GlobalConnPool, NeedPublish, CONN_POOL, NEED_PUBLISH},
};

use clap::Parser;
use env_logger::Env;
use log::info;
use model::folders::{add_article_folder, add_picture_folder};
use r2d2_sqlite::SqliteConnectionManager;
use std::path::PathBuf;
use text_io::read;

mod api;
mod model;
mod notifier;
mod publisher;

#[derive(Debug, Parser)]
struct Args {
    #[clap(short = 'c', long = "config", value_parser)]
    config_file: Option<String>,
}

/// Initialize Main
/// Parse The Args, Read the Config, and init many others
/// including threads and global vars
fn init() {
    info!("Initializing");
    let args: Args = Args::parse();
    let f = PathBuf::from(args.config_file.unwrap_or(String::from("config.yaml")));

    let manager = SqliteConnectionManager::file("arce.db");
    let global_conn_pool = GlobalConnPool(r2d2::Pool::new(manager).unwrap());
    CONN_POOL.set(global_conn_pool).unwrap();

    let need_publish = NeedPublish::new(false);
    NEED_PUBLISH.set(need_publish).unwrap();

    let config = GlobalConfig::from_file(f).expect("Reading Config file failed");
    CONFIG.set(config).unwrap();

    crate::model::init().expect("Initialize DB failed");

    crate::notifier::init().expect("Initialize Article Notifier failed");

    crate::publisher::start();

    NeedPublish::global().set(true);

    info!("Initialized");
}

fn main() {
    // initializing logger to always level
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    init();

    // loop for adding folders
    // it's only a temporary solution
    loop {
        let t: i32 = read!();
        match t {
            // 1 means Article folder
            1 => {
                let conn = GlobalConnPool::global().0.get().unwrap();
                let path: String = read!();
                let path = PathBuf::from(path);
                let deploy: String = read!();
                let need_confirm: bool = read!();
                let f = ArticleFolder {
                    path,
                    deploy,
                    need_confirm,
                };
                add_article_folder(&conn, f).unwrap();
            }
            // 2 means Picture folder
            2 => {
                let conn = GlobalConnPool::global().0.get().unwrap();
                let path: String = read!();
                let path = PathBuf::from(path);
                let f = PictureFolder { path };
                add_picture_folder(&conn, f).unwrap();
            }
            _ => {}
        }
    }
}
