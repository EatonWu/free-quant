use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use std::collections::vec_deque;
use fq_data_broker::{DataBroker, HashedBarSize};
use env_logger;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, Error};
use actix_web::cookie::time;
use time::macros::datetime;


struct RateLimiter {
    map: Mutex<HashMap<String, VecDeque<Instant>>>
}

#[get("/")]
async fn hello(req: HttpRequest, rate_limiter: web::Data<RateLimiter>) -> impl Responder {
    if let Some(val) = req.peer_addr() {
        // println!("IP: {:?}", val);
        if validate_rate_limit(rate_limiter.clone(), &val.to_string()) {
            HttpResponse::Ok().body("Hello world!")
        }
        else {
            HttpResponse::Ok().body("Too many requests")
        }
    }
    else {
        HttpResponse::Ok().body("No IP")
    }
    // get ip
}

fn validate_rate_limit(rate_limiter: web::Data<RateLimiter>, ip: &str) -> bool{
    match rate_limiter.map.lock() {
        Ok(mut map) => {
            let now = Instant::now();
            let deque = map.entry(ip.to_string()).or_insert(VecDeque::new());
            deque.push_back(now);
            while let Some(time) = deque.front() {
                if now.duration_since(*time) > Duration::from_secs(10) {
                    deque.pop_front();
                }
                else {
                    break;
                }
            }
            if deque.len() > 5 {
                println!("Too many requests from IP: {:?}", &ip);
                return false;
            }
            return true;
        },
        Err(e) => {
            println!("Error: {:?}", e);
            return false;
        }
    }
}

#[get("/ticker/{ticker}")]
async fn get_ticker(req: HttpRequest, path: web::Path<String>,  data_broker: web::Data<Mutex<DataBroker>>, rate_limiter: web::Data<RateLimiter>,) -> impl Responder {
    return if let Some(val) = req.peer_addr() {
        return if validate_rate_limit(rate_limiter.clone(), &val.ip().to_string()) {
            let data_broker = data_broker.lock();
            match data_broker {
                Ok(mut b) => {
                    let data = b.retrieve_data(
                        path.to_string(),
                        HashedBarSize::Min15,
                        datetime!(2021-01-01 00:00:00 UTC),
                        datetime!(2021-12-31 23:59:59 UTC),
                    );
                    match data {
                        Ok(d) => {
                            return HttpResponse::Ok().json(d);
                        },
                        Err(e) => {
                            println!("Error: {:?}", &e);
                            return HttpResponse::InternalServerError().body("Failed to get data");
                        }
                    }
                },
                Err(e) => {
                    println!("{:?}", e);
                    return HttpResponse::InternalServerError().body("Failed to get data broker");
                }
            }
        }
        else {
            return HttpResponse::TooManyRequests().body("Too many requests");
        }
    }
    else {
        return HttpResponse::InternalServerError().body("No IP");
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "DEBUG");
    }
    env_logger::init();
    // let arc_data_broker = Arc::new(DataBroker::new(None).unwrap());
    let rate_limiter = web::Data::new(RateLimiter {
        map: Mutex::new(HashMap::new())
    });

    let data_broker = DataBroker::new(Some("../@data".to_string()));
    let data_broker = match data_broker {
        Ok(b) => b,
        Err(e) => {
            println!("Error: {:?}", e);
            return Ok(());
        }
    };

    let data_broker = web::Data::new(Mutex::new(data_broker));

    // We should pass in a cloned version of the rate_limiter to each route.
    HttpServer::new(move || App::new()
            .app_data(rate_limiter.clone())
            .app_data(data_broker.clone())
            .service(echo)
            .service(hello)
            .service(get_ticker)
        )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}