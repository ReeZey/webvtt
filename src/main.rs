use std::fs;
use isolang::Language;
use dash_mpd::MPD;

#[tokio::main]
async fn main() {
    let cmd_line: Vec<String> = std::env::args().collect();
    if cmd_line.len() != 2 {
        panic!("put remote index.mpd as secound argument");
    }
    let link = cmd_line[1].clone();
	let link2 = link.clone();
    //let (link, _args) = index_mpd.split_once("?").unwrap();

    let (cdn_base_url, _) = link2.rsplit_once("/").unwrap();
    //println!("{:?}", cdn_base_url);

    let folder_name: String = cdn_base_url.chars().filter(|c| c.is_alphanumeric()).collect();
    match fs::create_dir(&folder_name) {
        Ok(_) => {},
        Err(err) => {
            println!("could not create folder:");
            println!("{:#?}", err);
            return;
        }
    };

    let body = reqwest::get(link)
        .await.unwrap()
        .text()
        .await.unwrap();

    let mpd: MPD = dash_mpd::parse(&body).unwrap();

    let periods = mpd.periods;
    for period in periods {
        let sub_url = match period.BaseURL.len() {
            0 => {
                ""
            }
            _ => {
                &period.BaseURL[0].base
            }
        };
        for adaption in period.adaptations {
            match adaption.contentType {
                Some(content_type) => if content_type != "text" {
                    continue;
                },
                None => continue,
            }

            let lang = adaption.lang;
            for representation in adaption.representations {
                match adaption.mimeType.clone() {
                    Some(meme) => {
                        if meme != "text/vtt" {
                            continue;
                        }
                    },
                    None => {
                        match representation.mimeType {
                            Some(meme) => {
                                if meme != "text/vtt" {
                                    continue;
                                }
                            },
                            None => continue,
                        }
                    },
                }

                for base_url in representation.BaseURL {
                    let base = base_url.clone().base;

                    let link = format!("{}/{}{}", cdn_base_url, sub_url, base);
                    let filename = match lang.clone() {
                        Some(lang) => {
                            format!("{}/{}.vtt", folder_name, Language::from_639_1(&lang).unwrap().to_name())
                        },
                        None => {
                            base
                        },
                    };
                    println!("{}", filename);

                    let body = reqwest::get(link)
                        .await.unwrap()
                        .text()
                        .await.unwrap();

                    fs::write(filename, body).unwrap();
                }
            }
        }
    }

    //let link: String = cmd_line[1].clone();
    //let (link, _args) = link.split_once("?").unwrap();

    //println!("{:?}", link);
}
