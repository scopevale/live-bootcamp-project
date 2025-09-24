fn main() {
    // Load .env so its vars are available at compile time
    let _ = dotenvy::from_filename(".env");
    if let Ok(url) = std::env::var("DATABASE_URL") {
        // Export to rustc env so sqlx macros can read it
        println!("cargo:rustc-env=DATABASE_URL={}", url);
    }
    // trigger recompilation when a new migration is added
    println!("cargo:rerun-if-changed=migrations");
}
