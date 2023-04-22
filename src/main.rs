use std::{fs, path::Path};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = vec![];

    // 向多个网站发送请求
    let sites = vec![
        "https://ssr1.scrape.center/page/1",
        "https://ssr1.scrape.center/page/2",
        "https://ssr1.scrape.center/page/3",
        "https://ssr1.scrape.center/page/4",
        "https://ssr1.scrape.center/page/5",
        "https://ssr1.scrape.center/page/6",
        "https://ssr1.scrape.center/page/7",
        "https://ssr1.scrape.center/page/8",
        "https://ssr1.scrape.center/page/9",
        "https://ssr1.scrape.center/page/10",
    ];

    for site in sites {
        let task = tokio::spawn(async move { download(site) });
        tasks.push(task);
    }
    create_file_dir("dl");
    for task in tasks {
        task.await?.await;
    }

    Ok(())
}

async fn download(url: &str) {
    let client = reqwest::Client::new();
    let text = client.get(url).send().await.unwrap().text().await.unwrap();

    let suffix = url.split('/').last().unwrap();
    let mut filename = format!("{}.html", suffix);
    // println!("{}", filename);

    filename = format!("dl/{filename}");
    fs::write(filename, &text).unwrap();
}

fn create_file_dir(filepath: &str) {
    let path = Path::new(filepath);
    if !path.exists() {
        fs::create_dir(path).unwrap();
    }
}
