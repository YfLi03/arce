use crate::api::articles::ArticleInfo;
use crate::api::err;
use crate::api::folders::{ArticleFolder, ArticleFolderList};
use crate::api::sync::{ConnPool, GlobalConnPool, NeedPublish};
use crate::model::articles::{delete_article, update_article};

use log::info;
use notify::event::{CreateKind, RemoveKind};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use std::ffi::OsString;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::thread;

/// initializing notifier for article folders
pub fn watch_folders(a_folders: ArticleFolderList) {
    let pool = GlobalConnPool::global().0.clone();
    for folder in a_folders {
        let pool = pool.clone();
        thread::spawn(|| {
            if let Err(e) = watch_article_folder(folder, pool) {
                println!("error: {:?}", e);
            }
        });
    }
}

/// notifier for a single article folder
fn watch_article_folder(folder: ArticleFolder, pool: ConnPool) -> Result<(), err::Error> {
    info!("Initializing Article Folder {:?}", &folder);

    let (tx, rx) = std::sync::mpsc::channel();
    let files = folder.path.read_dir()?;

    // all files exist at init will be added to the database
    for file in files {
        let file = file?;
        add_article(file.path(), &folder, &pool)?;
    }

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(folder.path.as_ref(), RecursiveMode::NonRecursive)?; // article folders are monitored non-recursively

    info!("Monitoring Article Folder {:?}", &folder);
    for res in rx {
        let event = match res {
            Ok(event) => event,
            Err(e) => return Err(e.into()),
        };

        // notifiers' event classification seems not to be working well.
        // deleting may not work, modifications may have wrong modifykind
        match event.kind {
            EventKind::Create(CreateKind::File) => {
                add_article(event.paths[0].clone(), &folder, &pool)?;
            }
            EventKind::Modify(_) => {
                if event.paths.len() == 1 {
                    add_article(event.paths[0].clone(), &folder, &pool)?;
                } else {
                    remove_article(event.paths[0].clone(), &pool)?;
                    add_article(event.paths[1].clone(), &folder, &pool)?;
                }
            }
            EventKind::Remove(RemoveKind::File) => {
                remove_article(event.paths[0].clone(), &pool)?;
            }
            _ => {}
        }

        // currently, all actions in the monitored folder will lead to a new publish
        let signal = NeedPublish::global();
        signal.set(true);
    }

    Ok(())
}

/// derterming whether it's markdown by comparing its extension
fn is_markdown(p: &PathBuf) -> bool {
    p.is_file() && p.extension().unwrap_or(&OsString::new()) == "md"
}

fn find_deploy_flag(path: &PathBuf) -> Result<bool, err::Error> {
    Ok(read_to_string(path)?.find("deploy: true").is_some())
}

/// updating an article if necessary
fn add_article(p: PathBuf, f: &ArticleFolder, pool: &ConnPool) -> Result<(), err::Error> {
    info!("Adding Article {:?}", p);

    // if you have soft-linked files or whatever (like onedrive)
    // two unique path for the same file may be captured
    if !p.starts_with(&f.path) {
        return Ok(());
    };

    if !is_markdown(&p) {
        return Ok(());
    };
    if f.need_confirm && !find_deploy_flag(&p)? {
        return Ok(());
    };

    update_article(&pool.get().unwrap(), ArticleInfo::new(p, f.deploy.clone()))?;
    Ok(())
}

/// deleting an article if necessary. may not work properly
fn remove_article(p: PathBuf, pool: &ConnPool) -> Result<(), err::Error> {
    info!("Removing Article {:?}", p);
    if !is_markdown(&p) {
        return Ok(());
    };
    delete_article(&pool.get().unwrap(), p)?;
    Ok(())
}
