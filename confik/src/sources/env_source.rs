use std::error::Error;

use envious::Config;

use crate::{ConfigurationBuilder, Source};

/// A [`Source`] referring to environment variables.
///
/// Uses the [envious](https://docs.rs/envious) crate for interpreting env vars.
///
/// # Examples
///
/// ```
/// use confik::{ConfigBuilder, Configuration, EnvSource};
///
/// #[derive(Configuration)]
/// struct Config {
///     port: u16,
/// }
///
/// std::env::set_var("PORT", "1234");
///
/// let config = ConfigBuilder::<Config>::default()
///     .override_with(EnvSource::new())
///     .try_build()
///     .unwrap();
///
/// assert_eq!(config.port, 1234);
/// ```
///
/// # Secrets
///
/// Secrets are allowed.
#[derive(Debug, Clone)]
pub struct EnvSource<'a> {
    config: Config<'a>,
    allow_secrets: bool,
}

impl<'a> Default for EnvSource<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> EnvSource<'a> {
    /// Creates a new [`Source`] referring to environment variables.
    pub fn new() -> Self {
        Self {
            config: Config::new(),
            allow_secrets: false,
        }
    }

    /// Sets the envious prefix.
    ///
    /// See [`Config::with_prefix`].
    pub fn with_prefix(mut self, prefix: &'a str) -> Self {
        self.config.with_prefix(prefix);
        self
    }

    /// Sets the envious separator.
    ///
    /// See [`Config::with_separator`].
    pub fn with_separator(mut self, separator: &'a str) -> Self {
        self.config.with_separator(separator);
        self
    }

    /// Sets the envious config.
    pub fn with_config(mut self, config: Config<'a>) -> Self {
        self.config = config;
        self
    }

    /// Allows this source to contain secrets.
    pub fn allow_secrets(mut self) -> Self {
        self.allow_secrets = true;
        self
    }
}

impl<'a> Source for EnvSource<'a> {
    fn allows_secrets(&self) -> bool {
        self.allow_secrets
    }

    fn provide<T: ConfigurationBuilder>(&self) -> Result<T, Box<dyn Error + Sync + Send>> {
        Ok(self.config.build_from_env()?)
    }
}
