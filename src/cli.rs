use {crate::test, clap::Parser};

/// A CLI tool for testing wifi download and upload speeds.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// List available servers sorted by distance
    #[arg(short, long)]
    pub list: bool,

    /// Specify a specific server ID to use
    #[arg(short, long)]
    pub server: Option<String>,

    /// Perform a download speed test
    #[arg(short, long)]
    pub down: bool,

    /// Perform an upload speed test
    #[arg(short, long)]
    pub up: bool,
}

#[derive(Debug, Default)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct Config {
    list: bool,
    server: Option<String>,
    direction: test::TestDirection,
}

impl Config {
    /// Check if the config is set to list servers.
    #[must_use]
    pub const fn has_list(&self) -> bool {
        self.list
    }

    /// Get the server ID if specified.
    #[must_use]
    pub const fn server_id(&self) -> Option<&String> {
        self.server.as_ref()
    }

    /// Get the test direction.
    #[must_use]
    pub const fn direction(&self) -> test::TestDirection {
        self.direction
    }
}

/// Builder for `Config` from CLI arguments.
pub struct ConfigBuilder {
    /// Whether to list servers
    list: bool,
    /// Optional server ID to use
    server: Option<String>,
    /// Whether to run download test
    down: bool,
    /// Whether to run upload test
    up: bool,
}

impl ConfigBuilder {
    /// Create a new `ConfigBuilder` from parsed CLI arguments.
    #[must_use]
    pub fn from_args(args: CliArgs) -> Self {
        Self {
            list: args.list,
            server: args.server,
            down: args.down,
            up: args.up,
        }
    }

    /// Build the final `Config` from the builder.
    #[must_use]
    pub fn build(self) -> Config {
        let direction = match (self.down, self.up) {
            (true, false) => test::TestDirection::Download,
            (false, true) => test::TestDirection::Upload,
            (true, true) | (false, false) => test::TestDirection::Both,
        };
        Config {
            list: self.list,
            server: self.server,
            direction,
        }
    }
}
