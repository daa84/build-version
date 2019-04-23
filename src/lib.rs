//! Application version information from git
//!
//! Creates constant GIT_BUILD_VERSION from git command `git describe --tags --always`
//! 
//! Add to `bulid.rs`:
//!
//! ```no_run
//! extern crate build_version;
//! fn main() {
//!     build_version::write_version_file().expect("Failed to write version.rs file");
//! }
//! ```
//!
//! Add to `main.rs`:
//!
//! ```ignore
//! include!(concat!(env!("OUT_DIR"), "/version.rs"));
//! ```
#[macro_use]
extern crate quick_error;

use std::process::Command;

use std::env;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Read, Write};
use std::path::Path;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: std::io::Error) {
            from()
        }
        MissingEnvVar {
        }
    }
}

fn same_content_as(path: &Path, content: &str) -> Result<bool, Error> {
    let mut f = File::open(path)?;
    let mut current = String::new();
    f.read_to_string(&mut current)?;

    Ok(current == content)
}

fn git_describe() -> Option<String> {
    Command::new("git")
        .args(&["describe", "--tags", "--always"])
        .output()
        .ok()
        .and_then(|out| {
            std::str::from_utf8(&out.stdout[..])
                .map(str::trim)
                .map(str::to_owned)
                .ok()
        })
}

/// Write version.rs file to OUT_DIR
pub fn write_version_file() -> Result<(), Error> {
    let path = env::var_os("OUT_DIR").ok_or(Error::MissingEnvVar)?;
    let path: &Path = path.as_ref();

    create_dir_all(path)?;

    let path = path.join("version.rs");

    let describe = git_describe();

    let content = if let Some(describe) = describe {
        format!("static GIT_BUILD_VERSION: Option<&'static str> = Some(\"{}\");\n", describe)
    } else {
        "static GIT_BUILD_VERSION: Option<&'static str> = None;\n".to_owned()
    };

    let is_fresh = if path.exists() {
        same_content_as(&path, &content)?
    } else {
        false
    };

    if !is_fresh {
        let mut file = BufWriter::new(File::create(&path)?);

        write!(file, "{}", content)?;
    }
    Ok(())
}
