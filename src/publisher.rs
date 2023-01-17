use lazy_static::lazy_static;
use log::{info, warn};
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use serde::Serialize;
use slug::slugify;
use std::{
    cmp::min,
    fs::{copy, create_dir_all},
    io::Write,
    path::PathBuf,
    process::Command,
    thread::{self, sleep},
    time::Duration,
};
use tera::{Context, Tera};

use crate::{
    api::{
        articles::{Article, ArticleInfo, ArticleYaml},
        config::{self, GlobalConfig, CONFIG},
        err,
        pictures::{self, PhotographyPicture, PhotographyPictureBrief, Picture},
        sync::{GlobalConnPool, NeedPublish},
    },
    model::{articles::get_articles, pictures::get_photography_pictures},
};

const FOLDERS: [&str; 5] = [
    "public/index",
    "public/gallery",
    "public/css",
    "public/picture",
    "public/category",
];

#[derive(Serialize, Clone)]
struct Page {
    pub title: String,
    pub tcolor: Vec<String>,
}

impl Page {
    fn new(i: usize, s: String) -> Self {
        let v = [0, 1, 2, 3]
            .into_iter()
            .map(|x| {
                if i == x {
                    "black".to_string()
                } else {
                    "grey".to_string()
                }
            })
            .collect();
        Page {
            title: s,
            tcolor: v,
        }
    }
}

#[derive(Serialize, Clone)]
struct Navigator {
    pub has_prev: bool,
    pub has_next: bool,
    pub prev: String,
    pub next: String,
}

impl Navigator {
    fn new(total: usize, now: usize) -> Self {
        Navigator {
            has_prev: now != 1,
            has_next: now != total,
            prev: (now - 1).to_string() + ".html",
            next: (now + 1).to_string() + ".html",
        }
    }
}

#[derive(Serialize)]
struct Category {
    pub title: String,
    pub url: String,
}

impl Category {
    fn new(s: String) -> Self {
        Category {
            url: "category/".to_string() + &slugify(&s) + ".html",
            title: s,
        }
    }
}

fn init() -> Result<(), err::Error> {
    for folder in FOLDERS {
        create_dir_all(folder)?;
    }
    copy("css/main.css", "public/css/main.css")?;
    copy("css/typora.css", "public/css/typora.css")?;
    let names: Vec<_> = TERA.get_template_names().collect();
    info!("Parsed {} Templates: {:?}", names.len(), names);
    Ok(())
}

fn picture_replace(mut content: String) -> Result<String, err::Error> {
    let re = Regex::new(r"!\[([^\]]+)\]\(([^\)]+)\)").unwrap();
    let config = GlobalConfig::global();
    for cap in re.captures_iter(&content.clone()) {
        if !cap[2].starts_with(&config.pic_replace_prefix) {
            continue;
        };
        let p = Picture::from_dir(PathBuf::from(&cap[2]))?;
        let path = p.register()?;
        let to =
            config.pic_cloud_prefix.clone() + "/" + &path.file_name().unwrap().to_str().unwrap();
        content = content.replace(&cap[2], &to);
    }
    Ok(content)
}

fn read_article_header(mut content: String) -> Result<Article, err::Error> {
    let re = Regex::new(r"---[\s\S]*?---").unwrap();
    let mut body;
    let yaml = match re.find(&content) {
        None => {
            warn!("YAML Front Matter not found. The content is {}", content);
            return Err(err::Error::new(
                err::Reason::ArticleRender,
                String::from("YAML not found"),
            ));
        }
        Some(c) => {
            body = content[c.end()..].to_string();
            &content[c.start() + 3..c.end() - 3]
        }
    };

    let yaml: ArticleYaml = serde_yaml::from_str(yaml)?;

    Ok(Article {
        title: yaml.title.clone(),
        date: yaml.date,
        summary: yaml.summary.unwrap_or_default(),
        url: yaml.path.unwrap_or_else(|| slugify(&yaml.title)),
        category: yaml.category.unwrap_or(String::from("未分类")),
        headline: yaml.headline.unwrap_or(false),
        content: body,
    })
}

fn markdown_paser(mut a: Article) -> Result<Article, err::Error> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(&a.content, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);

    a.content = output;
    Ok(a)
}

fn process_articles() -> Result<Vec<Article>, err::Error> {
    let conn = GlobalConnPool::global().0.get().unwrap();
    let mut articles = get_articles(&conn)?
        .into_iter()
        .map(|a| -> Result<Article, err::Error> {
            info!("Handling Article {:?}", &a.path);
            let content = std::fs::read_to_string(a.path)?;
            let mut article = read_article_header(content)?;
            article.content = picture_replace(article.content)?;
            article = markdown_paser(article)?;
            article.url = String::from("/") + &a.deploy_folder + "/" + &article.url + ".html";
            Ok(article)
        })
        .filter_map(|result| {
            if let Err(e) = result {
                warn!("Error Occured: {}", e);
                return None;
            };
            return Some(result.unwrap());
        })
        .collect::<Vec<Article>>();
    articles.sort_by(|a, b| (&b.date).cmp(&a.date));
    Ok(articles)
}

lazy_static! {
    static ref TERA: Tera = Tera::new("template/**/*.html").unwrap();
}
//the main render function
fn gen_html(context: &Context, template: &str, dst: &str) -> Result<(), err::Error> {
    let path = PathBuf::from(dst);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix)?;

    let t = TERA.render(template, &context).unwrap();
    let mut f = std::fs::File::create(&dst)?;
    f.write_all(t.as_bytes())?;

    info!("{} rendered", dst);
    Ok(())
}

fn index(articles: Vec<Article>) -> Result<(), err::Error> {
    let mut context = Context::new();
    let config = GlobalConfig::global();
    context.insert("global", config);
    context.insert("need_nav", &true);
    context.insert("page", &Page::new(0, "文章 | ".to_string() + &config.title));
    let headlines: Vec<Article> = articles.into_iter().filter(|a| a.headline).collect();
    //context.insert("article_briefs", &headlines);

    let page = (headlines.len()) / 20 + 1;

    for i in 1..=page {
        context.insert(
            "article_briefs",
            &headlines[(i - 1) * 20..min((i) * 20, headlines.len())],
        );
        context.insert("nav", &Navigator::new(page, i));
        gen_html(
            &context,
            "category.html",
            &("public/index/".to_string() + &i.to_string() + ".html"),
        )?;
    }
    Ok(())
}

fn category(c: Category, a: Vec<Article>) -> Result<(), err::Error> {
    let mut context = Context::new();
    let config = GlobalConfig::global();
    context.insert("global", config);
    context.insert("need_nav", &false);
    context.insert("page", &Page::new(2, "分类 | ".to_string() + &config.title));

    let articles: Vec<Article> = a.into_iter().filter(|a| a.category == c.title).collect();

    context.insert("article_briefs", &articles);
    gen_html(&context, "category.html", &("public/".to_string() + &c.url))?;
    Ok(())
}

fn article_category(articles: Vec<Article>) -> Result<(), err::Error> {
    let mut context = Context::new();
    let config = GlobalConfig::global();
    context.insert("global", config);
    context.insert("need_nav", &false);
    context.insert("page", &Page::new(2, "分类 | ".to_string() + &config.title));

    let mut categories = articles
        .clone()
        .into_iter()
        .map(|x| x.category)
        .collect::<Vec<String>>();
    categories.dedup();
    categories.sort();
    let categories: Vec<Category> = categories.into_iter().map(|c| Category::new(c)).collect();
    context.insert("categories", &categories);

    gen_html(
        &context,
        "category-list.html",
        "public/article_category.html",
    )?;
    for c in categories {
        category(c, articles.clone())?;
    }
    Ok(())
}

fn gallery(mut pictures: Vec<PhotographyPictureBrief>) -> Result<(), err::Error> {
    let mut context = Context::new();
    let config = GlobalConfig::global();
    context.insert("global", config);
    context.insert("need_nav", &true);
    context.insert("page", &Page::new(1, "照片 | ".to_string() + &config.title));
    pictures = pictures.into_iter().filter(|p| p.selected).collect();

    let page = (pictures.len()) / 20 + 1;

    for i in 1..=page {
        context.insert(
            "pics",
            &pictures[(i - 1) * 20..min((i) * 20, pictures.len())],
        );
        context.insert("nav", &Navigator::new(page, i));
        gen_html(
            &context,
            "picture.html",
            &("public/gallery/".to_string() + &i.to_string() + ".html"),
        )?;
    }

    Ok(())
}

fn picture(pictures: Vec<PhotographyPictureBrief>) -> Result<(), err::Error> {
    let mut context = Context::new();
    let config = GlobalConfig::global();
    context.insert("global", config);
    context.insert("need_nav", &true);
    context.insert("page", &Page::new(3, "图库 | ".to_string() + &config.title));

    let page = (pictures.len()) / 20 + 1;

    for i in 1..=page {
        context.insert(
            "pics",
            &pictures[(i - 1) * 20..min((i) * 20, pictures.len())],
        );
        context.insert("nav", &Navigator::new(page, i));
        gen_html(
            &context,
            "picture.html",
            &("public/picture/".to_string() + &i.to_string() + ".html"),
        )?;
    }

    Ok(())
}

fn article(articles: Vec<Article>) -> Result<(), err::Error> {
    let mut context = Context::new();
    let config = GlobalConfig::global();
    context.insert("global", config);
    context.insert("need_nav", &false);
    for a in articles {
        context.insert(
            "page",
            &Page::new(4, a.title.to_string() + " | " + &config.title),
        );
        context.insert("article", &a);
        gen_html(&context, "article.html", &("public".to_string() + &a.url))?;
    }
    Ok(())
}

fn render() -> Result<(), err::Error> {
    info!("Rendering");
    let articles = process_articles()?;
    index(articles.clone())?;
    article_category(articles.clone())?;

    let conn = GlobalConnPool::global().0.get().unwrap();
    let mut pictures = get_photography_pictures(&conn)?;
    pictures.sort_by(|a, b| (&b).date.cmp(&a.date));
    let pictures: Vec<PhotographyPictureBrief> = pictures
        .into_iter()
        .map(|p| PhotographyPictureBrief::from(p))
        .collect();
    gallery(pictures.clone())?;
    picture(pictures.clone())?;

    article(articles)?;
    info!("Rendered");
    Ok(())
}

fn deploy() {
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

pub fn start() {
    init().expect("Error initializing publisher");
    let config = GlobalConfig::global();
    if !config.deploy_auto {
        return;
    }
    thread::spawn(|| loop {
        sleep(Duration::new(config.deploy_interval.unwrap(), 0));
        let need_publish = NeedPublish::global().get();
        if !need_publish {
            continue;
        };
        info!("Start Publishing");
        if let Err(e) = render() {
            warn!("Error rendering , {:?}", e);
            continue;
        }
        deploy();
        NeedPublish::global().set(false);
    });
}
