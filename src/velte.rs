use std::{collections::HashSet, env, path::PathBuf};
use zed_extension_api::{self as zed, Result};

struct VelteExtension {
    installed: HashSet<String>,
    resolved_package: Option<&'static str>,
}

const PRIMARY_PACKAGE_NAME: &str = "velte-language-server";
const FALLBACK_PACKAGE_NAME: &str = "svelte-language-server";

fn get_package_path(package_name: &str) -> Result<PathBuf> {
    let path = env::current_dir()
        .map_err(|e| e.to_string())?
        .join("node_modules")
        .join(package_name);
    Ok(path)
}

impl VelteExtension {
    fn resolve_package_name(&mut self) -> Result<&'static str> {
        if let Some(package_name) = self.resolved_package {
            return Ok(package_name);
        }

        if zed::npm_package_installed_version(PRIMARY_PACKAGE_NAME)?.is_some() {
            self.resolved_package = Some(PRIMARY_PACKAGE_NAME);
            return Ok(PRIMARY_PACKAGE_NAME);
        }

        if zed::npm_package_installed_version(FALLBACK_PACKAGE_NAME)?.is_some() {
            self.resolved_package = Some(FALLBACK_PACKAGE_NAME);
            return Ok(FALLBACK_PACKAGE_NAME);
        }

        if zed::npm_package_latest_version(PRIMARY_PACKAGE_NAME).is_ok() {
            self.resolved_package = Some(PRIMARY_PACKAGE_NAME);
            return Ok(PRIMARY_PACKAGE_NAME);
        }

        // Fallback for current ecosystem where the Velte package name is not published.
        let _ = zed::npm_package_latest_version(FALLBACK_PACKAGE_NAME)?;
        self.resolved_package = Some(FALLBACK_PACKAGE_NAME);
        Ok(FALLBACK_PACKAGE_NAME)
    }

    fn install_package_if_needed(
        &mut self,
        id: &zed::LanguageServerId,
        package_name: &str,
    ) -> Result<()> {
        let installed_version = zed::npm_package_installed_version(package_name)?;

        if installed_version.is_some() && self.installed.contains(package_name) {
            return Ok(());
        }

        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let latest_version = zed::npm_package_latest_version(package_name)?;

        if installed_version.as_ref() != Some(&latest_version) {
            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            if let Err(error) = zed::npm_install_package(package_name, &latest_version) {
                if installed_version.is_none() {
                    Err(error)?;
                }
            }
        }

        self.installed.insert(package_name.to_string());
        Ok(())
    }
}

impl zed::Extension for VelteExtension {
    fn new() -> Self {
        Self {
            installed: HashSet::new(),
            resolved_package: None,
        }
    }

    fn language_server_command(
        &mut self,
        id: &zed::LanguageServerId,
        _: &zed::Worktree,
    ) -> Result<zed::Command> {
        let package_name = self.resolve_package_name()?;
        self.install_package_if_needed(id, package_name)?;

        let path = get_package_path(package_name)?
            .join("bin/server.js")
            .to_string_lossy()
            .to_string();

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![path, "--stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(VelteExtension);
