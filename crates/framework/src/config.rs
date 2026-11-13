use config::{Config, ConfigError, Environment};
use serde::de::DeserializeOwned;

/// Trait implementation to support any environment variable
/// configuration. All environment variables must be prefixed with
/// 'APP_' and separated by `__`.
///
/// Example:
/// APP_DATABASE__URL
pub trait EnvConfig: Sized + DeserializeOwned {
    fn from_env() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(
                Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__")
                    .try_parsing(true)
            )
            .build()?
            .try_deserialize()
    }
}
