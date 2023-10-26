fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("Missing environment variable DATABASE_URL");

    let env = dotenvy_macro::dotenv!("ENV");

    dbg!(database_url, env);
}
