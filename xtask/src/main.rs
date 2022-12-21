/*
    Appellation: xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Welcome to xtask...");

    let handle = std::thread::spawn(move || {
        xtask::cli::handle().join().unwrap();
    });
    handle.join().ok().unwrap();

    Ok(())
}
