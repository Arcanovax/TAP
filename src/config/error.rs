use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ConfigError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Parse {
        path: PathBuf,
        source: serde_yaml::Error,
    },
    Conflict {
        id: String,
        file_a: PathBuf,
        file_b: PathBuf,
    },
    DanglingRef {
        from_id: String,
        missing_ref: String,
    },
    ScopeViolation {
        from_id: String,
        ref_id: String,
        defined_in: PathBuf,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io { path, source } => {
                write!(f, "failed to read {}: {source}", path.display())
            }
            ConfigError::Parse { path, source } => {
                write!(f, "invalid YAML in {}: {source}", path.display())
            }
            ConfigError::Conflict { id, file_a, file_b } => write!(
                f,
                "duplicate id `{id}` defined in both {} and {}",
                file_a.display(),
                file_b.display()
            ),
            ConfigError::DanglingRef { from_id, missing_ref } => write!(
                f,
                "`{from_id}` references `{missing_ref}`, which does not exist"
            ),
            ConfigError::ScopeViolation { from_id, ref_id, defined_in } => write!(
                f,
                "`{from_id}` uses `{ref_id}` (defined in {}) without importing that file",
                defined_in.display()
            ),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io { source, .. } => Some(source),
            ConfigError::Parse { source, .. } => Some(source),
            _ => None,
        }
    }
}
