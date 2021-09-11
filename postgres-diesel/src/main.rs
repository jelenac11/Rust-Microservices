#[macro_use]
extern crate diesel;

use clap::{
    crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::pg::PgConnection;
use model::{NewPost, Post};
use std::env;
use failure::{Error, format_err};

const CMD_ADD: &str = "add";
const CMD_FIND: &str = "find";
const CMD_LIST: &str = "list";

type Connection = PooledConnection<ConnectionManager<PgConnection>>;
pub mod model;
pub mod repository;
pub mod schema;

fn create_post(conn: &Connection, title: &str, description: &str) -> Result<Post, Error> {
    let new_post = NewPost {
        title: &title,
        description: &description
    };
    let post = repository::create_post(new_post, conn);
    Ok(post.unwrap())
}

fn list_posts(conn: &Connection) -> Result<Vec<Post>, Error> {
    let posts = repository::get_all_posts(conn);
    Ok(posts)
}

fn find_post(conn: &Connection, id: i32) -> Result<Post, Error> {
    let post = repository::get_post(id, conn);

    if let Ok(post) = post {
        Ok(post)
    } else {
        Err(format_err!("Post not found"))
    }
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name(CMD_ADD).about("add post to the table")
                    .arg(Arg::with_name("TITLE")
                         .help("Sets the title of a post")
                         .required(true)
                         .index(1))
                    .arg(Arg::with_name("DESC")
                         .help("Sets the description of a post")
                         .required(true)
                         .index(2)))
        .subcommand(SubCommand::with_name(CMD_FIND).about("find post by id")
            .arg(Arg::with_name("ID")
                .help("Finds post by given id")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name(CMD_LIST).about("print list of posts"))
        .get_matches();

    let db_url = env::var("DATABASE_URL").expect("Can't get DB URL");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::new(manager)?;

    match matches.subcommand() {
        (CMD_FIND, Some(matches)) => {
            let id: i32 = matches.value_of("ID").unwrap().parse::<i32>().unwrap();
            let conn = pool.get()?;
            let post = find_post(&conn, id)?;
            println!("{:?}", post);
        }
        (CMD_ADD, Some(matches)) => {
            let title = matches.value_of("TITLE").unwrap();
            let desc = matches.value_of("DESC").unwrap();
            let conn = pool.get()?;
            let created = create_post(&conn, title, desc)?;
            println!("{:?}", created);
        }
        (CMD_LIST, _) => {
            let conn = pool.get()?;
            let posts = list_posts(&conn)?;
            for post in posts.iter() {
                println!("{:?}", post);
            }
        }
        _ => {
            matches.usage(); 
        }
    }

    Ok(())
}