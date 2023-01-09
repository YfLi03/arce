use notify::event::{CreateKind, RemoveKind, ModifyKind};
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config, EventKind};
use crate::api::folders::{ArticleFolder};
use crate::api::sync::{NeedPublish};
use std::path::Path;
use std::thread;

pub fn watch_folders(folders: Vec<ArticleFolder>, signal: NeedPublish) {
    
    for folder in folders {
        thread::spawn(||{
            if let Err(e) = watch_article_folder(folder){
                println!("error: {:?}", e);
            }
        });
    }
    
    //
}

fn watch_article_folder(folder: ArticleFolder) -> notify::Result<()>{
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(folder.path.as_ref(), RecursiveMode::NonRecursive)?;

    for res in rx {
        let event = 
            match res {
                Ok(event) => event,
                Err(e) => return Err(e),
            };
        match event.kind {
            EventKind::Create(CreateKind::File) => {
                
                // whether markdown

                // whether need to look for publish

                // 

            },
            EventKind::Modify(ModifyKind::Data(_Content)) => {

            },
            EventKind::Remove(RemoveKind::File) => {

            },

            _  => {}
        }
    }

    Ok(())
}
