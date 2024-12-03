use katharsis::cmd;
use serde::Deserialize;
use tokio::fs;
use tokio_test::assert_ok;

#[derive(Deserialize)]
struct Rss {
    title: String,
    description: String,
    site_url: String,
    image: String,
    copyright: String,
    language: String,
    output: String,
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

#[tokio::test]
async fn init() {
    let temp_dir = tempfile::tempdir().expect("failed to create temporary folder");
    let path = temp_dir.path().join("katharsis.config.toml");

    assert_ok!(cmd::handle::init(&path).await);

    let content = assert_ok!(fs::read_to_string(path).await);
    let config: Config = assert_ok!(toml::from_str(&content));
    let Config { rss, article } = config;

    assert_eq!(rss.title, "Your Website Title");
    assert_eq!(rss.description, "Your Website Description");
    assert_eq!(rss.site_url, "https://example.com");
    assert_eq!(rss.image, "favicon.png");
    assert_eq!(rss.copyright, "© Year Your Name");
    assert_eq!(rss.language, "en");
    assert_eq!(rss.output, "rss.xml");
    assert_eq!(article.title, "h1");
    assert_eq!(article.description, "[data-description]");
    assert_eq!(article.input, "articles/*.html");
    assert_eq!(article.author.name, "Your Name");
    assert_eq!(article.author.email, "Your Email");
    assert_eq!(article.link, "articles");
    assert_eq!(article.content, "[data-content]");
    assert_eq!(article.date, "time");
    assert_eq!(article.image, "articles/**/image.png");
    assert!(article.sort);
}
