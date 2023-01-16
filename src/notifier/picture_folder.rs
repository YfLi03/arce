use crate::api::err;
use crate::api::folders::{PictureFolder, PictureFolderList};
use crate::api::pictures::PhotographyPicture;
use crate::api::sync::{ConnPool, NeedPublish};
use notify::event::CreateKind;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use regex::Regex;
use std::ffi::OsStr;
use std::fs::{read_to_string, DirEntry};
use std::path::PathBuf;
use std::thread;

// for articles, notifier use db operations directly

pub fn watch_folders(p_folders: PictureFolderList, pool: ConnPool) {
    for folder in p_folders {
        let pool = pool.clone();
        thread::spawn(|| {
            if let Err(e) = watch_picture_folder(folder, pool) {
                println!("error: {:?}", e);
            }
        });
    }
}

fn watch_picture_folder(
    folder: PictureFolder,
    pool: ConnPool,
) -> Result<(), err::Error> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(folder.path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        let event = match res {
            Ok(event) => event,
            Err(e) => return Err(e.into()),
        };

        match event.kind {
            EventKind::Create(CreateKind::File) => {
                if is_deploy_file(&event.paths[0]) {
                    let path = event.paths[0].ancestors().next().unwrap();
                    search_folder(path.to_path_buf())?;
                }
            }
            _ => {}
        }

        let signal = NeedPublish::global();
        signal.set(true);
    }

    Ok(())
}

fn search_folder(p: PathBuf) -> Result<(), err::Error> {
    let settings = read_to_string(p.join("config.txt")).unwrap_or_default();

    let files = p.read_dir()?;
    for file in files {
        let file = file?;
        if is_pic(&file.path()) && !search_flag("IGNORE", &file, &settings) {
            let mut pic = PhotographyPicture::from_dir(
                file.path(),
                search_flag("SELECTED", &file, &settings),
                search_text("LINK", &file, &settings),
                search_text("TITLE", &file, &settings).unwrap_or(
                    PathBuf::from(file.file_name())
                        .file_stem()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                ),
            )?;

            pic = pic.read_info()?.process_and_store()?;
            pic.register_and_upload()?;
        }
    }
    Ok(())
}

// TODO: may have some bugs here
fn is_pic(p: &PathBuf) -> bool {
    p.is_file()
        && vec!["jpg, jpeg, png, JPG, PNG, JPEG"]
            .contains(&p.extension().unwrap_or(OsStr::new("")).to_str().unwrap())
}

fn is_deploy_file(p: &PathBuf) -> bool {
    p.is_file() && p.file_name() == Some(OsStr::new("DEPLOY"))
}

fn search_flag(flag: &str, file: &DirEntry, settings: &str) -> bool {
    settings.contains(&(flag.to_string() + "[" + file.file_name().to_str().unwrap() + "]"))
}

// TODO
fn search_text(flag: &str, file: &DirEntry, settings: &str) -> Option<String> {
    let re: Regex = Regex::new(
        &(flag.to_string() + "\\[" + file.file_name().to_str().unwrap() + "\\]\\{([A-Za-z]+)\\}"),
    )
    .unwrap();
    for cap in re.captures_iter(settings) {
        return Some(cap[1].to_string());
    }
    None
}
