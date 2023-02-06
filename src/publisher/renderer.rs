use lazy_static::lazy_static;
use log::{debug, info};
use serde::Serialize;
use sitemap::structs::UrlEntry;
use sitemap::{structs::UrlEntryBuilder, writer::SiteMapWriter};
use slug::slugify;
use std::{cell::RefCell, cmp::min, io::Write, path::PathBuf};
use tera::{Context, Tera};

use crate::api::{articles::Article, config::GlobalConfig, err, pictures::PhotographyPictureBrief};

/// Page Info context for Tera
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

/// Navigator info context for tera
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

/// category info context for tera
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

lazy_static! {
    pub static ref TERA: Tera = Tera::new("template/**/*.html").unwrap();
}

thread_local! {static URL_ENTRY: RefCell<Vec<UrlEntryBuilder>> = RefCell::new(vec![])}

/// generate html(s)
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

/// generate index page
fn index(articles: Vec<Article>) -> Result<(), err::Error> {
    let mut context = Context::new();
    let config = GlobalConfig::global();
    context.insert("global", config);
    context.insert("need_nav", &true);
    context.insert("page", &Page::new(0, "文章 | ".to_string() + &config.title));

    let headlines: Vec<Article> = articles.into_iter().filter(|a| a.headline).collect();
    let page = (headlines.len()) / 20 + 1;

    URL_ENTRY.with(|v| {
        // Assuming index is set to /index/1.html
        (*v.borrow_mut()).push(UrlEntry::builder().loc(config.url.clone()));
    });

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

        URL_ENTRY.with(|v| {
            // Assuming index is set to /index/1.html
            (*v.borrow_mut()).push(
                UrlEntry::builder()
                    .loc(config.url.clone() + &"/index/".to_string() + &i.to_string() + ".html"),
            );
        });
    }
    Ok(())
}

/// generate article (detail) page
fn article(articles: Vec<Article>, c: Category) -> Result<(), err::Error> {
    let mut context = Context::new();
    let config = GlobalConfig::global();
    context.insert("global", config);
    context.insert("need_nav", &false);
    context.insert("category", &c);

    for a in articles {
        context.insert(
            "page",
            &Page::new(4, a.title.to_string() + " | " + &config.title),
        );
        context.insert("article", &a);

        gen_html(&context, "article.html", &("public".to_string() + &a.url))?;

        URL_ENTRY.with(|v| {
            (*v.borrow_mut()).push(
                UrlEntry::builder().loc(config.url.clone() + "/" + a.url.clone().trim_matches('/')),
            );
        });
    }
    Ok(())
}

/// generate category (detail) page, also the articles
fn category(c: Category, a: Vec<Article>) -> Result<(), err::Error> {
    let mut context = Context::new();
    let config = GlobalConfig::global();
    context.insert("global", config);
    context.insert("need_nav", &false);
    context.insert("page", &Page::new(2, "分类 | ".to_string() + &config.title));

    let articles: Vec<Article> = a.into_iter().filter(|a| a.category == c.title).collect();

    context.insert("article_briefs", &articles);
    gen_html(&context, "category.html", &("public/".to_string() + &c.url))?;

    URL_ENTRY.with(|v| {
        (*v.borrow_mut()).push(UrlEntry::builder().loc(config.url.clone() + "/" + &c.url));
    });

    article(articles, c)?;

    Ok(())
}

/// generate pages for category lists, also the categories and articles
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
    categories.sort();
    categories.dedup();
    debug!("Categories {:?}", categories);
    let categories: Vec<Category> = categories.into_iter().map(|c| Category::new(c)).collect();
    context.insert("categories", &categories);

    gen_html(
        &context,
        "category-list.html",
        "public/article_category.html",
    )?;

    URL_ENTRY.with(|v| {
        (*v.borrow_mut())
            .push(UrlEntry::builder().loc(config.url.clone() + "/article_category.html"));
    });

    for c in categories {
        category(c, articles.clone())?;
    }

    Ok(())
}

/// generate the gallery pages
fn gallery(mut pictures: Vec<PhotographyPictureBrief>) -> Result<(), err::Error> {
    let mut context = Context::new();
    let config = GlobalConfig::global();
    context.insert("global", config);
    context.insert("need_nav", &true);
    context.insert("page", &Page::new(1, "照片 | ".to_string() + &config.title));
    pictures = pictures.into_iter().filter(|p| p.selected).collect();

    let page = (pictures.len()-1) / 20 + 1;

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

        URL_ENTRY.with(|v| {
            (*v.borrow_mut()).push(
                UrlEntry::builder()
                    .loc(config.url.clone() + &"/gallery/".to_string() + &i.to_string() + ".html"),
            );
        });
    }

    Ok(())
}

/// generate the picture pages
fn picture(pictures: Vec<PhotographyPictureBrief>) -> Result<(), err::Error> {
    let mut context = Context::new();
    let config = GlobalConfig::global();
    context.insert("global", config);
    context.insert("need_nav", &true);
    context.insert("page", &Page::new(3, "图库 | ".to_string() + &config.title));

    let page = (pictures.len()-1) / 20 + 1;

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

        URL_ENTRY.with(|v| {
            (*v.borrow_mut()).push(
                UrlEntry::builder()
                    .loc(config.url.clone() + &"/picture/".to_string() + &i.to_string() + ".html"),
            );
        });
    }

    Ok(())
}

fn sitemap() -> Result<(), err::Error> {
    let config = GlobalConfig::global();
    if let None = config.robot {
        return Ok(());
    };

    let mut robot = std::fs::File::create("public/robots.txt")?;
    robot.write(
        (String::from("Sitemap: ") + &config.url + "/" + config.robot.as_ref().unwrap()).as_bytes(),
    )?;

    let mut site_map =
        std::fs::File::create(PathBuf::from("public").join(config.robot.as_ref().unwrap()))?;
    let writer = SiteMapWriter::new(&mut site_map);
    let mut writer = writer.start_urlset()?;
    URL_ENTRY.with(|v| {
        for i in &*v.borrow() {
            writer
                .url(i.clone().build().unwrap())
                .expect("Unable To Write Url");
        }
    });
    writer.end()?;

    Ok(())
}

/// render all pages
/// sitemap is generated at the same time
pub fn render(
    articles: Vec<Article>,
    pictures: Vec<PhotographyPictureBrief>,
) -> Result<(), err::Error> {
    info!("Rendering");

    URL_ENTRY.with(|v| *v.borrow_mut() = vec![]);
    
    index(articles.clone())?;
    article_category(articles.clone())?;
    gallery(pictures.clone())?;
    picture(pictures.clone())?;

    sitemap()?;

    info!("Rendered");
    Ok(())
}
