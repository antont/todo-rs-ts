use refinery::embed_migrations;
use tokio_postgres::NoTls;

embed_migrations!("migrations");

#[tokio::main]
async fn main() {
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (mut client, connection) =
        tokio_postgres::connect(&database_url, NoTls)
            .await
            .expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {e}");
        }
    });

    println!("Running migrations...");
    migrations::runner()
        .run_async(&mut client)
        .await
        .expect("Failed to run migrations");
    println!("Migrations complete.");
}
