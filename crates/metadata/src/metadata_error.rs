use miette::{self, Diagnostic};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum MetadataError {
    #[diagnostic(code(MetadataError::FileIO), help(""))]
    #[error("file I/O error")]
    FileIO(#[from] std::io::Error),

    #[diagnostic(code(MetadataError::FileNotFound), help(""))]
    #[error("Veryl.toml is not found")]
    FileNotFound,

    #[diagnostic(code(MetadataError::Deserialize), help(""))]
    #[error("toml load failed")]
    Deserialize(#[from] toml::de::Error),

    #[diagnostic(code(MetadataError::Walkdir), help(""))]
    #[error("walkdir error")]
    Walkdir(#[from] walkdir::Error),

    #[diagnostic(code(MetadataError::StripPrefix), help(""))]
    #[error("strip prefix error")]
    StripPrefix(#[from] std::path::StripPrefixError),

    #[diagnostic(code(MetadataError::Git), help(""))]
    #[error("git operation failure: \"{msg}\"\n  {context}")]
    Git { msg: String, context: String },

    #[diagnostic(
        code(MetadataError::InvalidProjectName),
        help("\"[a-zA-Z_][0-9a-zA-Z_]*\" can be used as project name")
    )]
    #[error("project name \"{0}\" is invalid")]
    InvalidProjectName(String),

    #[diagnostic(
        code(MetadataError::InvalidLicense),
        help("license text should follow SPDX expression")
    )]
    #[error("license parse failed")]
    InvalidLicense(#[from] spdx::ParseError),
}
