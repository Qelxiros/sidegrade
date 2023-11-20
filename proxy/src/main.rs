use actix_web::{get, web, App, HttpServer, Responder};
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use convert_case::{Case, Casing};

pub mod calendar;
use calendar::CalendarEvents;

#[get("/locations/{location}/event")]
async fn screen(location: web::Path<String>) -> impl Responder {
    println!("Get calendar events for {}", location);
    let upcoming_events = CalendarEvents::new().await;
    match upcoming_events {
        Ok(u) => {
            match u.get_next_at_location(location.to_string().to_case(Case::Title)) {
                Some(e) => {
                    return e.format_1602()
                },
                None => {
                    return "No upcoming events.".to_string()
                }
            }
        },
        Err(e) => {
            return format!("Failed to get calendar events: {}", e).to_string()
        },
    }
}

#[get("/reserve/<location>/")]
async fn reserve(location: web::Path<String>, start: web::Path<String>, end: web::Path<String>) -> impl Responder {
    let proposed_start = DateTime::parse_from_rfc3339(&start.to_string())
            .expect("Failed to parse timestamp")
            .with_timezone(&Utc);

    let proposed_end = DateTime::parse_from_rfc3339(&end.to_string())
            .expect("Failed to parse timestamp")
            .with_timezone(&Utc);

    let upcoming_events = CalendarEvents::new().await;
    match upcoming_events {
        Ok(u) => {
            if u.is_free_at_location(location.to_string(), proposed_start, proposed_end) {
                return "Is free!".to_string()
            }           
            return "Reserved at that time.".to_string()
        },
        _ => {
            return "Failed to get calendar events".to_string()
        },
    }
}

#[get("/{name}")]
async fn name(name: web::Path<String>) -> impl Responder {
    format!("FuckOff, {}!", &name)
}

#[get("/")]
async fn test() -> impl Responder {
    format!("FuckOff!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Launching fuckoff4-proxy");
    println!("Check dotenv");
    dotenv().ok();
    println!("Run webserver");
    HttpServer::new(
        || App::new()
            .service(screen)
            .service(name)
            .service(test)
    )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
