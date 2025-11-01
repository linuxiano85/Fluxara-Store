use anyhow::Result;
use fluxara_core::PackageManager;
use fluxara_provider_flatpak::FlatpakProvider;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "search" => {
            if args.len() < 3 {
                eprintln!("Usage: fluxara search <query>");
                return Ok(());
            }
            let query = &args[2];
            cmd_search(query).await?;
        }
        "install" => {
            if args.len() < 3 {
                eprintln!("Usage: fluxara install <package-id>");
                return Ok(());
            }
            let package_id = &args[2];
            cmd_install(package_id).await?;
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("Usage: fluxara remove <package-id>");
                return Ok(());
            }
            let package_id = &args[2];
            cmd_remove(package_id).await?;
        }
        "update" => {
            if args.len() >= 3 {
                let package_id = &args[2];
                cmd_update_package(package_id).await?;
            } else {
                cmd_update_all().await?;
            }
        }
        "list" => {
            cmd_list().await?;
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage();
        }
    }

    Ok(())
}

fn print_usage() {
    println!("Fluxara CLI - Universal Linux Package Manager");
    println!();
    println!("Usage:");
    println!("  fluxara search <query>      Search for packages");
    println!("  fluxara install <package>   Install a package");
    println!("  fluxara remove <package>    Remove a package");
    println!("  fluxara update [package]    Update package(s)");
    println!("  fluxara list                List installed packages");
}

async fn cmd_search(query: &str) -> Result<()> {
    println!("Searching for: {}", query);

    let provider = FlatpakProvider::new();
    let packages = provider.search(query).await?;

    if packages.is_empty() {
        println!("No packages found.");
    } else {
        println!("Found {} packages:", packages.len());
        for package in packages {
            println!(
                "  {} - {} ({})",
                package.id,
                package.name,
                package.version.unwrap_or_else(|| "unknown".to_string())
            );
            if let Some(desc) = package.description {
                println!("    {}", desc);
            }
        }
    }

    Ok(())
}

async fn cmd_install(package_id: &str) -> Result<()> {
    println!("Installing: {}", package_id);

    let provider = FlatpakProvider::new();
    provider.install(package_id).await?;

    println!("Successfully installed {}", package_id);
    Ok(())
}

async fn cmd_remove(package_id: &str) -> Result<()> {
    println!("Removing: {}", package_id);

    let provider = FlatpakProvider::new();
    provider.remove(package_id).await?;

    println!("Successfully removed {}", package_id);
    Ok(())
}

async fn cmd_update_package(package_id: &str) -> Result<()> {
    println!("Updating: {}", package_id);

    let provider = FlatpakProvider::new();
    provider.update(package_id).await?;

    println!("Successfully updated {}", package_id);
    Ok(())
}

async fn cmd_update_all() -> Result<()> {
    println!("Updating all Flatpak packages...");

    let provider = FlatpakProvider::new();
    let updates = provider.list_updates().await?;

    if updates.is_empty() {
        println!("All packages are up to date.");
    } else {
        println!("Found {} updates", updates.len());
        // In a real implementation, would update each package
    }

    Ok(())
}

async fn cmd_list() -> Result<()> {
    println!("Listing installed packages...");

    let provider = FlatpakProvider::new();
    let packages = provider.list_installed().await?;

    if packages.is_empty() {
        println!("No packages installed.");
    } else {
        println!("Installed packages:");
        for package in packages {
            println!(
                "  {} - {} ({})",
                package.id,
                package.name,
                package.version.unwrap_or_else(|| "unknown".to_string())
            );
        }
    }

    Ok(())
}
