use crate::error::app::Errors;
use anyhow::Result;
use chrono::{DateTime, Local, NaiveDate};
use glob::{glob, GlobResult};
use rss::{Channel, ChannelBuilder, Enclosure, Guid, Image, Item};
use scraper::{Html, Selector};
use serde::Deserialize;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use tokio::{fs, fs::File};
use url::Url;

#[derive(Deserialize)]
struct Rss {
    title: String,
    description: String,
    site_url: Url,
    image: String,
    copyright: String,
    language: String,
    output: PathBuf,
}

#[derive(Deserialize)]
struct Author {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct Article {
    title: String,
    description: String,
    input: String,
    author: Author,
    link: String,
    content: String,
    date: String,
    image: String,
    sort: bool,
}

#[derive(Deserialize)]
struct Config {
    rss: Rss,
    article: Article,
}

async fn item_builder(entry: GlobResult, rss: &Rss, article: &Article) -> Result<Item, Errors> {
    let path = entry?;
    let mut item: Item = Item::default();
    let mut guid = Guid::default();
    let mut enclosure = Enclosure::default();

    println!(
        "- parse: {}",
        path.file_name()
            .expect("failed to extract the filename")
            .to_string_lossy()
    );

    let file_stem = path
        .file_stem()
        .expect("failed to extract the file stem")
        .to_string_lossy();
    let image_path = PathBuf::from(article.image.replace("**", &file_stem));
    let article_url = format!("{}{}/{}", rss.site_url, article.link, file_stem);

    guid.set_value(&article_url);
    if image_path.try_exists()? {
        enclosure.set_url(format!(
            "{}/{}",
            article_url,
            image_path
                .file_name()
                .expect("failed to extract the filename")
                .to_string_lossy()
        ));
        enclosure.set_length(fs::metadata(&image_path).await?.len().to_string());
        enclosure.set_mime_type(format!(
            "image/{}",
            image_path
                .extension()
                .expect("failed to extract the file extension")
                .to_string_lossy()
        ));

        item.set_enclosure(enclosure);
    } else {
        eprintln!("  - warn: image not find");
    }
    item.set_author(format!(
        "{} ({})",
        article.author.email, article.author.name
    ));
    item.set_link(article_url);
    item.set_guid(guid);

    let html = fs::read_to_string(&path).await?;
    let document = Html::parse_document(&html);
    let title_selector = Selector::parse(&article.title).expect("title selector parse error");
    let description_selector =
        Selector::parse(&article.description).expect("description selector parse error");
    let content_selector = Selector::parse(&article.content).expect("content selector parse error");
    let date_selector = Selector::parse(&article.date).expect("date selector parse error");
    if let Some(element) = document.select(&title_selector).next() {
        let title = element.text().collect::<Vec<&str>>().join(" ");

        item.set_title(title);
    } else {
        eprintln!("  - warn: title element not find");
    }
    if let Some(element) = document.select(&description_selector).next() {
        let description = element.text().collect::<Vec<&str>>().join(" ");

        item.set_description(description);
    } else {
        eprintln!("  - warn: description element not find");
    }
    if let Some(element) = document.select(&content_selector).next() {
        let content = element.inner_html();

        item.set_content(content);
    } else {
        eprintln!("  - warn: content element not find");
    }
    if let Some(element) = document.select(&date_selector).next() {
        let datetime = element
            .value()
            .attr("datetime")
            .expect("datetime attribute not found");
        let naive_date = NaiveDate::parse_from_str(datetime, "%Y-%m-%d")?;
        let formatted_date = naive_date.format("%a, %d %b %Y 00:00:00 GMT").to_string();

        item.set_pub_date(formatted_date);
    } else {
        eprintln!("  - warn: date element not find");
    }

    Ok(item)
}

fn channel_builder(rss: Rss, items: Vec<Item>) -> Channel {
    let mut image = Image::default();
    image.set_url(format!("{}{}", rss.site_url, rss.image));
    image.set_title(&rss.title);
    image.set_link(rss.site_url.as_ref());

    ChannelBuilder::default()
        .title(rss.title)
        .description(rss.description)
        .link(rss.site_url)
        .image(image)
        .copyright(rss.copyright)
        .language(rss.language)
        .items(items)
        .docs(String::from("https://validator.w3.org/feed/docs/rss2.html"))
        .generator(String::from("https://github.com/kurosakishigure/katharsis"))
        .last_build_date(Local::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string())
        .build()
}

/// Build the RSS feed based on the fields in the config file.
///
/// # Examples
///
/// ```no_run
/// use anyhow::Result;
/// use katharsis::arg;
/// use std::path::PathBuf;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let path = PathBuf::from("katharsis.config.toml");
///
///     arg::handle::builder(&path).await?;
///
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// - When the file’s existence cannot be verified.
/// - When the file does not exist.
/// - When the file cannot be read.
/// - When the TOML file cannot be deserialized.
/// - When the pattern cannot be parsed.
/// - When the iteration path cannot be read.
/// - When the file metadata cannot be read.
/// - When the date cannot be parsed.
/// - When the file cannot be created.
/// - When the file cannot be written to.
pub async fn builder(config_path: &PathBuf) -> Result<Channel, Errors> {
    if config_path.try_exists()? {
        let config: Config = toml::from_str(&fs::read_to_string(config_path).await?)?;
        let Config { rss, article } = config;
        let mut items: Vec<Item> = vec![];

        for entry in glob(&article.input)? {
            items.push(item_builder(entry, &rss, &article).await?);
        }

        if article.sort {
            items.sort_by(|a, b| {
                let a_date = a
                    .pub_date
                    .as_ref()
                    .and_then(|d| DateTime::parse_from_rfc2822(d).ok());
                let b_date = b
                    .pub_date
                    .as_ref()
                    .and_then(|d| DateTime::parse_from_rfc2822(d).ok());

                b_date.cmp(&a_date)
            });
        }

        println!("- create: {}", rss.output.to_string_lossy());
        let mut rss_file = {
            if rss.output == PathBuf::from("[temp]") {
                File::from_std(tempfile::tempfile()?)
            } else {
                File::create(&rss.output).await?
            }
        };
        let channel = channel_builder(rss, items);
        rss_file.write_all(channel.to_string().as_ref()).await?;

        Ok(channel)
    } else {
        eprintln!("- {}", config_path.to_string_lossy());

        Err(Errors::FileNotExistError)
    }
}
