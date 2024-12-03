use chrono::DateTime;
use katharsis::arg;
use rss::{Enclosure, Guid, Image};
use std::path::PathBuf;
use tokio_test::assert_ok;

#[tokio::test]
async fn builder() {
    let channel =
        assert_ok!(arg::handle::builder(&PathBuf::from("tests/katharsis.config.toml")).await);

    assert_eq!(channel.title(), "The React Framework for the Web");
    assert_eq!(channel.description(), "Used by some of the world's largest companies, Next.js enables you to create high-quality web applications with the power of React components.");
    assert_eq!(channel.link(), "https://nextjs.org/");

    let mut image = Image::default();
    image.set_url("https://nextjs.org/favicon.png");
    image.set_title("The React Framework for the Web");
    image.set_link("https://nextjs.org/");
    assert_eq!(channel.image(), Some(&image));

    assert_eq!(channel.copyright(), Some("© 2024 Vercel Inc"));
    assert_eq!(channel.language(), Some("en"));

    assert_eq!(channel.items().len(), 2);
    let beta_item = &channel.items[0];
    let enclosure = Enclosure {
        url: String::from("https://nextjs.org/articles/beta/opengraph-image.png"),
        length: String::from("4875"),
        mime_type: String::from("image/png"),
    };
    assert_eq!(beta_item.enclosure, Some(enclosure));
    assert_eq!(
        beta_item.author,
        Some(String::from("nextjs@vercel.com (Vercel Inc)"))
    );
    assert_eq!(
        beta_item.link,
        Some(String::from("https://nextjs.org/articles/beta"))
    );
    let guid = Guid {
        value: String::from("https://nextjs.org/articles/beta"),
        permalink: true,
    };
    assert_eq!(beta_item.guid, Some(guid));
    assert_eq!(beta_item.title, Some(String::from("Beta Article Title")));
    assert_eq!(
        beta_item.description,
        Some(String::from("Beta article description."))
    );
    assert_eq!(beta_item.content, Some(String::from("\n            <p data-mdx-description=\"true\">Beta article description.</p>\n            <h2>Beta Article Subheading</h2>\n            <p>Beta article subheading content.</p>\n        ")));
    assert_eq!(
        beta_item.pub_date,
        Some(String::from("Thu, 24 Oct 2024 00:00:00 GMT"))
    );

    assert_eq!(
        channel.docs(),
        Some("https://validator.w3.org/feed/docs/rss2.html")
    );
    assert_eq!(
        channel.generator(),
        Some("https://github.com/kurosakishigure/katharsis")
    );
    if let Some(last_build_date) = channel.last_build_date() {
        assert!(DateTime::parse_from_rfc2822(last_build_date).is_ok());
    } else {
        panic!("lastBuildDate not found")
    }
}
