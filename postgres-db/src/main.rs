use clap::{
    crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};
use postgres::{Client, Error, NoTls, Row};
use r2d2_postgres::PostgresConnectionManager;
use std::env;

fn create_table(client: &mut Client) -> Result<(), Error> {
    client.batch_execute("CREATE TABLE IF NOT EXISTS posts (
                    id SERIAL PRIMARY KEY,
                    title VARCHAR NOT NULL,
                    description VARCHAR NOT NULL
                  )")
          .map(drop)
}

fn create_post(client: &mut Client, title: &str, description: &str) -> Result<(), Error> {
    client.execute("INSERT INTO posts (title, description) VALUES ($1, $2)",
                 &[&title, &description])
        .map(drop)
}

fn list_posts(client: &mut Client) -> Result<(), Error> {
    for row in client.query("SELECT title, description FROM posts", &[])? {
        print_post(row);
    }
    Ok(())
}

fn find_post(client: &mut Client, id: i32) -> Result<(), Error> {
    let row = client.query_opt("SELECT title, description FROM posts WHERE id = $1", &[&id])?;
    match row {
        Some(row) => {
            print_post(row);
        }
        None => println!("No matching post"),
    }
    Ok(())
}

fn print_post(post: Row) {
    let title: String = post.get("title");
    let description: String = post.get("description");

    println!("Title: {}, description: {}", title, description);
}

const CMD_ADD: &str = "add";
const CMD_FIND: &str = "find";
const CMD_LIST: &str = "list";

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
    /*
        CREATING SINGLE CONNECTION USING POSTGRES CRATE
    
    let client = Client::connect(db_url.as_str(), NoTls)?;
    */

    /*
        CREATING CONNECTION POOL USING R2D2_POSTGRES CRATE

    */
    let manager = PostgresConnectionManager::new(db_url.as_str().parse().unwrap(), NoTls);
    let pool = r2d2::Pool::new(manager).unwrap();
    let mut client = pool.get().unwrap();

    create_table(&mut client)?;

    match matches.subcommand() {
        (CMD_FIND, Some(matches)) => {
            let id: i32 = matches.value_of("ID").unwrap().parse::<i32>().unwrap();
            find_post(&mut client, id)?;
        }
        (CMD_ADD, Some(matches)) => {
            let title = matches.value_of("TITLE").unwrap();
            let desc = matches.value_of("DESC").unwrap();
            create_post(&mut client, title, desc)?;
        }
        (CMD_LIST, _) => {
            list_posts(&mut client)?;
        }
        _ => {
            matches.usage(); 
        }
    }

    Ok(())
}