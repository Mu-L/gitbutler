use crate::error::Error;
use anyhow::{bail, Context};
use std::env;
use tracing::instrument;
use url::Url;

pub(crate) fn open_that(path: &str) -> anyhow::Result<()> {
    let target_url = Url::parse(path).with_context(|| format!("Invalid path format: '{path}'"))?;
    if ![
        "http",
        "https",
        "mailto",
        "vscode",
        "vscodium",
        "vscode-insiders",
        "zed",
        "windsurf",
        "cursor",
    ]
    .contains(&target_url.scheme())
    {
        bail!("Invalid path scheme: {}", target_url.scheme());
    }

    fn clean_env_vars<'a, 'b>(
        var_names: &'a [&'b str],
    ) -> impl Iterator<Item = (&'b str, String)> + 'a {
        var_names
            .iter()
            .filter_map(|name| env::var(name).map(|value| (*name, value)).ok())
            .map(|(name, value)| {
                (
                    name,
                    value
                        .split(':')
                        .filter(|path| {
                            !path.contains("appimage-run") && !path.contains("/tmp/.mount")
                        })
                        .collect::<Vec<_>>()
                        .join(":"),
                )
            })
    }

    let mut cmd_errors = Vec::new();

    for mut cmd in open::commands(path) {
        let cleaned_vars = clean_env_vars(&[
            "APPDIR",
            "GDK_PIXBUF_MODULE_FILE",
            "GIO_EXTRA_MODULES",
            "GIO_EXTRA_MODULES",
            "GSETTINGS_SCHEMA_DIR",
            "GST_PLUGIN_SYSTEM_PATH",
            "GST_PLUGIN_SYSTEM_PATH_1_0",
            "GTK_DATA_PREFIX",
            "GTK_EXE_PREFIX",
            "GTK_IM_MODULE_FILE",
            "GTK_PATH",
            "LD_LIBRARY_PATH",
            "PATH",
            "PERLLIB",
            "PYTHONHOME",
            "PYTHONPATH",
            "QT_PLUGIN_PATH",
            "XDG_DATA_DIRS",
        ]);

        cmd.envs(cleaned_vars);
        cmd.current_dir(env::temp_dir());
        if cmd.status().is_ok() {
            return Ok(());
        } else {
            cmd_errors.push(anyhow::anyhow!("Failed to execute command {:?}", cmd));
        }
    }
    if !cmd_errors.is_empty() {
        bail!("Errors occurred: {:?}", cmd_errors);
    }
    Ok(())
}

#[tauri::command(async)]
#[instrument(err(Debug))]
pub fn open_url(url: &str) -> Result<(), Error> {
    Ok(open_that(url)?)
}

#[tauri::command(async)]
#[instrument(err(Debug))]
pub fn show_in_finder(path: &str) -> Result<(), Error> {
    // Cross-platform implementation to open file/directory in the default file manager
    // macOS: Opens in Finder (with -R flag to reveal the item)
    // Windows: Opens in File Explorer
    // Linux: Opens in the default file manager

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open")
            .arg("-R")
            .arg(path)
            .status()
            .with_context(|| format!("Failed to show '{}' in Finder", path))?;
    }

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("explorer")
            .arg("/select,")
            .arg(path)
            .status()
            .with_context(|| format!("Failed to show '{}' in Explorer", path))?;
    }

    #[cfg(target_os = "linux")]
    {
        // For directories, open the directory directly
        if std::path::Path::new(path).is_dir() {
            open_that(path)
                .with_context(|| format!("Failed to open directory '{}' in file manager", path))?;
        } else {
            // For files, try to open the parent directory
            if let Some(parent) = std::path::Path::new(path).parent() {
                let parent_str = parent.to_string_lossy();
                open_that(&parent_str).with_context(|| {
                    format!(
                        "Failed to open parent directory of '{}' in file manager",
                        path
                    )
                })?;
            } else {
                open_that(path)
                    .with_context(|| format!("Failed to open '{}' in file manager", path))?;
            }
        }
    }

    Ok(())
}
