use {
    clap::{
        CommandFactory,
        Parser,
    },
    std::path::{
        Path,
        PathBuf,
    },
    termimad::crossterm::style::Stylize,
};

static INTRO: &str = "
ddoc is a markdown based static site generator.

Documentation at https://dystroy.org/ddoc

";

/// Launch arguments
#[derive(Debug, Parser)]
#[command(
    author,
    about,
    version,
    disable_version_flag = true,
    disable_help_flag = true
)]
pub struct Args {
    /// Print help information
    #[arg(long)]
    pub help: bool,

    /// Print the version
    #[arg(long)]
    pub version: bool,

    /// Initialize a ddoc project in the specified directory
    #[arg(long)]
    pub init: bool,

    /// Gives the list of known plugins, which can then be reset with `--reset-plugin`
    #[arg(long)]
    pub list_plugins: bool,

    /// Init the specified plugin in the project
    /// (will ask for confirmation if the plugin already exists in the project)
    #[arg(long)]
    pub init_plugin: Option<String>,

    /// Serve files (for local development)
    #[arg(long)]
    pub serve: bool,

    /// Port to use when serving files (default: 8004)
    #[arg(long)]
    pub port: Option<u16>,

    pub path: Option<PathBuf>,
}

impl Args {
    pub fn print_help(&self) {
        let printer = clap_help::Printer::new(Args::command())
            .with("introduction", INTRO)
            .with("options", clap_help::TEMPLATE_OPTIONS_MERGED_VALUE)
            .without("author");
        printer.print_help();
    }
    /// Return a verified existing path to a ddoc project,
    /// or None if no path was specified and the --init flag is not set.
    ///
    /// Print errors if needed
    pub fn project_path(&self) -> Option<PathBuf> {
        let project_path = self.path.as_deref().unwrap_or(Path::new("."));
        if !project_path.exists() {
            if self.init {
                // If the path doesn't exist but the user wants to initialize
                // a project there, we will create it later, so we don't print an error
                return Some(project_path.to_owned());
            }
            eprintln!(
                "Path {} does not exist.",
                project_path.to_string_lossy().red().bold(),
            );
            eprintln!(
                "Use {} to initialize a new ddoc project at this location.",
                "ddoc --init".green(),
            );
            return None;
        }
        if !project_path.is_dir() {
            eprintln!(
                "Path {} is not a directory.",
                project_path.to_string_lossy().red().bold(),
            );
            eprintln!("Please check that the path is correct.");
            return None;
        }
        let Ok(project_path) = project_path.canonicalize() else {
            eprintln!(
                "Error accessing path {}.",
                project_path.to_string_lossy().red().bold(),
            );
            eprintln!(
                "Please check that the path is correct and that you have the necessary permissions.",
            );
            return None;
        };
        Some(project_path)
    }
}
