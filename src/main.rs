use std::sync::Mutex;

use actix_web::{web, App, HttpResponse, HttpServer, FromRequest};

#[derive(Clone, Debug)]
pub struct Configuration {
    pub number: u32,
    pub description: String,
}

#[derive(Debug)]
pub struct SomeData {
    counter: u64,
    my_data: Vec<u8>,
}

impl SomeData {
    fn new() -> Self {
        SomeData {
            counter: 0,
            my_data: vec![],
        }
    }
}

pub async fn _upload(config: web::Data<Configuration>, other_data: web::Data<Mutex<SomeData>>, base64_audio: String) -> HttpResponse {
    let mut data = other_data.lock().unwrap();

    // Load audio.bytes from String
    let _audio_bytes = match base64::decode(&base64_audio) {
        Ok(audio) => audio,
        Err(e) => {
            let error = format!("failed to decode audio.data: {}\n", e);
            eprint!("{}", &error);
            return HttpResponse::InternalServerError()
                .content_type("plain/text")
                .body(error)
        }
    };

    println!("{:?}", config);

    // do something with SomeData
    data.counter = 10;
    data.my_data.push(10);
    println!("{:?}", data);

    HttpResponse::Ok()
    .content_type("plain/text")
    .body("upload successful")
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = Configuration {
        number: 42,
        description: "example data".to_string(),
    };
    let some_data = web::Data::new(Mutex::new(SomeData::new()));

    HttpServer::new(move || {
        App::new()
            .data(config.clone())
            .app_data(some_data.clone())
            .service(
                web::resource("/upload").data(
                    String::configure(|cfg| {
                        // limit audio file size in bytes
                        cfg.limit(1024 * 1024)
                    })
                )
                .route(web::post().to(_upload)))
            })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
