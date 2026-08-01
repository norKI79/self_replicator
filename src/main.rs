use std::{
    env,
    fs,
    io,
    path::{Path, PathBuf},
    process,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() -> io::Result<()> {
    let dir_name = generate_directory_name();
    let directory = Path::new(&dir_name);

    fs::create_dir(directory)?;

    println!("Created directory: {}", directory.display());

    let copied_file = copy_self(directory)?;

    println!("Copied executable to: {}", copied_file.display());

    Ok(())
}

fn generate_directory_name() -> String {
    // Combine current time + PID to create a mostly unique name.
    // This is not random, but is sufficient for avoiding normal collisions.
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System clock error")
        .as_nanos();

    let pid = process::id();

    format!("{:x}_{:05x}", timestamp, pid)
}

fn copy_self(destination: &Path) -> io::Result<PathBuf> {
    // Finds the path of the currently running executable.
    let current_exe = env::current_exe()?;

    let filename = current_exe
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No filename found"))?;

    // PathBuf safely builds paths instead of manually joining strings.
    let destination_file = destination.join(filename);

    fs::copy(&current_exe, &destination_file)?;

    Ok(destination_file)
}