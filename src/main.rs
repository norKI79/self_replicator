use std::{
    env,
    fs,
    io,
    path::{Path, PathBuf},
    process,
    time::{SystemTime, UNIX_EPOCH},
};

use sha2::{Digest, Sha256};

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

    if config.verbose {
        verify_copy(&env::current_exe()?, &copied_file)?;
    }

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

fn calculate_sha256(path: &Path) -> io::Result<String> {
    let data = fs::read(path)?;

    let mut hasher = Sha256::new();
    hasher.update(data);

    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}

fn verify_copy(original: &Path, copied: &Path) -> io::Result<()> {
    println!("\nSHA-256 Verification:");

    let original_hash = calculate_sha256(original)?;
    let copied_hash = calculate_sha256(copied)?;

    println!("Original  : {}", original_hash);
    println!("Copied    : {}", copied_hash);

    if original_hash == copied_hash {
        println!("Verification successful");
    } else {
        println!("Verification FAILED");
    }

    Ok(())
}