use database::models::Motorcycle;
use diesel::prelude::*;

use self::database::establish_connection;

mod database;

fn main() {
    let pg_conn = &mut establish_connection();
    use self::database::schema::app::motorcycles::dsl::*;
    let results = motorcycles
        .filter(make.eq("Yamaha Motor"))
        .limit(5)
        .select(Motorcycle::as_select())
        .load(pg_conn)
        .expect("Error loading posts");
    println!("Motorcycles found: {} motorcycles", results.len());
}
