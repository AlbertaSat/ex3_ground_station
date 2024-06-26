use crate::obc_client::ObcClient;
use sea_orm::{Database, DatabaseConnection, Set, ActiveModelTrait, EntityTrait, QueryOrder};
use rocket::fs::{relative, FileServer};
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::State;
use tokio::sync::Mutex;
use once_cell::sync::Lazy;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use rocket::http::Status;
use entities::command;
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate rocket;

mod obc_client;
mod message;
mod entities;

static OBC_CLIENT: Lazy<Mutex<ObcClient>> = Lazy::new(|| Mutex::new(
    ObcClient::new("localhost".to_string(), 50000)
));

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct CommandInput<'r> {
    payload: &'r str,
    cmd: &'r str,
    data: &'r str,
    timestamp: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct CommandOutput {
    id: i32,
    payload: String,
    cmd: String,
    data: String,
    timestamp: String,
}

#[get("/api/cmd", format="json")]
async fn get_cmds(connection: &State<DatabaseConnection>) -> Json<Vec<CommandOutput>> {
    let commands = command::Entity::find()
        .order_by_asc(command::Column::Timestamp)
        .all(connection.inner())
        .await
        .expect("Could not fetch commands");

    let result: Vec<CommandOutput> = commands.into_iter().map(|cmd| CommandOutput {
        id: cmd.id,
        payload: cmd.payload,
        cmd: cmd.command,
        data: cmd.data,
        timestamp: cmd.timestamp,
    }).collect();

    Json(result)
}

#[post("/api/cmd", format = "json", data = "<input>")]
async fn post_cmd(connection: &State<DatabaseConnection>, input: Json<CommandInput<'_>>) {
    println!("Got a form! Payload: {}, Cmd: {}, Data: {}", input.payload, input.cmd, input.data);

    let new_command = command::ActiveModel {
        command: Set(input.cmd.to_string()),
        payload: Set(input.payload.to_string()),
        data: Set(input.data.to_string()),
        timestamp: Set(input.timestamp.to_string()),
        ..Default::default()
    };

    new_command.save(connection.inner()).await.expect("Could not save command");

    let mut client = OBC_CLIENT.lock().await;
    match client.send_cmd([input.payload, input.cmd, input.data]).await {
        Ok(rc) => println!("Client response: {}", rc),
        Err(e) => println!("Client send error: {}", e),
    };
}

#[options("/api/cmd")]
fn options_cors() -> Status {
    Status::Ok
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok(); 

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let connection = match Database::connect(db_url).await {
        Ok(connection) => connection,
        Err(e) => panic!("Error connecting to database: {}", e),
    };

    let mut client = OBC_CLIENT.lock().await;
    match client.connect().await {
        Ok(_) => println!("Connected to OBC"),
        Err(e) => println!("Connection error: {}", e),
    }

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_headers(AllowedHeaders::all())
        .allow_credentials(true)
        .to_cors().expect("Error creating CORS options");

    rocket::build()
        .manage(connection)
        .mount("/", routes![get_cmds, post_cmd, options_cors])
        .mount("/", FileServer::from(relative!("static")))
        .attach(cors)
}
