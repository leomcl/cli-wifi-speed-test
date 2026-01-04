use {
    crate::{
        cli::Config,
        server::{Server, ServerList},
    },
    anyhow::{Result, bail},
    speedtest_rs::{speedtest, speedtest_config::SpeedTestConfig},
    std::io::{self, Write},
    tracing::{error, info, warn},
};

/// 1 Mega Bits = 1,000,000 bits
const MBPS_DIVISOR: f64 = 1_000_000.0;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TestDirection {
    Download,
    Upload,
    #[default]
    Both,
}

pub struct Test {
    pub server: Server,
    pub direction: TestDirection,
}

impl Test {
    /// Create a new Test instance
    #[must_use]
    pub const fn new(server: Server, direction: TestDirection) -> Self {
        Self { server, direction }
    }

    /// Run the test on the configured server.
    ///
    /// # Errors
    /// Returns an error if the speedtest configuration cannot be retrieved
    /// or if the network test fails.
    pub fn run(&self) -> Result<()> {
        let mut speed_config = speedtest::get_configuration()
            .map_err(|e| anyhow::anyhow!("Failed to retrieve speedtest configuration: {e:?}"))?;

        self.run_test(&mut speed_config)
    }

    /// Check if download test should be performed.
    const fn should_download(&self) -> bool {
        matches!(
            self.direction,
            TestDirection::Download | TestDirection::Both
        )
    }

    /// Check if upload test should be performed.
    const fn should_upload(&self) -> bool {
        matches!(self.direction, TestDirection::Upload | TestDirection::Both)
    }

    /// # Errors
    /// Will return `Err` if either the download or upload test fails.
    fn run_test(&self, config: &mut SpeedTestConfig) -> Result<()> {
        let server = self.server.to_speedtest_server();
        info!(
            "Testing connection on server: {} ({})",
            server.id, server.name
        );

        if self.should_download() {
            self.run_download_test(config)?;
        }

        if self.should_upload() {
            self.run_upload_test(config)?;
        }

        Ok(())
    }

    /// # Errors
    /// Will return `Err` if the download test fails.
    fn run_download_test(&self, config: &mut SpeedTestConfig) -> Result<()> {
        let server = self.server.to_speedtest_server();
        info!("Performing download speed test...");
        // todo seperate progress callback (added print for mean time will seprate in next pr)
        let measurement = speedtest::test_download_with_progress_and_config(
            &server,
            || {
                print!("#");
                if let Err(e) = io::stdout().flush() {
                    error!("Failed to flush stdout: {e}");
                }
            },
            config,
        )
        .map_err(|e| anyhow::anyhow!("Download speed test failed: {e:?}"))?;
        println!();
        let download_mbps = Self::calculate_mbps(measurement.bps_f64());
        info!("Download Speed: {download_mbps:.2} Mbps");
        Ok(())
    }

    /// # Errors
    /// Will return `Err` if the upload test fails.
    fn run_upload_test(&self, config: &SpeedTestConfig) -> Result<()> {
        let server = self.server.to_speedtest_server();
        info!("Performing upload speed test...");
        // todo seperate progress callback (added print for mean time will seprate in next pr)
        let measurement = speedtest::test_upload_with_progress_and_config(
            &server,
            || {
                print!("#");
                if let Err(e) = io::stdout().flush() {
                    error!("Failed to flush stdout: {e}");
                }
            },
            config,
        )
        .map_err(|e| anyhow::anyhow!("Upload speed test failed: {e:?}"))?;
        println!();
        let upload_mbps = Self::calculate_mbps(measurement.bps_f64());
        info!("Upload Speed: {upload_mbps:.2} Mbps");
        Ok(())
    }

    /// Calculate Mbps from bits per second.
    /// Assumes input is in BPS (bits per second).
    const fn calculate_mbps(bps: f64) -> f64 {
        bps / MBPS_DIVISOR
    }
}

impl Test {
    /// Execute the speed test based on the provided configuration.
    ///
    /// # Errors
    /// Will return `Err` if no servers are available or all tests fail.
    pub fn run_config(config: &Config) -> Result<()> {
        let servers = ServerList::select_server(config.server_id().cloned())?;

        if servers.is_empty() {
            error!("No servers available for testing");
            bail!("No servers available for testing");
        }

        for (index, server) in servers.iter().enumerate() {
            let test = Self::new(server.clone(), config.direction());
            match test.run() {
                Ok(()) => return Ok(()),
                Err(e) => {
                    error!("Error with server {}: {}", server.id, e);
                    if index < servers.len() - 1 {
                        warn!("Trying next server...");
                    } else {
                        let msg = "All attempts failed. \
                                                        Please check your \
                                                        connection.";
                        error!("{msg}");
                        bail!("{msg}");
                    }
                }
            }
        }
        Ok(())
    }
}
