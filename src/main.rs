use std::sync::Arc;
use std::net::SocketAddr;
use dotenv::dotenv;
use std::env;
use futures::io::ErrorKind;

// use std::convert::Infallible;
use hyper::service::{make_service_fn, service_fn};
// use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::{Server};
// use futures::TryStreamExt as _;

// extern crate postgres;
// use tokio_postgres::{NoTls, Error};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

use log::{error, warn, info, debug, trace};

#[macro_use] mod helpers; // mod order is important!
// mod middleware;
pub(crate) mod controllers;
pub(crate) mod router;
pub(crate) mod middleware;
pub(crate) mod model;
pub(crate) mod services;
use services::logs::simple_logger;

#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;

use serde_json::value::{to_value, Value};
// use std::error::Error;
use tera::{Result as TeraResult, Tera};

use std::collections::HashMap;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("views/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                // ::std::process::exit(1);
                panic!("this is a terrible mistake!");
                // @todo verification check all templates. Probably init it earlier (dont know how) and return exit()?
            }
        };
        tera.autoescape_on(vec!["html", ".sql"]);
        tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}

pub fn do_nothing_filter(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let s = try_get_value!("do_nothing_filter", "value", String, value);
    Ok(to_value(&s).unwrap())
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

// async fn tokio_db_request() -> Result<(), Error> {
//     println!("here in 1");
//     // Connect to the database.
//     let (client, connection) =
//         tokio_postgres::connect("host=localhost user=postgres password=postgres1 port=5433 dbname=rust", NoTls).await?;
//
//     // The connection object performs the actual communication with the database,
//     // so spawn it off to run on its own.
//     tokio::spawn(async move {
//         if let Err(e) = connection.await {
//             eprintln!("connection error: {}", e);
//         }
//     });
//
//     // Now we can execute a simple statement that just returns its parameter.
//     let rows = client
//         .query("select id, name from client LIMIT 10", &[])
//         .await?;
//
//     for row in rows {
//         let id: i64 = row.get(0);
//         let name: &str = row.get(1);
//         // let data: Option<&[u8]> = row.get(2);
//
//         println!("found person: {} {}", id, name);
//     }
//
//     Ok(())
// }

async fn db_pg_pool_init() -> Result<Pool<Postgres>, sqlx::Error> {
    let db_url = match env::var("DATABASE_URL") {
        Ok(db_url) => {
            db_url
        },
        Err(_) => {
            return Err(sqlx::Error::Io(std::io::Error::new(ErrorKind::Other, "Cant read DATABASE from .env")))
        }
    };

    let pool = PgPoolOptions::new()
        .max_connections(5)
        // .connect("host=localhost user=postgres password=postgres port=5433 dbname=rust").await?;
        .connect(db_url.as_str()).await?;

    // let pool = match pool_result {
    //     Ok(pool) => {
    //         pool
    //     }
    //     Err(err) => {
    //         println!("Error while init db pool: {:?}", err);
    //     }
    // };

    // Make a simple query to return the given parameter
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;
    assert_eq!(row.0, 150);

    Ok(pool)
}

async fn rabbitmq_init() -> Result<amiquip::Connection, amiquip::Error> {
    let rabbitmq_url = match env::var("RABBITMQ_URL") {
        Ok(val) => {
            val
        },
        Err(_) => {
            return Err(amiquip::Error::IoErrorReadingSocket {source: std::io::Error::new(ErrorKind::Other, "Cant read RABBITMQ_URL from .env")})
        }
    };

    let mut connection = amiquip::Connection::insecure_open(&rabbitmq_url)?;

    Ok(connection)
}

fn logger_init() {
    use log::{LevelFilter};

    match log::set_logger(&simple_logger::SimpleLogger)
        .map(|()| log::set_max_level(LevelFilter::Info))
    {
        Ok(_) => {
            error!("Logger init error!");
            warn!("Logger init warn!");
            info!("Logger init info!");
            debug!("Logger init debug!");
            trace!("Logger init trace!");
        }
        Err(err) => {
            println!("Error while init logger: {}", err);
        }
    };
}

pub struct ServerContext {
    db_pool: Pool<Postgres>,
    // rabbitmq_channel: amiquip::Channel,
    // rabbitmq_channel: amiquip::Connection,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    logger_init();
    let pool: Pool<Postgres> = match db_pg_pool_init().await {
        Ok(pool) => {
            pool
        },
        Err(e) => {
            eprintln!("db error: {:?}", e);
            return Err(std::io::Error::new(ErrorKind::Other, "oh no!"))
        }
    };
    // let rabbitmq_channel = match rabbitmq_init().await {
    //     Ok(connection) => {
    //         connection
    //     },
    //     Err(e) => {
    //         eprintln!("rabbitmq error: {:?}", e);
    //         return Err(std::io::Error::new(ErrorKind::Other, "oh no!"))
    //     }
    // };

    let server_context = Arc::new(
        ServerContext {
            db_pool: pool,
            // rabbitmq_channel: rabbitmq_channel,
        }
    );

    // We'll bind to 127.0.0.1:7878
    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let service = make_service_fn(move |_| {
        let server_ctx = Arc::clone(&server_context);
        async move {
            // service_fn converts our function into a `Service`
            let resp = service_fn(move |_req| router::router_handler(_req, Arc::clone(&server_ctx)));
            Ok::<_, hyper::Error>(resp)
        }
    });

    // And construct the `Server` like normal...
    let server = Server::bind(&addr).serve(service);

    // And now add a graceful shutdown signal...
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    };

    // Not work. How close?
    // &server_context.rabbitmq_connection.close();

    Ok(())

    // // Run this server for... forever!
    // if let Err(e) = server.await {
    //     eprintln!("server error: {}", e);
    // }
}

//https://stackoverflow.com/questions/61541215/what-is-the-idiomatic-way-to-write-rust-microservice-with-shared-db-connections

// https://www.arewewebyet.org/
// Educational Rust live coding - Building a web app - Part 1
// https://www.youtube.com/watch?v=yNe9Xr35n4Q&list=PL8lUUBadSMNBNKMYJpUE830tBiN6bxVRw

// Practical Rust Web Development - API Rest
// https://dev.to/werner/practical-rust-web-development-api-rest-29g1
