use crate::api::articles::find_deploy_flag;
use crate::api::err;
use crate::api::folders::{ArticleFolder, ArticleFolderList};
use crate::api::sync::{ConnPool, NeedPublish};
use crate::model::articles::{delete_article, update_article};
use log::info;
use notify::event::{CreateKind, ModifyKind, RemoveKind};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::ffi::OsString;
use std::path::PathBuf;
use std::thread;

pub fn watch_folders(a_folders: ArticleFolderList, pool: ConnPool) {
    for folder in a_folders {
        let pool = pool.clone();
        thread::spawn(|| {
            if let Err(e) = watch_article_folder(folder, pool) {
                println!("error: {:?}", e);
            }
        });
    }
}

fn watch_article_folder(folder: ArticleFolder, pool: ConnPool) -> Result<(), err::Error> {
    info!("Initing Article Folder {:?}", &folder);
    let (tx, rx) = std::sync::mpsc::channel();
    let files = folder.path.read_dir()?;
    for file in files {
        let file = file?;
        add_article(file.path(), &folder, &pool)?;
    }

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(folder.path.as_ref(), RecursiveMode::NonRecursive)?;

    info!("Monitoring Article Folder {:?}", &folder);
    for res in rx {
        let event = match res {
            Ok(event) => event,
            Err(e) => return Err(e.into()),
        };

        info!("Event noticed {:?}", event);

        match event.kind {
            EventKind::Create(CreateKind::File) => {
                add_article(event.paths[0].clone(), &folder, &pool)?;
            }
            EventKind::Modify(_) => {
                if event.paths.len() == 1 {
                    add_article(event.paths[0].clone(), &folder, &pool)?;
                    continue;
                }
                remove_article(event.paths[0].clone(), &pool)?;
                add_article(event.paths[1].clone(), &folder, &pool)?;
            }
            EventKind::Remove(RemoveKind::File) => {
                remove_article(event.paths[0].clone(), &pool)?;
            }
            _ => {}
        }

        let signal = NeedPublish::global();
        signal.set(true);
    }

    Ok(())
}

fn is_markdown(p: &PathBuf) -> bool {
    p.is_file() && p.extension().unwrap_or(&OsString::new()) == "md"
}

// for articles, notifier use db operations directly

fn add_article(p: PathBuf, f: &ArticleFolder, pool: &ConnPool) -> Result<(), err::Error> {
    info!("Adding Article {:?}", p);
    if !is_markdown(&p) {
        return Ok(());
    };
    if f.need_confirm && !find_deploy_flag(&p)? {
        return Ok(());
    };
    update_article(&pool.get().unwrap(), (p, f.deploy.clone()).into())?;
    Ok(())
}

fn remove_article(p: PathBuf, pool: &ConnPool) -> Result<(), err::Error> {
    info!("Removing Article {:?}", p);
    if !is_markdown(&p) {
        return Ok(());
    };
    delete_article(&pool.get().unwrap(), p)?;
    Ok(())
}
