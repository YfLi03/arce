use crate::api::err;
use crate::api::folders::{PictureFolder, PictureFolderList};
use crate::api::pictures::PhotographyPicture;
use crate::api::sync::NeedPublish;

use log::info;
use notify::event::CreateKind;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use regex::Regex;

use std::ffi::OsStr;
use std::fs::{read_to_string, DirEntry};
use std::path::PathBuf;
use std::thread;

/// initializing notifier for picture folders
pub fn watch_folders(p_folders: PictureFolderList) {
    for folder in p_folders {
        thread::spawn(|| {
            if let Err(e) = watch_picture_folder(folder) {
                println!("error: {:?}", e);
            }
        });
    }
}

/// notifier for a single picture folder
fn watch_picture_folder(folder: PictureFolder) -> Result<(), err::Error> {
    info!("Watching Picture Folder {:?}", folder);

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(folder.path.as_ref(), RecursiveMode::Recursive)?; // picture folders are monitored recursively

    for res in rx {
        let event = match res {
            Ok(event) => event,
            Err(e) => return Err(e.into()),
        };

        // Pictures are published only if a DEPLOY file is found in the same directory
        // with the keyword DEPLOY inside the file
        match event.kind {
            EventKind::Create(CreateKind::File) => {
                if is_deploy_file(&event.paths[0]) && find_deploy_flag(&event.paths[0])? {
                    let path = event.paths[0].ancestors().next().unwrap().parent().unwrap();
                    search_folder(path.to_path_buf())?;
                } else {
                    continue;
                }
            }
            EventKind::Modify(_) => {
                if is_deploy_file(&event.paths[0]) && find_deploy_flag(&event.paths[0])? {
                    let path = event.paths[0].ancestors().next().unwrap().parent().unwrap();
                    search_folder(path.to_path_buf())?;
                } else {
                    continue;
                }
            }
            _ => {
                continue;
            }
        }

        // those unexpected modifications will be filtered and won't reach here
        let signal = NeedPublish::global();
        signal.set(true);
    }

    Ok(())
}

/// searching for pictures inside a folder
fn search_folder(p: PathBuf) -> Result<(), err::Error> {
    info!("Searching Picture Folder {:?}", p);

    // the deploy file also acts as the config/setting file
    let settings = read_to_string(p.join("DEPLOY")).unwrap_or_default();

    let files = p.read_dir()?;
    for file in files {
        let file = file?;
        info!("Searching file{:?}", file);

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
                        .to_string(), // the default title is its file name ( without extension )
                ),
            )?;

            info!("Getting Pic {:?}", pic);

            // we don't need to do anything if it already exists in database
            if pic.is_registered()? {
                continue;
            }

            // read exif and copy it to the local picture folder
            // it will be renamed, and compressed if necessary
            pic = pic.read_info()?.process_and_store()?;

            // store its info in the database and upload it to the server using scp
            pic.register_and_upload()?;
        }
    }
    Ok(())
}

// Determine whether a file is a picture according to its extension
fn is_pic(p: &PathBuf) -> bool {
    p.is_file()
        && vec!["jpg", "jpeg", "png", "JPG", "PNG", "JPEG"]
            .contains(&p.extension().unwrap_or(OsStr::new("")).to_str().unwrap())
}

fn is_deploy_file(p: &PathBuf) -> bool {
    p.is_file() && p.file_name() == Some(OsStr::new("DEPLOY"))
}

/// SEARCHING for bool Flags
fn search_flag(flag: &str, file: &DirEntry, settings: &str) -> bool {
    settings.contains(&(flag.to_string() + "[" + file.file_name().to_str().unwrap() + "]"))
}

/// SEARCHING for linking Flags such as SELECTED[xxx.jpg]{yyy.com}
/// settings is the string content which you search
fn search_text(flag: &str, file: &DirEntry, settings: &str) -> Option<String> {
    let re: Regex = Regex::new(
        &(flag.to_string() + "\\[" + file.file_name().to_str().unwrap() + "\\]\\{([\\s\\S]*?)\\}"),
    )
    .unwrap();

    // return the first pattern found
    for cap in re.captures_iter(settings) {
        return Some(cap[1].to_string());
    }

    None
}

/// finding the DEPLOY keyword inside a file
fn find_deploy_flag(path: &PathBuf) -> Result<bool, err::Error> {
    Ok(read_to_string(path)?.find("DEPLOY").is_some())
}
