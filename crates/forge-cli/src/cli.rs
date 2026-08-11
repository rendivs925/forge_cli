use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "forge",
    version,
    about = "A software quality control plane",
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, clap::Args)]
pub struct GlobalArgs {
    /// Reduce output to errors only
    #[arg(short = 'q', long = "quiet", global = true)]
    pub quiet: bool,

    /// Increase verbosity; repeat for more detail
    #[arg(short = 'v', long = "verbose", action = ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Disable colored output
    #[arg(long = "no-color", global = true)]
    pub no_color: bool,

    /// Output format
    #[arg(long = "format", value_enum, default_value_t = Format::Terminal, global = true)]
    pub format: Format,

    /// Path to the configuration file
    #[arg(long = "config", global = true)]
    pub config: Option<PathBuf>,

    /// Analysis profile to use
    #[arg(long = "profile", global = true)]
    pub profile: Option<String>,

    /// Workspace root directory
    #[arg(long = "workspace", global = true)]
    pub workspace: Option<PathBuf>,

    /// Do not access the network
    #[arg(long = "offline", action = ArgAction::SetTrue, global = true)]
    pub offline: Option<bool>,

    /// Disable the analysis cache
    #[arg(long = "no-cache", action = ArgAction::SetTrue, global = true)]
    pub no_cache: Option<bool>,

    /// Stop after the first failure
    #[arg(long = "fail-fast", action = ArgAction::SetTrue, global = true)]
    pub fail_fast: Option<bool>,

    /// Report command timings
    #[arg(long = "timings", global = true)]
    pub timings: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Format {
    Terminal,
    Json,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialize Forge in a repository
    Init,
    /// Fast local quality check
    Check,
    /// Full analysis
    Scan(ScanArgs),
    /// Evaluate quality gate
    Gate,
    /// Manage rules and rule packs
    Rules(RulesArgs),
    /// Manage integrated analyzers
    Tools(ToolsArgs),
    /// Manage analysis profiles
    Profile(ProfileArgs),
    /// Manage quality policies
    Policy(PolicyArgs),
    /// Manage existing technical debt
    Baseline(BaselineArgs),
    /// Explain a finding or rule
    Explain(ExplainArgs),
    /// Apply supported automatic fixes
    Fix(FixArgs),
    /// Generate reports
    Report(ReportArgs),
    /// Analyze changes
    Diff(DiffArgs),
    /// Inspect effective configuration
    Config(ConfigArgs),
    /// Diagnose the Forge environment
    Doctor,
    /// Manage the analysis cache
    Cache(CacheArgs),
    /// Print version information
    Version,
}

#[derive(Debug, Clone, clap::Args)]
pub struct ScanArgs {
    /// Evaluate the quality gate after scanning
    #[arg(long)]
    pub gate: bool,
}

#[derive(Debug, Clone, clap::Args)]
pub struct RulesArgs {
    #[command(subcommand)]
    pub command: RulesCommand,
}

#[derive(Debug, Clone, Subcommand)]
pub enum RulesCommand {
    /// List available rules
    List,
    /// Explain a rule
    Explain { rule: String },
}

#[derive(Debug, Clone, clap::Args)]
pub struct ToolsArgs {
    #[command(subcommand)]
    pub command: ToolsCommand,
}

#[derive(Debug, Clone, Subcommand)]
pub enum ToolsCommand {
    /// List integrated tools
    List,
    /// Diagnose tool environment
    Doctor,
}

#[derive(Debug, Clone, clap::Args)]
pub struct ProfileArgs {
    #[command(subcommand)]
    pub command: ProfileCommand,
}

#[derive(Debug, Clone, Subcommand)]
pub enum ProfileCommand {
    /// List analysis profiles
    List,
    /// Show a profile
    Show { name: String },
}

#[derive(Debug, Clone, clap::Args)]
pub struct PolicyArgs {
    #[command(subcommand)]
    pub command: PolicyCommand,
}

#[derive(Debug, Clone, Subcommand)]
pub enum PolicyCommand {
    /// List quality policies
    List,
    /// Show a policy
    Show { name: String },
}

#[derive(Debug, Clone, clap::Args)]
pub struct BaselineArgs {
    #[command(subcommand)]
    pub command: BaselineCommand,
}

#[derive(Debug, Clone, Subcommand)]
pub enum BaselineCommand {
    /// Record current findings as the baseline
    Create,
    /// Show the current baseline
    Show,
    /// Update the baseline
    Update,
    /// Clear the baseline
    Clear,
}

#[derive(Debug, Clone, clap::Args)]
pub struct ExplainArgs {
    /// Finding or rule identifier
    pub target: String,
}

#[derive(Debug, Clone, clap::Args)]
pub struct FixArgs {
    /// Show available fixes without applying them
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Debug, Clone, clap::Args)]
pub struct ReportArgs {
    #[command(subcommand)]
    pub command: ReportCommand,
}

#[derive(Debug, Clone, Subcommand)]
pub enum ReportCommand {
    /// Generate a report from the last analysis
    Generate,
}

#[derive(Debug, Clone, clap::Args)]
pub struct DiffArgs {
    /// Base revision to compare against
    #[arg(long)]
    pub base: Option<String>,
}

#[derive(Debug, Clone, clap::Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

#[derive(Debug, Clone, Subcommand)]
pub enum ConfigCommand {
    /// Show the effective configuration
    Show,
    /// Explain where a configuration value comes from
    Explain { key: String },
}

#[derive(Debug, Clone, clap::Args)]
pub struct CacheArgs {
    #[command(subcommand)]
    pub command: CacheCommand,
}

#[derive(Debug, Clone, Subcommand)]
pub enum CacheCommand {
    /// Show cache status
    Status,
    /// Clear the cache
    Clear,
    /// Prune stale cache entries
    Prune,
}
