use std::{
    env,
    fs,
    io,
    path::{Path, PathBuf},
    process,
    time::{SystemTime, UNIX_EPOCH},
};

struct Config {
    destination: PathBuf,
    filename: Option<String>,
    verbose: bool,
}

fn main() -> io::Result<()> {
    let config = parse_arguments()?;

    fs::create_dir_all(&config.destination)?;

    if config.verbose {
        println!("Created directory: {}", config.destination.display());
    }

    let copied_file = copy_self(
        &config.destination,
        config.filename.as_deref(),
        config.verbose,
    )?;

    println!("Copied executable to: {}", copied_file.display());

    Ok(())
}

fn parse_arguments() -> io::Result<Config> {
    let mut destination = default_destination();
    let mut filename = None;
    let mut verbose = false;

    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--dest" => {
                let value = args.next().ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "--dest requires a path",
                    )
                })?;

                destination = PathBuf::from(value);
            }

            "--name" => {
                let value = args.next().ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "--name requires a filename",
                    )
                })?;

                filename = Some(value);
            }

            "--verbose" => {
                verbose = true;
            }

            unknown => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Unknown argument: {}", unknown),
                ));
            }
        }
    }

    Ok(Config {
        destination,
        filename,
        verbose,
    })
}

fn default_destination() -> PathBuf {
    let sandbox = PathBuf::from("sandbox");

    if !sandbox.exists() {
        fs::create_dir_all(&sandbox)
            .expect("Failed to create sandbox directory");
    }

    sandbox.join(generate_directory_name())
}

fn generate_directory_name() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System clock error")
        .as_nanos();

    let pid = process::id();

    format!("{:x}_{:05x}", timestamp, pid)
}

fn copy_self(
    destination: &Path,
    new_name: Option<&str>,
    verbose: bool,
) -> io::Result<PathBuf> {
    let current_exe = env::current_exe()?;

    let filename = match new_name {
        Some(name) => name.into(),

        None => current_exe
            .file_name()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "No filename found",
                )
            })?
            .to_os_string(),
    };

    let destination_file = destination.join(filename);

    if verbose {
        println!("Source      : {}", current_exe.display());
        println!("Destination : {}", destination_file.display());
    }

    fs::copy(&current_exe, &destination_file)?;

    Ok(destination_file)
}