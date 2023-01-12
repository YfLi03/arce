use notify::event::{CreateKind, RemoveKind, ModifyKind};
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config, EventKind};
use crate::api::articles::find_deploy_flag;
use crate::api::err;
use crate::api::folders::{ArticleFolderList, ArticleFolder};
use crate::api::sync::{NeedPublish, ConnPool};
use crate::model::articles::{update_article, delete_article};
use std::ffi::{OsString};
use std::path::{Path, PathBuf};
use std::thread;

pub fn watch_folders(a_folders: ArticleFolderList, signal: NeedPublish, pool: ConnPool) {
    for folder in a_folders {
        let pool = pool.clone();
        let signal = signal.clone();
        thread::spawn(||{
            if let Err(e) = watch_article_folder(folder, pool, signal){
                println!("error: {:?}", e);
            }
        });
    }
}


fn watch_article_folder(folder: ArticleFolder, pool: ConnPool, signal: NeedPublish) -> Result<(), err::Error>{
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(folder.path.as_ref(), RecursiveMode::NonRecursive)?;

    for res in rx {
        let event = 
            match res {
                Ok(event) => event,
                Err(e) => return Err(e.into()),
            };
        
        match event.kind {
            EventKind::Create(CreateKind::File) => {
                add_article(event.paths[0].clone(), &folder, &pool)?;
            },
            EventKind::Modify(ModifyKind::Data(_)) => {
                add_article(event.paths[0].clone(), &folder, &pool)?;
            },
            EventKind::Modify(ModifyKind::Name(_)) => {
                remove_article(event.paths[0].clone(), &pool)?;
                add_article(event.paths[1].clone(), &folder, &pool)?;
            },
            EventKind::Remove(RemoveKind::File) => {
                remove_article(event.paths[0].clone(), &pool)?;
            },
            _  => {}
        }

        signal.set(true);
    }

    Ok(())
}


fn is_markdown(p: &PathBuf) -> bool {
    p.is_file() && 
    p.extension()
     .unwrap_or(&OsString::new())
     == "md"
}

// for articles, notifier use db operations directly

fn add_article(p: PathBuf, f: &ArticleFolder, pool: &ConnPool) -> Result<(), err::Error>{
    if !is_markdown(&p) {return Ok(())};
    if f.need_confirm && !find_deploy_flag(&p)? {return Ok(())};
    update_article(&pool.get().unwrap(), (p, f.deploy.clone()).into())?;
    Ok(())
}

fn remove_article(p: PathBuf, pool: &ConnPool) -> Result<(), err::Error>{
    if !is_markdown(&p) {return Ok(())};
    delete_article(&pool.get().unwrap(), p)?;
    Ok(())
}
