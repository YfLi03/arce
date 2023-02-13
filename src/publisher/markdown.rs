use log::{info, warn};
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use slug::slugify;
use std::path::PathBuf;

use crate::{
    api::{
        articles::{Article, ArticleYaml},
        config::GlobalConfig,
        err,
        pictures::Picture,
        sync::GlobalConnPool,
    },
    model::articles::get_articles,
};

/// replace the local pictures in the md with online pic urls
fn picture_replace(mut content: String) -> Result<String, err::Error> {
    let re = Regex::new(r"!\[([^\]]+)\]\(([^\)]+)\)").unwrap();

    let config = GlobalConfig::global();

    for cap in re.captures_iter(&content.clone()) {
        if !cap[2].starts_with(&config.pic_replace_prefix) {
            continue;
        };

        // register and upload the picture
        let p = Picture::from_dir(PathBuf::from(&cap[2]))?;
        let path = p.register()?;
        let to =
            config.pic_cloud_prefix.clone() + "/" + &path.file_name().unwrap().to_str().unwrap();

        // replace the urls
        content = content.replace(&cap[2], &to);
    }

    Ok(content)
}

/// getting the yaml front matter
fn read_article_header(content: String) -> Result<Article, err::Error> {
    let re = Regex::new(r"---[\s\S]*?---").unwrap();
    let body;
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

    // mergeing the yaml settings with default ones
    Ok(Article {
        title: yaml.title.clone(),
        date: yaml.date,
        summary: yaml.summary.unwrap_or_default(),
        url: yaml.path.unwrap_or_else(|| slugify(&yaml.title)),
        category: yaml.category.unwrap_or(String::from("未分类")),
        headline: yaml.headline.unwrap_or(false),
        content: body,
        encrypt: yaml.password.is_some(),
        password: yaml.password.unwrap_or(String::new()),
        hint: yaml.hint.unwrap_or(String::new())
    })
}

/// parse the markdown to html
fn markdown_paser(mut a: Article) -> Result<Article, err::Error> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(&a.content, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);

    a.content = output;
    Ok(a)
}

/// render the article bodies, and do the necessary processes
pub fn process_articles() -> Result<Vec<Article>, err::Error> {
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
    info!("Handled {} articles in total",articles.len());
    Ok(articles)
}
