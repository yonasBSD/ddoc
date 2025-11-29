mod args;

pub use args::*;

use {
    crate::*,
    clap::Parser,
    std::path::Path,
};

pub fn run() -> DdResult<()> {
    let args: Args = Args::parse();
    info!("args: {:#?}", &args);

    if args.help {
        args.print_help();
        return Ok(());
    }

    if args.version {
        println!("cradoc {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // TODO support ~
    let project_path = args.path.as_deref().unwrap_or(Path::new("."));
    let project_path = project_path.canonicalize().map_err(|error| DdError::Read {
        path: project_path.to_owned(),
        error,
    })?;
    let project = Project::load(&project_path)?;

    project.build()?;

    if args.serve {
        let port = args.port.unwrap_or(8004);
        serve_project(&project, port)?;
    }

    Ok(())
}
