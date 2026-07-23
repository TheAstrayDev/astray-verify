use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::fs;
use std::io::{BufRead, BufReader, IsTerminal, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const CONFIG_FILE: &str = "astray.verify.json";
const FIXTURES_DIR: &str = "fixtures";
const MCP_VERSION: &str = "2025-06-18";
const DEFAULT_TIMEOUT_MS: u64 = 20_000;

#[derive(Clone, Copy, Debug, ValueEnum)]
enum ColorMode {
    Auto,
    Always,
    Never,
}

#[derive(Parser)]
#[command(
    name = "astray-verify",
    version,
    about = "Record and replay MCP server contracts",
    long_about = "Astray Verify snapshots an MCP server's tools/list contract and catches breaking changes before your AI clients do.",
    after_help = "Examples:\n  astray-verify init\n  astray-verify record --name filesystem -- npx -y @modelcontextprotocol/server-filesystem ./demo\n  astray-verify test"
)]
struct Cli {
    #[arg(long, global = true, value_enum, default_value_t = ColorMode::Auto)]
    color: ColorMode,
    #[arg(long, global = true)]
    json: bool,
    #[arg(long, global = true, num_args = 0..=1, default_missing_value = "logs/astray-verify.jsonl")]
    log: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create astray.verify.json and the fixtures directory.
    Init,
    /// Record the tools, resources, and prompts exposed by an MCP stdio server.
    Record {
        #[arg(long)]
        name: String,
        #[arg(long, value_delimiter = ',', default_value = "tools")]
        checks: Vec<CheckKind>,
        #[arg(long)]
        timeout_ms: Option<u64>,
        #[arg(required = true, last = true)]
        server: Vec<String>,
    },
    /// Replay saved fixtures and report interface regressions.
    Test {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        timeout_ms: Option<u64>,
    },
    /// Inspect an MCP server and identify its highest-risk protocol or contract area.
    Audit {
        #[arg(long)]
        name: Option<String>,
        #[arg(long, default_value_t = DEFAULT_TIMEOUT_MS)]
        timeout_ms: u64,
        #[arg(last = true)]
        server: Vec<String>,
    },
    /// Display the resolved configuration and per-fixture settings.
    Config,
    /// Diagnose the project: configuration, fixtures, JSONL log, optional drift.
    Doctor {
        #[arg(long)]
        with_test: bool,
    },
    /// Re-run `test` whenever a fixture or configuration file changes.
    Watch {
        #[arg(long)]
        initial: bool,
    },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum CheckKind {
    Tools,
    Resources,
    Prompts,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectConfig {
    version: u8,
    fixtures_dir: String,
    #[serde(default)]
    defaults: CheckSettings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CheckSettings {
    #[serde(default = "default_checks")]
    checks: Vec<CheckKind>,
    #[serde(default = "default_timeout_ms")]
    timeout_ms: u64,
}

fn default_checks() -> Vec<CheckKind> {
    vec![CheckKind::Tools]
}
fn default_timeout_ms() -> u64 {
    DEFAULT_TIMEOUT_MS
}

impl Default for CheckSettings {
    fn default() -> Self {
        Self {
            checks: default_checks(),
            timeout_ms: default_timeout_ms(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Fixture {
    version: u8,
    name: String,
    transport: String,
    server: Vec<String>,
    protocol_version: String,
    tools: Value,
    #[serde(default)]
    resources: Value,
    #[serde(default)]
    prompts: Value,
    #[serde(default)]
    settings: CheckSettings,
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

    fn response(&self, expected_id: u64, timeout: Duration) -> Result<Value> {
        let deadline = Instant::now() + timeout;
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                bail!(
                    "MCP server did not respond to request {expected_id} within {} ms",
                    timeout.as_millis()
                );
            }
            let line = self
                .stdout
                .recv_timeout(remaining)
                .map_err(|error| match error {
                    mpsc::RecvTimeoutError::Timeout => anyhow::anyhow!(
                        "MCP server did not respond to request {expected_id} within {} ms",
                        timeout.as_millis()
                    ),
                    mpsc::RecvTimeoutError::Disconnected => anyhow::anyhow!(
                        "MCP server closed stdout before responding to request {expected_id}"
                    ),
                })??;
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

    fn discover_contract(&mut self, settings: &CheckSettings) -> Result<ServerContract> {
        let timeout = Duration::from_millis(settings.timeout_ms);
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
        let initialize = self.response(1, timeout)?;
        self.send(&json!({"jsonrpc": "2.0", "method": "notifications/initialized"}))?;
        let capabilities = initialize
            .get("result")
            .and_then(|value| value.get("capabilities"))
            .cloned()
            .unwrap_or_else(|| json!({}));
        let mut next_id = 2u64;
        let mut list = |method: &str, key: &str, capability: &str| -> Result<Value> {
            if !capability.is_empty() && capabilities.get(capability).is_none() {
                return Ok(Value::Array(vec![]));
            }
            let id = next_id;
            next_id += 1;
            self.send(&json!({"jsonrpc": "2.0", "id": id, "method": method, "params": {}}))?;
            self.response(id, timeout)?
                .get("result")
                .and_then(|result| result.get(key))
                .cloned()
                .with_context(|| format!("{method} response did not contain result.{key}"))
                .and_then(normalize_named_items)
        };
        let tools = list("tools/list", "tools", "")?;
        let resources = list("resources/list", "resources", "resources")?;
        let prompts = list("prompts/list", "prompts", "prompts")?;
        Ok(ServerContract {
            protocol_version: initialize
                .get("result")
                .and_then(|value| value.get("protocolVersion"))
                .and_then(Value::as_str)
                .map(str::to_owned),
            capabilities,
            tools,
            resources,
            prompts,
        })
    }
}

#[derive(Debug)]
struct ServerContract {
    protocol_version: Option<String>,
    capabilities: Value,
    tools: Value,
    resources: Value,
    prompts: Value,
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

fn safe_relative_path(path: &str, field: &str) -> Result<PathBuf> {
    let path = Path::new(path);
    if path.as_os_str().is_empty()
        || path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        bail!("{field} must be a non-empty relative path without . or .. components");
    }
    Ok(path.to_path_buf())
}

fn validate_fixture_name(name: &str) -> Result<()> {
    if name.is_empty()
        || name == "."
        || name == ".."
        || name.contains(['/', '\\'])
        || Path::new(name).file_name().and_then(|value| value.to_str()) != Some(name)
    {
        bail!("fixture name must be a simple, non-empty file name");
    }
    Ok(())
}

fn default_config() -> ProjectConfig {
    ProjectConfig {
        version: 2,
        fixtures_dir: FIXTURES_DIR.into(),
        defaults: CheckSettings::default(),
    }
}

fn load_config(root: &Path) -> Result<ProjectConfig> {
    let path = root.join(CONFIG_FILE);
    let config: ProjectConfig = serde_json::from_str(
        &fs::read_to_string(&path)
            .with_context(|| format!("could not read {}; run init first", path.display()))?,
    )
    .with_context(|| format!("could not parse {}", path.display()))?;
    if !(1..=2).contains(&config.version) {
        bail!(
            "unsupported config version {}; expected 1 or 2",
            config.version
        );
    }
    safe_relative_path(&config.fixtures_dir, "fixtures_dir")?;
    Ok(config)
}

fn fixtures_dir(root: &Path, config: &ProjectConfig) -> Result<PathBuf> {
    Ok(root.join(safe_relative_path(&config.fixtures_dir, "fixtures_dir")?))
}

fn fixture_path(root: &Path, config: &ProjectConfig, name: &str) -> Result<PathBuf> {
    validate_fixture_name(name)?;
    Ok(fixtures_dir(root, config)?.join(format!("{name}.mcp.json")))
}

fn ensure_project(root: &Path) -> Result<(ProjectConfig, bool)> {
    let config_path = root.join(CONFIG_FILE);
    let created = !config_path.exists();
    let config = if created {
        let config = default_config();
        fs::write(&config_path, serde_json::to_string_pretty(&config)? + "\n")?;
        config
    } else {
        load_config(root)?
    };
    let dir = fixtures_dir(root, &config)?;
    fs::create_dir_all(&dir)?;
    Ok((config, created))
}

fn normalized_value(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted = BTreeMap::new();
            for (key, value) in map {
                sorted.insert(key, normalized_value(value));
            }
            Value::Object(sorted.into_iter().collect::<Map<_, _>>())
        }
        Value::Array(values) => Value::Array(values.into_iter().map(normalized_value).collect()),
        value => value,
    }
}

fn normalize_named_items(items: Value) -> Result<Value> {
    let items = items
        .as_array()
        .context("MCP list response must be an array")?;
    let mut named_items = Vec::with_capacity(items.len());
    for item in items {
        let name = item
            .get("name")
            .and_then(Value::as_str)
            .filter(|name| !name.is_empty())
            .context("every item in an MCP list response must have a non-empty string name")?;
        named_items.push((name.to_owned(), normalized_value(item.clone())));
    }
    named_items.sort_by(|left, right| left.0.cmp(&right.0));
    if named_items.windows(2).any(|pair| pair[0].0 == pair[1].0) {
        bail!("MCP list response contains duplicate names");
    }
    Ok(Value::Array(
        named_items.into_iter().map(|(_, item)| item).collect(),
    ))
}

fn normalize_tools(tools: Value) -> Result<Value> {
    normalize_named_items(tools)
}

fn tool_names(tools: &Value) -> Vec<String> {
    tools
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|tool| tool.get("name").and_then(Value::as_str))
        .map(str::to_owned)
        .collect()
}

fn tool_map(tools: &Value) -> BTreeMap<&str, &Value> {
    tools
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|tool| {
            tool.get("name")
                .and_then(Value::as_str)
                .map(|name| (name, tool))
        })
        .collect()
}

fn contract_changes(expected: &Value, actual: &Value) -> Vec<String> {
    let expected = tool_map(expected);
    let actual = tool_map(actual);
    let removed = expected
        .keys()
        .filter(|name| !actual.contains_key(**name))
        .map(|name| format!("removed tool `{name}`"));
    let added = actual
        .keys()
        .filter(|name| !expected.contains_key(**name))
        .map(|name| format!("added tool `{name}`"));
    let changed = expected
        .iter()
        .filter(|(name, tool)| {
            actual
                .get(**name)
                .is_some_and(|candidate| *candidate != **tool)
        })
        .map(|(name, _)| format!("changed contract for `{name}`"));
    removed.chain(added).chain(changed).collect()
}

fn validate_fixture(fixture: &Fixture, path: &Path) -> Result<()> {
    if !(1..=2).contains(&fixture.version) {
        bail!(
            "{}: unsupported fixture version {}; expected 1 or 2",
            path.display(),
            fixture.version
        );
    }
    validate_fixture_name(&fixture.name)
        .with_context(|| format!("{}: invalid fixture name", path.display()))?;
    if fixture.transport != "stdio" {
        bail!(
            "{}: unsupported transport {}",
            fixture.name,
            fixture.transport
        );
    }
    if fixture.server.is_empty() || fixture.server[0].is_empty() {
        bail!("{}: server command is empty", fixture.name);
    }
    normalize_tools(fixture.tools.clone())?;
    normalize_named_items(fixture.resources.clone())?;
    normalize_named_items(fixture.prompts.clone())?;
    if fixture.settings.timeout_ms == 0 {
        bail!("{}: timeout_ms must be greater than zero", fixture.name);
    }
    Ok(())
}

#[derive(Serialize)]
struct InitReport<'a> {
    command: &'static str,
    status: &'static str,
    config: String,
    fixtures_dir: String,
    created: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<&'a str>,
}

#[derive(Serialize)]
struct RecordReport {
    command: &'static str,
    status: &'static str,
    fixture: String,
    path: String,
    tools: Vec<String>,
}

#[derive(Serialize)]
struct FailedFixture {
    name: String,
    reason: String,
}

#[derive(Serialize)]
struct TestReport {
    command: &'static str,
    status: &'static str,
    passed: Vec<String>,
    failed: Vec<FailedFixture>,
}

#[derive(Serialize, Clone)]
struct AuditIssue {
    area: String,
    severity: &'static str,
    score: u8,
    finding: String,
    recommendation: String,
}

#[derive(Serialize)]
struct AuditReport {
    command: &'static str,
    status: &'static str,
    protocol_version: Option<String>,
    capabilities: Value,
    tools: usize,
    resources: usize,
    prompts: usize,
    weakest_link: AuditIssue,
    findings: Vec<AuditIssue>,
}

struct Ui {
    color: bool,
    json: bool,
    interactive: bool,
}

impl Ui {
    fn new(color_mode: ColorMode, json: bool) -> Self {
        let interactive = std::io::stdout().is_terminal();
        Self {
            color: !json
                && match color_mode {
                    ColorMode::Auto => interactive,
                    ColorMode::Always => true,
                    ColorMode::Never => false,
                },
            json,
            interactive,
        }
    }

    fn paint(&self, code: &str, text: impl AsRef<str>) -> String {
        if self.color {
            format!("\x1b[{code}m{}\x1b[0m", text.as_ref())
        } else {
            text.as_ref().to_owned()
        }
    }

    fn header(&self, title: &str) {
        if !self.json && self.interactive {
            println!(
                "{}  {}",
                self.paint("1;38;5;110", "ASTRAY VERIFY"),
                self.paint("2", "MCP CONTRACTS")
            );
            println!(
                "{}",
                self.paint("2", "────────────────────────────────────────")
            );
            println!("{}", self.paint("1", title));
        }
    }

    fn step(&self, text: impl AsRef<str>) {
        if !self.json {
            println!("{} {}", self.paint("38;5;110", "●"), text.as_ref());
        }
    }

    fn success(&self, text: impl AsRef<str>) {
        if !self.json {
            println!("{} {}", self.paint("1;32", "✓"), text.as_ref());
        }
    }

    fn failure(&self, text: impl AsRef<str>) {
        if !self.json {
            println!("{} {}", self.paint("1;31", "✗"), text.as_ref());
        }
    }

    fn error(&self, text: impl AsRef<str>) {
        if !self.json {
            eprintln!("{} {}", self.paint("1;31", "✗"), text.as_ref());
        }
    }

    fn detail(&self, label: &str, value: impl AsRef<str>) {
        if !self.json {
            println!("  {} {}", self.paint("2", label), value.as_ref());
        }
    }

    fn report<T: Serialize>(&self, report: &T) -> Result<()> {
        if self.json {
            println!("{}", serde_json::to_string(report)?);
        }
        Ok(())
    }
}

fn init(root: &Path, ui: &Ui) -> Result<ProjectConfig> {
    ui.header("Initialize project");
    let (config, created) = ensure_project(root)?;
    let config_path = root.join(CONFIG_FILE);
    let dir = fixtures_dir(root, &config)?;
    if created {
        ui.success(format!("Created {}", config_path.display()));
    } else {
        ui.success(format!("Using {}", config_path.display()));
    }
    ui.detail("fixtures", dir.display().to_string());
    ui.report(&InitReport {
        command: "init",
        status: "ok",
        config: config_path.display().to_string(),
        fixtures_dir: dir.display().to_string(),
        created,
        note: (!created).then_some("configuration already existed"),
    })?;
    Ok(config)
}

fn record(
    root: &Path,
    name: String,
    checks: Vec<CheckKind>,
    timeout_ms: Option<u64>,
    server: Vec<String>,
    ui: &Ui,
) -> Result<()> {
    validate_fixture_name(&name)?;
    let (config, created) = ensure_project(root)?;
    let path = fixture_path(root, &config, &name)?;
    ui.header("Record contract");
    if created {
        ui.success(format!("Created {}", root.join(CONFIG_FILE).display()));
    }
    ui.step(format!("Starting {name}"));
    let settings = CheckSettings {
        checks,
        timeout_ms: timeout_ms.unwrap_or(config.defaults.timeout_ms),
    };
    if settings.timeout_ms == 0 {
        bail!("timeout_ms must be greater than zero");
    }
    let mut process = McpProcess::start(&server)?;
    ui.step("Negotiating MCP connection");
    let contract = process.discover_contract(&settings)?;
    let names = tool_names(&contract.tools);
    ui.success("MCP handshake complete");
    ui.success(format!(
        "Captured {} tool{}",
        names.len(),
        if names.len() == 1 { "" } else { "s" }
    ));
    let fixture = Fixture {
        version: 2,
        name: name.clone(),
        transport: "stdio".into(),
        server,
        protocol_version: contract
            .protocol_version
            .unwrap_or_else(|| MCP_VERSION.into()),
        tools: contract.tools,
        resources: contract.resources,
        prompts: contract.prompts,
        settings,
    };
    fs::write(&path, serde_json::to_string_pretty(&fixture)? + "\n")?;
    ui.success(format!("Saved {}", path.display()));
    ui.report(&RecordReport {
        command: "record",
        status: "ok",
        fixture: name,
        path: path.display().to_string(),
        tools: names,
    })
}

fn test_fixture(path: &Path, timeout_override: Option<u64>, ui: &Ui) -> Result<String> {
    let fixture: Fixture = serde_json::from_str(
        &fs::read_to_string(path).with_context(|| format!("could not read {}", path.display()))?,
    )
    .with_context(|| format!("could not parse fixture {}", path.display()))?;
    validate_fixture(&fixture, path)?;
    let settings = CheckSettings {
        timeout_ms: timeout_override.unwrap_or(fixture.settings.timeout_ms),
        ..fixture.settings.clone()
    };
    if settings.timeout_ms == 0 {
        bail!("{}: timeout_ms must be greater than zero", fixture.name);
    }
    ui.step(format!("Checking {}", fixture.name));
    let mut process = McpProcess::start(&fixture.server)?;
    let actual = process.discover_contract(&settings)?;
    let expected_tools = normalize_tools(fixture.tools)?;
    let expected_resources = normalize_named_items(fixture.resources)?;
    let expected_prompts = normalize_named_items(fixture.prompts)?;
    let mut changes = contract_changes(&expected_tools, &actual.tools);
    if settings.checks.contains(&CheckKind::Resources) {
        changes.extend(
            contract_changes(&expected_resources, &actual.resources)
                .into_iter()
                .map(|change| format!("resources: {change}")),
        );
    }
    if settings.checks.contains(&CheckKind::Prompts) {
        changes.extend(
            contract_changes(&expected_prompts, &actual.prompts)
                .into_iter()
                .map(|change| format!("prompts: {change}")),
        );
    }
    if changes.is_empty() && actual.tools == expected_tools {
        ui.success(format!("PASS  {}", fixture.name));
        return Ok(fixture.name);
    }
    ui.failure(format!("FAIL  {}", fixture.name));
    for change in &changes {
        ui.detail("", change);
    }
    if changes.is_empty() {
        ui.detail(
            "",
            "tools/list changed; record again only if this was intentional.",
        );
    }
    bail!("fixture mismatch: {}", path.display())
}

fn run_tests(
    root: &Path,
    only_name: Option<String>,
    timeout_override: Option<u64>,
    ui: &Ui,
) -> Result<bool> {
    ui.header("Verify contracts");
    let config = load_config(root)?;
    let mut paths = if let Some(name) = only_name {
        vec![fixture_path(root, &config, &name)?]
    } else {
        let dir = fixtures_dir(root, &config)?;
        fs::read_dir(&dir)
            .with_context(|| format!("no fixtures directory at {}; run init first", dir.display()))?
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
            .collect::<Vec<_>>()
    };
    paths.sort();
    if paths.is_empty() {
        bail!("no fixtures found; run record first");
    }
    let mut passed = Vec::new();
    let mut failed = Vec::new();
    for path in paths {
        match test_fixture(&path, timeout_override, ui) {
            Ok(name) => passed.push(name),
            Err(error) => {
                let name = path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or("unknown")
                    .trim_end_matches(".mcp")
                    .to_owned();
                failed.push(FailedFixture {
                    name,
                    reason: format!("{error:#}"),
                });
            }
        }
    }
    let status = if failed.is_empty() { "ok" } else { "failed" };
    if !ui.json {
        let summary = format!("{} passed · {} failed", passed.len(), failed.len());
        if failed.is_empty() {
            ui.success(summary);
        } else {
            ui.failure(summary);
        }
    }
    ui.report(&TestReport {
        command: "test",
        status,
        passed,
        failed,
    })?;
    Ok(status == "ok")
}

fn audit_contract(contract: &ServerContract) -> Vec<AuditIssue> {
    let mut findings = Vec::new();
    if contract.protocol_version.is_none() {
        findings.push(AuditIssue {
            area: "protocol handshake".into(),
            severity: "critical",
            score: 100,
            finding: "initialize did not return protocolVersion".into(),
            recommendation: "Return the negotiated protocolVersion from initialize before serving other requests.".into(),
        });
    }
    if !contract.capabilities.get("tools").is_some()
        && !contract
            .tools
            .as_array()
            .is_some_and(|items| items.is_empty())
    {
        findings.push(AuditIssue {
            area: "capability advertisement".into(),
            severity: "high",
            score: 85,
            finding: "tools/list returned tools but initialize did not advertise tools capability".into(),
            recommendation: "Declare the tools capability in initialize so clients can discover the surface reliably.".into(),
        });
    }
    for (area, values) in [
        ("tools", &contract.tools),
        ("resources", &contract.resources),
        ("prompts", &contract.prompts),
    ] {
        let items = values.as_array().expect("normalized MCP lists are arrays");
        if items.is_empty() {
            findings.push(AuditIssue {
                area: area.into(),
                severity: "medium",
                score: 45,
                finding: format!("{area}/list exposes no items"),
                recommendation: format!(
                    "Confirm that {area} are intentionally unavailable, or expose and snapshot the expected {area} contract."
                ),
            });
        }
        for item in items {
            if item
                .get("description")
                .and_then(Value::as_str)
                .is_none_or(str::is_empty)
            {
                findings.push(AuditIssue {
                    area: format!("{area} contract"),
                    severity: "low",
                    score: 20,
                    finding: format!(
                        "`{}` has no description",
                        item.get("name")
                            .and_then(Value::as_str)
                            .unwrap_or("unknown")
                    ),
                    recommendation:
                        "Add a concise description so MCP clients and agents can select it safely."
                            .into(),
                });
            }
        }
    }
    if findings.is_empty() {
        findings.push(AuditIssue {
            area: "MCP contract".into(),
            severity: "info",
            score: 0,
            finding: "No protocol or discoverability weaknesses detected in the selected checks."
                .into(),
            recommendation: "Keep the recorded fixture in CI to preserve this baseline.".into(),
        });
    }
    findings.sort_by(|left, right| right.score.cmp(&left.score));
    findings
}

fn audit(
    root: &Path,
    name: Option<String>,
    timeout_ms: u64,
    server: Vec<String>,
    ui: &Ui,
) -> Result<()> {
    if timeout_ms == 0 {
        bail!("timeout_ms must be greater than zero");
    }
    let (server, settings) = if server.is_empty() {
        let name = name
            .context("provide a server command after -- or select a saved fixture with --name")?;
        let config = load_config(root)?;
        let path = fixture_path(root, &config, &name)?;
        let fixture: Fixture = serde_json::from_str(
            &fs::read_to_string(&path)
                .with_context(|| format!("could not read {}", path.display()))?,
        )?;
        validate_fixture(&fixture, &path)?;
        (
            fixture.server,
            CheckSettings {
                checks: vec![CheckKind::Tools, CheckKind::Resources, CheckKind::Prompts],
                timeout_ms,
            },
        )
    } else {
        (
            server,
            CheckSettings {
                checks: vec![CheckKind::Tools, CheckKind::Resources, CheckKind::Prompts],
                timeout_ms,
            },
        )
    };
    ui.header("Audit MCP server");
    ui.step("Inspecting MCP handshake and discovery surfaces");
    let mut process = McpProcess::start(&server)?;
    let contract = process.discover_contract(&settings)?;
    let findings = audit_contract(&contract);
    let weakest_link = findings[0].clone();
    if !ui.json {
        ui.failure(format!(
            "Weakest link: {} ({})",
            weakest_link.area, weakest_link.severity
        ));
        ui.detail("finding", &weakest_link.finding);
        ui.detail("action", &weakest_link.recommendation);
        for finding in findings.iter().skip(1) {
            ui.detail(
                &finding.severity,
                format!("{}: {}", finding.area, finding.finding),
            );
        }
    }
    ui.report(&AuditReport {
        command: "audit",
        status: "ok",
        protocol_version: contract.protocol_version,
        capabilities: contract.capabilities,
        tools: tool_names(&contract.tools).len(),
        resources: tool_names(&contract.resources).len(),
        prompts: tool_names(&contract.prompts).len(),
        weakest_link,
        findings,
    })
}

fn show_config(root: &Path, ui: &Ui) -> Result<()> {
    let config = load_config(root)?;
    if !ui.json {
        ui.header("Project configuration");
        ui.detail("fixtures", &config.fixtures_dir);
        ui.detail(
            "checks",
            format!("{:?}", config.defaults.checks).to_lowercase(),
        );
        ui.detail("timeout", format!("{} ms", config.defaults.timeout_ms));
    }
    ui.report(&json!({"command": "config", "status": "ok", "config": config}))
}

fn doctor(root: &Path, with_test: bool, ui: &Ui) -> Result<()> {
    ui.header("Diagnose project");
    let mut checks = Vec::new();
    let mut ok = true;
    let config_path = root.join(CONFIG_FILE);
    let config = if config_path.exists() {
        let config = load_config(root)?;
        ui.success(format!("Config file present: {}", config_path.display()));
        checks.push(
            json!({"name": "config", "status": "ok", "path": config_path.display().to_string()}),
        );
        config
    } else {
        let (config, _) = ensure_project(root)?;
        ui.success(format!("Created {}", config_path.display()));
        checks.push(json!({"name": "config", "status": "initialized", "path": config_path.display().to_string()}));
        config
    };
    let dir = fixtures_dir(root, &config)?;
    let fixtures = fs::read_dir(&dir)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok().map(|entry| entry.path()))
                .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
                .count()
        })
        .unwrap_or(0);
    if fixtures == 0 {
        ui.failure("No fixtures captured yet");
        ok = false;
    } else {
        ui.success(format!(
            "{fixtures} fixture{} available",
            if fixtures == 1 { "" } else { "s" }
        ));
    }
    checks.push(json!({"name": "fixtures", "status": if fixtures == 0 { "missing" } else { "ok" }, "count": fixtures}));
    ui.detail(
        "default checks",
        format!("{:?}", config.defaults.checks).to_lowercase(),
    );
    ui.detail(
        "default timeout",
        format!("{} ms", config.defaults.timeout_ms),
    );
    let log_path = root.join("logs/astray-verify.jsonl");
    if log_path.exists() {
        ui.success(format!("Execution log: {}", log_path.display()));
        checks.push(json!({"name": "log", "status": "ok", "path": log_path.display().to_string()}));
    } else {
        ui.detail("log", "no execution log yet; pass --log <path> to enable");
        checks.push(
            json!({"name": "log", "status": "absent", "path": log_path.display().to_string()}),
        );
    }
    if with_test {
        let passed = run_tests(root, None, None, ui)?;
        if !passed {
            ok = false;
        }
    }
    ui.report(&json!({"command": "doctor", "status": if ok { "ok" } else { "attention" }, "checks": checks}))?;
    Ok(())
}

fn run_watch(root: &Path, initial: bool, ui: &Ui) -> Result<bool> {
    use std::time::Duration as StdDuration;
    let config = load_config(root)?;
    let config_path = root.join(CONFIG_FILE);
    let fixtures_path = fixtures_dir(root, &config)?;
    let watched = vec![config_path, fixtures_path];
    if initial {
        let _ = run_tests(root, None, None, ui)?;
    }
    ui.header("Watching project for changes");
    ui.step("Press Ctrl+C to stop");
    let mut last_signature: Option<(u64, u64)> = None;
    loop {
        let mut signature = (0u64, 0u64);
        for path in &watched {
            signature.0 = signature.0.wrapping_add(read_mtime(
                path.metadata().ok().and_then(|meta| meta.modified().ok()),
            ));
            if path.is_dir() {
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries.flatten() {
                        signature.1 = signature.1.wrapping_add(read_mtime(
                            entry
                                .path()
                                .metadata()
                                .ok()
                                .and_then(|meta| meta.modified().ok()),
                        ));
                    }
                }
            }
        }
        if last_signature.is_some() && Some(signature) != last_signature {
            ui.step("Changes detected, re-running tests");
            let _ = run_tests(root, None, None, ui);
        }
        last_signature = Some(signature);
        thread::sleep(StdDuration::from_millis(750));
    }
}

fn read_mtime(time: Option<SystemTime>) -> u64 {
    time.and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn run(cli: Cli, ui: &Ui) -> Result<bool> {
    let root = project_root()?;
    let log_path = cli
        .log
        .as_deref()
        .map(|path| safe_relative_path(path, "log path"))
        .transpose()?
        .map(|path| root.join(path));
    let command = match &cli.command {
        Commands::Init => "init",
        Commands::Record { .. } => "record",
        Commands::Test { .. } => "test",
        Commands::Audit { .. } => "audit",
        Commands::Config => "config",
        Commands::Doctor { .. } => "doctor",
        Commands::Watch { .. } => "watch",
    };
    let result = match cli.command {
        Commands::Init => init(&root, ui).map(|_| true),
        Commands::Record {
            name,
            checks,
            timeout_ms,
            server,
        } => record(&root, name, checks, timeout_ms, server, ui).map(|_| true),
        Commands::Test { name, timeout_ms } => run_tests(&root, name, timeout_ms, ui),
        Commands::Audit {
            name,
            timeout_ms,
            server,
        } => audit(&root, name, timeout_ms, server, ui).map(|_| true),
        Commands::Config => show_config(&root, ui).map(|_| true),
        Commands::Doctor { with_test } => doctor(&root, with_test, ui).map(|_| true),
        Commands::Watch { initial } => run_watch(&root, initial, ui).map(|_| true),
    };
    if let Some(path) = log_path {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let status = if matches!(result, Ok(true)) {
            "ok"
        } else {
            "failed"
        };
        let error = result.as_ref().err().map(|error| format!("{error:#}"));
        let entry =
            json!({"timestamp": timestamp, "command": command, "status": status, "error": error});
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        writeln!(file, "{}", serde_json::to_string(&entry)?)?;
    }
    result
}

fn main() {
    let cli = Cli::parse();
    let json = cli.json;
    let ui = Ui::new(cli.color, json);
    match run(cli, &ui) {
        Ok(true) => {}
        Ok(false) => std::process::exit(1),
        Err(error) => {
            if json {
                println!(
                    "{}",
                    json!({"status": "error", "error": format!("{error:#}")})
                );
            } else {
                ui.error(format!("{error:#}"));
            }
            std::process::exit(1);
        }
    }
}
