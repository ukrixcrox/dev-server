use actix_web::{App, get, HttpServer, middleware::Logger, HttpRequest, HttpResponse, web::{self, Data}, Error};
use actix_files::Files;
use actix_toolbox::ws;
use notify::{RecursiveMode, Watcher};
use tokio::sync::watch;
use crate::config::{parse_config};

mod config;


// WebSocket handshake and start `WebSocket` actor.
#[get("/ws")]
async fn websocket(req:HttpRequest, stream: web::Payload, notifier: Data<watch::Sender<()>>) -> Result<HttpResponse, Error>{
    let (sender, mut _receiver, response) = ws::start(&req, stream)?;

    let mut notifier = notifier.subscribe();
    
    tokio::spawn(async move{
        loop{
            notifier.changed().await.unwrap();
           
            log::error!("foo");
            if sender.send(ws::Message::Text(("reload").into())).await.is_err(){    
                break
            };
        }

    });
    Ok(response)
}

#[tokio::main] 
async fn main() -> std::io::Result<()>{

    let serverconfig_parsed = parse_config();

    let (send, _) = watch::channel(());
    let send = Data::new(send);
    let send2 = send.clone();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mut watcher = notify::recommended_watcher( move|res: Result<_, _>| {
        res.unwrap();
        send2.send_replace(());
    }).unwrap();

    watcher.watch(serverconfig_parsed.projectfolder_path.as_ref(), RecursiveMode::Recursive).unwrap();

    HttpServer::new(move || {
        App::new()
            .service(websocket)
            .service(Files::new("/", serverconfig_parsed.projectfolder_path.clone()).index_file("index.html"))
            .app_data(send.clone())
            .wrap(Logger::default())
    })
    .bind((serverconfig_parsed.ip_address, serverconfig_parsed.port))?
    .run()
    .await

}
