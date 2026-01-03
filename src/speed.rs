use speedtest_rs::{speedtest::{self, SpeedTestServer}, speedtest_config::SpeedTestConfig};

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

    let servers = speedtest::get_server_list_with_config(&config)
        .map_err(|e| format!("Failed to get server list: {:?}", e))?;

    let targets: Vec<SpeedTestServer> = if let Some(id_str) = server_id {
        let id = id_str.parse::<u32>()
            .map_err(|_| "Server ID must be a valid number")?;

        vec![servers.servers.iter().find(|s| s.id == id).cloned()
            .ok_or(format!("Server with ID {} not found", id))?]

    } else {
        servers.servers_sorted_by_distance(&config)
            .into_iter()
            .take(3)
            .collect()
    };

    if targets.is_empty() {
        return Err("No servers available for testing".to_string());
    }

    for (index, server) in targets.iter().enumerate() {
        match test_connection_on_server(server, &mut config, do_down, do_up) {
            Ok(_) => return Ok(()),
            Err(e) => {
                eprintln!("Error with server {}: {}", server.id, e);

                if index < targets.len() - 1 {
                    println!("Trying next server...");
                } else {
                    return Err("All attempts failed. Please check your connection.".to_string());
                }

            }
        }
    }
    Ok(())
}

fn test_connection_on_server(
    server: &SpeedTestServer,
    config: &mut SpeedTestConfig,
    do_down: bool,
    do_up: bool,
) -> Result<(), String> {
    println!("Testing connection on server: {} ({})", server.id, server.name);

     if do_down {
        println!("Performing download speed test...");

        let measurement = speedtest::test_download_with_progress_and_config(&server, || {
            print!("#");
            io::stdout().flush().unwrap();
        }, config).map_err(|_| format!("Download test failed, try another servier"))?;

        let download_mbps = measurement.bps_f64() / 1_000_000.0;

        println!("\nDownload Speed: {:.2} Mbps", download_mbps);

    }
    if do_up {
        println!("Performing upload speed test...");

        let measurement = speedtest::test_upload_with_progress_and_config(&server, || {
            print!("#");
            io::stdout().flush().unwrap();
        }, config).map_err(|_| format!("Download test failed, try another servier"))?;

        let upload_mbps = measurement.bps_f64() / 1_000_000.0;

        println!("\nUpload Speed: {:.2} Mbps", upload_mbps);
    }
    Ok(())
}
