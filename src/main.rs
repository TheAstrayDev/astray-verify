use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const CONFIG_FILE: &str = "astray.verify.json";
const FIXTURES_DIR: &str = "fixtures";
const MCP_VERSION: &str = "2025-06-18";

#[derive(Parser)]
#[command(
    name = "astray-verify",
    about = "Record and replay basic MCP server contracts"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create astray.verify.json and the fixtures directory.
    Init,
    /// Record the tools exposed by an MCP stdio server.
    Record {
        /// Fixture name, for example "filesystem".
        #[arg(long)]
        name: String,
        /// Program and arguments used to launch the stdio MCP server.
        #[arg(required = true, last = true)]
        server: Vec<String>,
    },
    /// Replay saved fixtures and report interface regressions.
    Test {
        /// Test only one named fixture.
        #[arg(long)]
        name: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectConfig {
    version: u8,
    fixtures_dir: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Fixture {
    version: u8,
    name: String,
    transport: String,
    server: Vec<String>,
    protocol_version: String,
    tools: Value,
}

struct McpProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: mpsc::Receiver<Result<String>>,
}

impl McpProcess {
    fn start(server: &[String]) -> Result<Self> {
        let (program, args) = server
            .split_first()
            .context("server command is required after --")?;
        let mut child = Command::new(program)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .with_context(|| format!("could not start MCP server: {program}"))?;
        let stdin = child.stdin.take().context("could not open server stdin")?;
        let stdout = child
            .stdout
            .take()
            .context("could not open server stdout")?;
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if tx.send(line.context("could not read MCP stdout")).is_err() {
                    break;
                }
            }
        });
        Ok(Self {
            child,
            stdin,
            stdout: rx,
        })
    }

    fn send(&mut self, message: &Value) -> Result<()> {
        let line = serde_json::to_string(message)?;
        writeln!(self.stdin, "{line}")?;
        self.stdin.flush()?;
        Ok(())
    }

    fn response(&self, expected_id: u64) -> Result<Value> {
        loop {
            let line = self
                .stdout
                .recv_timeout(Duration::from_secs(20))
                .context("MCP server did not respond within 20 seconds")??;
            let value: Value = serde_json::from_str(&line)
                .with_context(|| format!("MCP server wrote non-JSON data to stdout: {line}"))?;
            if value.get("id") == Some(&json!(expected_id)) {
                if let Some(error) = value.get("error") {
                    bail!("MCP server returned an error: {error}");
                }
                return Ok(value);
            }
        }
    }

    fn discover_tools(&mut self) -> Result<Value> {
        self.send(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": MCP_VERSION,
                "capabilities": {},
                "clientInfo": {"name": "astray-verify", "version": env!("CARGO_PKG_VERSION")}
            }
        }))?;
        self.response(1)?;
        self.send(&json!({"jsonrpc": "2.0", "method": "notifications/initialized"}))?;
        self.send(&json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}))?;
        let response = self.response(2)?;
        response
            .get("result")
            .and_then(|result| result.get("tools"))
            .cloned()
            .context("tools/list response did not contain result.tools")
    }
}

impl Drop for McpProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn project_root() -> Result<PathBuf> {
    std::env::current_dir().context("could not determine current directory")
}

fn fixture_path(root: &Path, name: &str) -> PathBuf {
    root.join(FIXTURES_DIR).join(format!("{name}.mcp.json"))
}

fn init(root: &Path) -> Result<()> {
    fs::create_dir_all(root.join(FIXTURES_DIR))?;
    let config = root.join(CONFIG_FILE);
    if !config.exists() {
        fs::write(
            &config,
            serde_json::to_string_pretty(&ProjectConfig {
                version: 1,
                fixtures_dir: FIXTURES_DIR.into(),
            })? + "\n",
        )?;
        println!("created {}", config.display());
    } else {
        println!("{} already exists", config.display());
    }
    Ok(())
}

fn record(root: &Path, name: String, server: Vec<String>) -> Result<()> {
    if name.contains('/') || name.contains('\\') || name.is_empty() {
        bail!("fixture name must be a simple file name");
    }
    init(root)?;
    let mut process = McpProcess::start(&server)?;
    let tools = process.discover_tools()?;
    let fixture = Fixture {
        version: 1,
        name: name.clone(),
        transport: "stdio".into(),
        server,
        protocol_version: MCP_VERSION.into(),
        tools,
    };
    let path = fixture_path(root, &name);
    fs::write(&path, serde_json::to_string_pretty(&fixture)? + "\n")?;
    println!("recorded {}", path.display());
    Ok(())
}

fn test_fixture(path: &Path) -> Result<()> {
    let fixture: Fixture = serde_json::from_str(
        &fs::read_to_string(path).with_context(|| format!("could not read {}", path.display()))?,
    )?;
    if fixture.transport != "stdio" {
        bail!(
            "{}: unsupported transport {}",
            fixture.name,
            fixture.transport
        );
    }
    let mut process = McpProcess::start(&fixture.server)?;
    let actual = process.discover_tools()?;
    if actual == fixture.tools {
        println!("PASS  {}", fixture.name);
        return Ok(());
    }
    println!("FAIL  {}", fixture.name);
    println!("  tools/list changed; record again only if this was intentional.");
    bail!("fixture mismatch: {}", path.display());
}

fn run_tests(root: &Path, only_name: Option<String>) -> Result<()> {
    let paths = if let Some(name) = only_name {
        vec![fixture_path(root, &name)]
    } else {
        let dir = root.join(FIXTURES_DIR);
        fs::read_dir(&dir)
            .with_context(|| format!("no fixtures directory at {}; run init first", dir.display()))?
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
            .collect()
    };
    if paths.is_empty() {
        bail!("no fixtures found; run record first");
    }
    let mut failures = 0;
    for path in paths {
        if let Err(error) = test_fixture(&path) {
            eprintln!("  {error:#}");
            failures += 1;
        }
    }
    if failures > 0 {
        bail!("{failures} fixture(s) failed");
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = project_root()?;
    match cli.command {
        Commands::Init => init(&root),
        Commands::Record { name, server } => record(&root, name, server),
        Commands::Test { name } => run_tests(&root, name),
    }
}
