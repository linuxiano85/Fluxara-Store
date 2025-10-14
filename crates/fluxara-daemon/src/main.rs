use fluxara_daemon::UpdateDaemon;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let daemon = UpdateDaemon::new()?;
    daemon.run().await?;

    Ok(())
}
