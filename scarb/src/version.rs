//! Version information about Scarb and Cairo.

use std::fmt;
use std::fmt::Write;

use serde::{Deserialize, Serialize};

/// Scarb's version.
#[derive(Serialize, Deserialize, Debug)]
pub struct VersionInfo {
    pub version: String,
    pub commit_info: Option<CommitInfo>,
    pub cairo: CairoVersionInfo,
}

/// Cairo's version.
#[derive(Serialize, Deserialize, Debug)]
pub struct CairoVersionInfo {
    pub version: String,
    pub commit_info: Option<CommitInfo>,
}

/// Information about the Git repository where the crate was built from.
#[derive(Serialize, Deserialize, Debug)]
pub struct CommitInfo {
    pub short_commit_hash: String,
    pub commit_hash: String,
    pub commit_date: Option<String>,
}

impl VersionInfo {
    pub fn short(&self) -> String {
        display_version_and_commit_info(&self.version, &self.commit_info, None)
    }

    pub fn long(&self) -> String {
        format!(
            "\
                {short}\n\
                cairo: {cairo}\
            ",
            short = self.short(),
            cairo = self.cairo.short()
        )
    }
}

impl CairoVersionInfo {
    pub fn short(&self) -> String {
        display_version_and_commit_info(
            &self.version,
            &self.commit_info,
            Some("cairo-lang-compiler"),
        )
    }
}

fn display_version_and_commit_info(
    version: &str,
    commit_info: &Option<CommitInfo>,
    crate_name: Option<&str>,
) -> String {
    let mut text = version.to_string();
    if let Some(commit_info) = commit_info {
        write!(&mut text, " ({commit_info})").unwrap();
    } else if let Some(crate_name) = crate_name {
        write!(
            &mut text,
            " (https://crates.io/crates/{crate_name}/{version})"
        )
        .unwrap();
    }
    text
}

impl fmt::Display for CommitInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.short_commit_hash)?;

        if let Some(date) = &self.commit_date {
            write!(f, " {}", date)?;
        }

        Ok(())
    }
}

/// Get information about Scarb's version.
pub fn get() -> VersionInfo {
    macro_rules! option_env_str {
        ($name:expr) => {
            option_env!($name).map(|s| s.to_string())
        };
    }

    let version = env!("CARGO_PKG_VERSION").to_string();

    let commit_info = option_env_str!("SCARB_COMMIT_HASH").map(|commit_hash| CommitInfo {
        short_commit_hash: option_env_str!("SCARB_COMMIT_SHORT_HASH").unwrap(),
        commit_hash,
        commit_date: option_env_str!("SCARB_COMMIT_DATE"),
    });

    let cairo = {
        let version = env!("SCARB_CAIRO_VERSION").to_string();

        let commit_info = option_env_str!("SCARB_CAIRO_COMMIT_HASH").map(|commit_hash| {
            let mut short_commit_hash = commit_hash.clone();
            short_commit_hash.truncate(9);

            CommitInfo {
                short_commit_hash,
                commit_hash,
                commit_date: None,
            }
        });

        CairoVersionInfo {
            version,
            commit_info,
        }
    };

    VersionInfo {
        version,
        commit_info,
        cairo,
    }
}
