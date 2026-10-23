//! Runtime configuration: the `Config` tree, its source-code defaults and
//! file loading.
//!
//! Defaults live in the modules under `config/`. On first run they are written
//! to the config file so users have something to edit; afterwards the file is
//! loaded on top of the defaults, so missing keys keep their default value.

mod borders;
mod icons;
mod strings;
mod theme;
mod titles;

pub(crate) use borders::{BordersConfig, PaneBorderConfig};
pub(crate) use icons::IconsConfig;
pub(crate) use strings::StringsConfig;
pub(crate) use theme::Theme;
pub(crate) use titles::TitlesConfig;

use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use home::home_dir;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct Config {
    pub(crate) show_parent: bool,
    pub(crate) reverse_columns: bool,
    pub(crate) reverse_rows: bool,
    pub(crate) list_cycle: bool,
    /// Name of the active theme: a user theme from `themes`, or a built-in one.
    pub(crate) theme: String,
    /// User-defined themes, merged over the terminal palette.
    pub(crate) themes: Vec<Theme>,
    pub(crate) titles: TitlesConfig,
    pub(crate) strings: StringsConfig,
    pub(crate) borders: BordersConfig,
    pub(crate) icons: IconsConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_parent: true,
            reverse_columns: false,
            reverse_rows: false,
            list_cycle: true,
            theme: "terminal".to_string(),
            themes: Vec::new(),
            titles: TitlesConfig::default(),
            strings: StringsConfig::default(),
            borders: BordersConfig::default(),
            icons: IconsConfig::default(),
        }
    }
}

impl Config {
    /// Load the config: `--config` when given, otherwise the default path,
    /// which is created with the built-in defaults on first run.
    ///
    /// Invalid or unreadable files are reported on stderr and fall back to the
    /// built-in defaults so the app always starts.
    pub(crate) fn load_or_default(path: Option<&Path>) -> Self {
        if let Some(path) = path {
            return Self::load_file(path, true);
        }
        let Some(path) = Self::default_config_path() else {
            return Self::default();
        };
        if !path.exists() {
            Self::write_default(&path);
        }
        Self::load_file(&path, false)
    }

    fn load_file(path: &Path, explicit: bool) -> Self {
        match fs::read_to_string(path) {
            Ok(content) => match Self::from_toml(&content) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("zap: invalid config {}: {e}", path.display());
                    Self::default()
                }
            },
            Err(e) if e.kind() == ErrorKind::NotFound => {
                if explicit {
                    eprintln!("zap: config not found: {}", path.display());
                }
                Self::default()
            }
            Err(e) => {
                eprintln!("zap: cannot read config {}: {e}", path.display());
                Self::default()
            }
        }
    }

    /// Write the built-in defaults so there is a config file to edit.
    fn write_default(path: &Path) {
        let content = match toml::to_string_pretty(&Self::default()) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("zap: cannot serialize default config: {e}");
                return;
            }
        };
        if let Some(dir) = path.parent()
            && let Err(e) = fs::create_dir_all(dir)
        {
            eprintln!("zap: cannot create config dir {}: {e}", dir.display());
            return;
        }
        if let Err(e) = fs::write(path, content) {
            eprintln!("zap: cannot write default config {}: {e}", path.display());
        }
    }

    /// Parse a config file on top of the built-in defaults, so partial configs
    /// only need to declare the keys they change.
    fn from_toml(content: &str) -> Result<Self, toml::de::Error> {
        let mut merged = toml::Value::try_from(Self::default()).expect("Config is serializable");
        merge(&mut merged, toml::from_str(content)?);
        merged.try_into()
    }

    /// Resolve `theme` to a palette, preferring user themes then built-ins.
    pub(crate) fn resolve_theme(&self) -> Theme {
        if let Some(theme) = self.themes.iter().find(|theme| theme.name == self.theme) {
            return theme.clone();
        }
        Theme::builtin(&self.theme).unwrap_or_else(|| {
            eprintln!(
                "zap: unknown theme `{}`, using terminal (available: {})",
                self.theme,
                Theme::builtin_names().join(", ")
            );
            Theme::default()
        })
    }

    /// `$XDG_CONFIG_HOME/zap/config.toml`, or `~/.config/zap/config.toml`.
    fn default_config_path() -> Option<PathBuf> {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| home_dir().map(|home| home.join(".config")))
            .map(|dir| dir.join("zap").join("config.toml"))
    }
}

/// Recursively overlay `patch` onto `base`, keeping keys only present in `base`.
fn merge(base: &mut toml::Value, patch: toml::Value) {
    match (base, patch) {
        (toml::Value::Table(base), toml::Value::Table(patch)) => {
            for (key, value) in patch {
                match base.get_mut(&key) {
                    Some(slot) => merge(slot, value),
                    None => {
                        base.insert(key, value);
                    }
                }
            }
        }
        (base, patch) => *base = patch,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_config_keeps_defaults() {
        let config = Config::from_toml("reverse_columns = true\ntheme = \"nord\"\n").unwrap();
        assert!(config.reverse_columns);
        assert!(config.show_parent);
        assert_eq!(config.theme, "nord");
        assert!(!config.reverse_rows);
    }

    #[test]
    fn nested_table_merge_keeps_siblings() {
        let config = Config::from_toml("[borders.current]\nkind = \"rounded\"\n").unwrap();
        assert_eq!(config.borders.current.kind, "rounded");
        let defaults = Config::default();
        assert_eq!(config.borders.current.sides, defaults.borders.current.sides);
        assert_eq!(config.borders.parent.sides, defaults.borders.parent.sides);
    }

    #[test]
    fn invalid_values_are_errors() {
        assert!(Config::from_toml("reverse_columns = tru").is_err());
        assert!(Config::from_toml("reverse_columns = \"yes\"").is_err());
    }

    #[test]
    fn unknown_keys_are_ignored() {
        assert!(Config::from_toml("nope = 1").is_ok());
    }

    #[test]
    fn user_theme_overrides_only_declared_colours() {
        let config = Config::from_toml(
            "theme = \"mine\"\n[[themes]]\nname = \"mine\"\naccent = \"#ff0000\"\n",
        )
        .unwrap();
        let theme = config.resolve_theme();
        assert_eq!(theme.name, "mine");
        assert_eq!(theme.accent, ratatui::style::Color::Rgb(255, 0, 0));
        assert_eq!(theme.text, Theme::default().text);
    }

    #[test]
    fn invalid_theme_colour_is_an_error() {
        assert!(Config::from_toml("[[themes]]\nname = \"mine\"\naccent = \"nope\"\n").is_err());
    }

    #[test]
    fn unknown_theme_falls_back_to_terminal() {
        let config = Config::from_toml("theme = \"nope\"\n").unwrap();
        assert_eq!(config.resolve_theme().name, "terminal");
    }

    #[test]
    fn builtin_themes_are_resolvable() {
        let config = Config::from_toml("theme = \"dracula\"\n").unwrap();
        assert_eq!(config.resolve_theme().name, "dracula");
    }

    #[test]
    fn source_defaults_are_complete() {
        let config = Config::default();
        assert_eq!(config.theme, "terminal");
        assert_eq!(config.icons.dir, "\u{f115}");
        assert_eq!(config.icons.by_extension.len(), 105);
        assert_eq!(config.icons.by_name.len(), 10);
        assert_eq!(config.icons.by_extension["rs"], "\u{e7a8}");
    }

    #[test]
    fn default_config_round_trips_through_toml() {
        let toml = toml::to_string_pretty(&Config::default()).unwrap();
        let parsed = Config::from_toml(&toml).unwrap();
        assert_eq!(parsed.theme, "terminal");
        assert!(parsed.show_parent);
        assert_eq!(parsed.icons.by_extension.len(), 105);
    }

    #[test]
    fn writes_default_config_file() {
        let path = std::env::temp_dir().join(format!("zap-config-{}.toml", std::process::id()));
        let _ = std::fs::remove_file(&path);
        Config::write_default(&path);
        let content = std::fs::read_to_string(&path).unwrap();
        std::fs::remove_file(&path).unwrap();
        assert!(content.contains("theme = \"terminal\""));
        assert!(content.contains("[icons.by_extension]"));
    }

    #[test]
    fn invalid_config_file_falls_back_to_defaults() {
        let path = std::env::temp_dir().join(format!("zap-bad-{}.toml", std::process::id()));
        std::fs::write(&path, "reverse_columns = \"yes\"").unwrap();
        let config = Config::load_file(&path, false);
        std::fs::remove_file(&path).unwrap();
        assert!(config.show_parent);
        assert!(!config.reverse_columns);
    }
}
