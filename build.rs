extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate scraper;

use futures::future::Future;
use futures::stream::Stream;

use std::io::Write;

fn fetch<C: hyper::client::Connect>(client: &hyper::Client<C>, url: &str) -> Box<Future<Item=Vec<String>,Error=()>> {
    let url: hyper::Uri = url.parse().unwrap();
    Box::new(client.get(url.clone()).and_then(move |res| {
        if res.status() != hyper::StatusCode::Ok {
            panic!("Failed to load {}", url);
        }
        res.body().concat2()
    })
    .and_then(|body| {
        let dom = scraper::Html::parse_fragment(&String::from_utf8(body.to_vec()).unwrap());
        let selector = scraper::Selector::parse(".navbox-list a").unwrap();

        Ok(dom.select(&selector).map(|e| {
            e.inner_html()
        }).collect())
    }).map_err(|e| panic!("{:?}", e)))
}

fn main() {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();

    let client = hyper::Client::configure()
        .connector(hyper_tls::HttpsConnector::new(2, &handle).unwrap())
        .build(&handle);

    let task = fetch(&client, "https://zh.moegirl.org/Template:ACG%E7%BB%8F%E5%85%B8%E5%8F%B0%E8%AF%8D")
        .join3(
            fetch(&client, "https://zh.moegirl.org/Template:ACG%E5%9C%88%E7%94%A8%E8%AF%AD"),
            fetch(&client, "https://zh.moegirl.org/Template:%E5%A4%A9%E6%9C%9D%E7%BD%91%E7%BB%9C%E6%B5%81%E8%A1%8C%E8%AF%AD%E5%8F%A5"))
        .and_then(|(a, b, c)| {
            let result = a.into_iter()
                .chain(b.into_iter())
                .chain(c.into_iter());
            let content = result.map(|x| format!("r#\"{}\"#", x))
                .collect::<Vec<_>>()
                .join(",\n        ");
            let content = format!("pub fn get_memes() -> Vec<&'static str> {{
    vec![
        {}
    ]
}}", content);
            let out_dir = std::env::var("OUT_DIR").unwrap();
            let path = std::path::Path::new(&out_dir).join("memes.rs");
            let mut file = std::fs::File::create(&path).unwrap();
            file.write_all(content.as_bytes()).unwrap();
            Ok(())
        });

    core.run(task).unwrap();
}
