use std::{collections::HashSet, path::PathBuf, thread::{self, Thread}};
use clap::{ArgAction, Parser};
use api::{config::{CONFIG, GlobalConfig}, sync::{ConnPool, GlobalConnPool, CONN_POOL, NeedPublish, NEED_PUBLISH}};
use r2d2_sqlite::SqliteConnectionManager;

mod api;
mod model;
mod notifier;
mod publisher;

#[derive(Debug, Parser)]
struct Args {
    #[clap(short='c', long = "config", value_parser)]
    config_file: Option<String>
}

fn init(){
    let args : Args = Args::parse();
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

    // init renderer
}

fn main() {
    init();
    while true {

    }
}
