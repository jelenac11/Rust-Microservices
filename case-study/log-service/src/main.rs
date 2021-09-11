#[macro_use]
extern crate serde_derive;
extern crate job_scheduler;
extern crate rev_lines;

use bson::{bson, doc};
use chrono::offset::Utc;
use job_scheduler::{Job, JobScheduler};
use mongodb::db::{Database, ThreadedDatabase};
use r2d2::Pool;
use r2d2_mongodb::{ConnectionOptionsBuilder, MongodbConnectionManager};
use rev_lines::RevLines;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::time;
use url::Url;

mod log;

fn main() {
    dotenv::dotenv().ok();

    let users = env::var("LOG_USERS").expect("LOG_USERS not found.");
    let posts = env::var("LOG_POSTS").expect("LOG_POSTS not found.");
    let search = env::var("LOG_SEARCH").expect("LOG_SEARCH not found.");
    let rating = env::var("LOG_RATING").expect("LOG_RATING not found.");

    let database = env::var("DATABASE_URL").expect("DATABASE_URL not found.");
    let url = Url::parse(&database).expect("Database connecting...");

    let opts = ConnectionOptionsBuilder::new()
        .with_host(url.host_str().unwrap_or("localhost"))
        .with_port(url.port().unwrap_or(27017))
        .with_db(&url.path()[1..])
        .build();

    let manager = MongodbConnectionManager::new(opts);

    let pool = Pool::builder().max_size(4).build(manager).unwrap();

    let mut scheduler = JobScheduler::new();
    let mut treshold_users = Utc::now().naive_utc();
    let mut treshold_search = Utc::now().naive_utc();
    let mut treshold_rating = Utc::now().naive_utc();
    let mut treshold_posts = Utc::now().naive_utc();

    scheduler.add(Job::new("1/2 * * * * *".parse().unwrap(), || {
        match read_logs_from_file(
            treshold_users,
            &users,
            &pool.get().expect("Database connection"),
        ) {
            Ok(new_treshold) => treshold_users = new_treshold,
            Err(error) => println!("{:?}", error),
        }
        match read_logs_from_file(
            treshold_search,
            &search,
            &pool.get().expect("Database connection"),
        ) {
            Ok(new_treshold) => treshold_search = new_treshold,
            Err(error) => println!("{:?}", error),
        }
        match read_logs_from_file(
            treshold_rating,
            &rating,
            &pool.get().expect("Database connection"),
        ) {
            Ok(new_treshold) => treshold_rating = new_treshold,
            Err(error) => println!("{:?}", error),
        }
        match read_logs_from_file(
            treshold_posts,
            &posts,
            &pool.get().expect("Database connection"),
        ) {
            Ok(new_treshold) => treshold_posts = new_treshold,
            Err(error) => println!("{:?}", error),
        }
    }));

    loop {
        scheduler.tick();
        std::thread::sleep(time::Duration::from_millis(500));
    }
}

fn read_logs_from_file(
    treshold: chrono::NaiveDateTime,
    path: &str,
    conn: &Database,
) -> Result<chrono::NaiveDateTime, std::io::Error> {
    let mut new_treshold = Utc::now().naive_utc();
    match File::open(path) {
        Ok(file) => {
            let rev_lines = RevLines::new(BufReader::new(file));
            match rev_lines {
                Ok(rev_lines) => {
                    let mut set_new_treshold = false;
                    for line in rev_lines {
                        let parts: Vec<&str> = line.split("/").collect();
                        let log = log::Log {
                            datetime: parts[0].to_string(),
                            level: parts[1].to_string(),
                            target: parts[2].to_string(),
                            message: parts[3].to_string(),
                        };
                        let log_date =
                            chrono::NaiveDateTime::parse_from_str(parts[0], "%Y-%m-%d %H:%M:%S")
                                .unwrap();
                        if !set_new_treshold {
                            set_new_treshold = true;
                            new_treshold = log_date;
                        }
                        if log_date > treshold {
                            save_log_to_db(log, &conn)
                        }
                    }

                    Ok(new_treshold)
                }
                Err(error) => Err(error),
            }
        }
        Err(error) => Err(error),
    }
}

fn save_log_to_db(new_log: log::Log, conn: &Database) {
    let doc = doc! {
        "datetime": new_log.datetime,
        "level": new_log.level,
        "target": new_log.target,
        "message": new_log.message
    };
    let coll = conn.collection("logs");
    coll.insert_one(doc, None)
        .map(drop)
        .expect("Inserting log to database...");
}
