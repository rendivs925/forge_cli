use std::fs;
use std::io;
use std::path::PathBuf;

use crate::analysis::Analysis;

/// Errors from the persisted analysis store.
#[derive(Debug)]
pub enum StoreError {
    Io { path: PathBuf, source: io::Error },
    Serialize { message: String },
    Deserialize { path: PathBuf, message: String },
    NotFound { path: PathBuf },
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(f, "could not access '{}': {source}", path.display())
            }
            Self::Serialize { message } => write!(f, "could not serialize analysis: {message}"),
            Self::Deserialize { path, message } => {
                write!(
                    f,
                    "could not read analysis from '{}': {message}",
                    path.display()
                )
            }
            Self::NotFound { path } => write!(f, "no analysis result at '{}'", path.display()),
        }
    }
}

impl std::error::Error for StoreError {}

/// Persists the latest analysis result under `<workspace>/.forge/analysis/`.
pub struct AnalysisStore {
    workspace: PathBuf,
}

impl AnalysisStore {
    pub fn new(workspace: PathBuf) -> Self {
        Self { workspace }
    }

    pub fn path(&self) -> PathBuf {
        self.workspace
            .join(".forge")
            .join("analysis")
            .join("latest.json")
    }

    pub fn save(&self, analysis: &Analysis) -> Result<(), StoreError> {
        let path = self.path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|source| StoreError::Io {
                path: parent.to_path_buf(),
                source,
            })?;
        }
        let json =
            serde_json::to_string_pretty(analysis).map_err(|error| StoreError::Serialize {
                message: error.to_string(),
            })?;
        fs::write(&path, json).map_err(|source| StoreError::Io {
            path: path.clone(),
            source,
        })
    }

    pub fn load(&self) -> Result<Analysis, StoreError> {
        let path = self.path();
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Err(StoreError::NotFound { path });
            }
            Err(source) => {
                return Err(StoreError::Io { path, source });
            }
        };
        serde_json::from_str(&content).map_err(|error| StoreError::Deserialize {
            path,
            message: error.to_string(),
        })
    }

    pub fn exists(&self) -> bool {
        self.path().is_file()
    }
}

#[cfg(test)]
mod tests {
    use super::AnalysisStore;
    use crate::analysis::Analysis;
    use crate::finding::{Category, Finding, Location, Severity};

    #[test]
    fn saves_and_loads_analysis() {
        let dir = std::env::temp_dir().join(format!("forge-store-test-{}", std::process::id()));
        let store = AnalysisStore::new(dir);
        let analysis = Analysis::new(
            vec![Finding::new(
                "demo",
                "demo.rule",
                Severity::Major,
                Category::Maintainability,
                Location {
                    file: "f.rs".to_string(),
                    start_line: Some(1),
                    start_column: None,
                    end_line: None,
                    end_column: None,
                },
                "message",
                None,
            )],
            Vec::new(),
        );
        store.save(&analysis).unwrap();
        assert!(store.exists());
        let loaded = store.load().unwrap();
        assert_eq!(loaded, analysis);
    }

    #[test]
    fn missing_store_reports_not_found() {
        let dir = std::env::temp_dir().join(format!("forge-store-missing-{}", std::process::id()));
        let store = AnalysisStore::new(dir);
        assert!(!store.exists());
        assert!(store.load().is_err());
    }
}
