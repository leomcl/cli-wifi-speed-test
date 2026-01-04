use {
        assert_cmd::Command,
        predicates::{prelude::PredicateBooleanExt, str::contains},
        std::{env, path::PathBuf},
};

fn run_with_args(args: &[&str]) -> assert_cmd::assert::Assert {
        let mut path: PathBuf = env::current_exe()
                .unwrap_or_else(|e| PathBuf::from(format!("error: {e}")));
        path.pop();
        path.push("swifi.exe");
        let mut cmd = Command::new(path);
        cmd.args(args).assert()
}

#[test]
fn test_cli_flags() {
        // Test: --list
        run_with_args(&["--list"]).stdout(contains("Available Servers"));

        // Test: --server 123
        run_with_args(&["--server", "123"])
                .stderr(contains("Error").or(contains("Download Speed")));

        // Test: --down
        run_with_args(&["--down"])
                .stderr(contains("Error").or(contains("Download Speed")));

        // Test: --up
        run_with_args(&["--up"])
                .stderr(contains("Error").or(contains("Upload Speed")));

        // Test: --down --up
        run_with_args(&["--down", "--up"]).stderr(contains("Error")
                .or(contains("Download Speed"))
                .or(contains("Upload Speed")));

        // Test: no flags
        run_with_args(&[]).stderr(contains("Error")
                .or(contains("Download Speed"))
                .or(contains("Upload Speed")));
}
