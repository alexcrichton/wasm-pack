use std::env;
use std::fs;

use failure::{Error, ResultExt};

pub fn install() -> Result<(), Error> {
    let path = env::var_os("PATH").unwrap_or_default();
    let rustup = env::split_paths(&path)
        .map(|p| p.join("rustup").with_extension(env::consts::EXE_EXTENSION))
        .find(|p| p.exists());
    let rustup = match rustup {
        Some(path) => path,
        None => {
            bail!(
                "failed to find an installation of `rustup` in `PATH`, \
                 is rustup already installed?"
            );
        }
    };
    let installation_dir = match rustup.parent() {
        Some(parent) => parent,
        None => bail!("can't install when `rustup` is at the root of the filesystem"),
    };
    let destination = installation_dir
        .join("wasm-pack")
        .with_extension(env::consts::EXE_EXTENSION);

    if destination.exists() {
        if !env::args().any(|arg| arg == "-f") {
            bail!(
                "existing wasm-pack installation found at `{}`, pass `-f` to \
                 force installation over this file, otherwise aborting \
                 installation now",
                destination.display()
            );
        }
    }

    let me = env::current_exe()?;
    fs::copy(&me, &destination)
        .with_context(|_| format!("failed to copy executable to `{}`", destination.display()))?;
    println!(
        "info: successfully installed wasm-pack to `{}`",
        destination.display()
    );

    Ok(())
}
