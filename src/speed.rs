use speedtest_rs::speedtest;

use std::io::{self, Write};

pub fn list_servers() -> Result<(), String>  {
    println!("Listing available servers...");

    let config = speedtest::get_configuration()
        .map_err(|e| format!("Failed to get config: {:?}", e))?;

    let servers = speedtest::get_server_list_with_config(&config)
        .map_err(|e| format!("Failed to get server list: {:?}", e))?;

    let sorted_servers = servers.servers_sorted_by_distance(&config);

    println!("\nTop 10 Closest Servers:");
    println!("{:<10} {:<25} {:<15} {:<10}", "ID", "Sponsor", "Name", "Dist (km)");
    println!("{}", "-".repeat(65));

    for server in sorted_servers.iter().take(10) {
        println!(
            "{:<10} {:<25} {:<15} {:.2}",
            server.id,
            ellipsize(&server.sponsor, 24), 
            ellipsize(&server.name, 14),
            server.distance.unwrap_or(0.0)
        );
    }

    Ok(())
}

fn ellipsize(text: &str, max_len: usize) -> String {
    if text.len() > max_len {
        format!("{}...", &text[..max_len-3])
    } else {
        text.to_string()
    }
}

pub fn do_test(server_id: Option<String>, do_down: bool, do_up: bool) -> Result<(), String> {
    let mut config = speedtest::get_configuration()
        .map_err(|e| format!("Failed to get config: {:?}", e))?;

    let server = if let Some(id_str) = server_id {
        println!("Locating server with ID: {}", id_str);

        let id = id_str.parse::<u32>()
            .map_err(|_| "Server ID must be a valid number")?;

        let servers = speedtest::get_server_list_with_config(&config)
            .map_err(|e| format!("Failed to get server list: {:?}", e))?;

        servers.servers.iter().find(|s| s.id == id).cloned()
            .ok_or(format!("Server with ID {} not found", id))?

    } else {
        println!("Finding closest server");

        let servers = speedtest::get_server_list_with_config(&config)
            .map_err(|e| format!("Could not retrieve server list: {:?}", e))?;

        let sorted = servers.servers_sorted_by_distance(&config);

        sorted.first().cloned()
            .ok_or("No servers available".to_string())?
    };   

    println!("Testing against server: {} ({})", server.id, server.name);

    if do_down {
        println!("Performing download speed test...");

        let measurement = speedtest::test_download_with_progress_and_config(&server, || {
            print!(".");
            io::stdout().flush().unwrap();
        }, &mut config).map_err(|e| format!("Download test failed: {:?}", e))?;

        let download_mbps = measurement.bps_f64() / 1_000_000.0;

        println!("\nDownload Speed: {:.2} Mbps", download_mbps);

    }
    if do_up {
        println!("Performing upload speed test...");

        let measurement = speedtest::test_upload_with_progress_and_config(&server, || {
            print!(".");
            io::stdout().flush().unwrap();
        }, &mut config).map_err(|e| format!("Upload test failed: {:?}", e))?;

        let upload_mbps = measurement.bps_f64() / 1_000_000.0;

        println!("\nUpload Speed: {:.2} Mbps", upload_mbps);
    }
    Ok(())
}
