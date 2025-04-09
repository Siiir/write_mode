use std::{
    io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

/// Specifies different strategies for opening a file in write mode.
///
/// ## Variants
/// - `CreateNew`: Creates a new file, fails if the file already exists.
/// - `UpdateExisting`: Opens an existing file without creating a new one, fails if the file does not exist.
/// - `ClassicWrite`: Opens a file for writing, creating it if it doesn't exist.
#[derive(
    // CRUD-C: Constructors
    Deserialize,
    strum::EnumString,
    Default,
    Clone,
    // CRUD-R: Comparisons
    PartialEq,
    Eq,
    Hash,
    // CRUD-R: Displayers
    Debug,
    strum::IntoStaticStr,
    strum::Display,
    // CRUD-R: Misc
    Serialize,
)]
pub enum WriteMode {
    #[default]
    #[strum(
        serialize = "CreateNew",
        serialize = "Create",
        serialize = "C",
        ascii_case_insensitive
    )]
    CreateNew,

    #[strum(
        serialize = "UpdateExisting",
        serialize = "Update",
        serialize = "U",
        ascii_case_insensitive
    )]
    UpdateExisting,

    #[strum(
        serialize = "ClassicWrite",
        serialize = "Write",
        serialize = "W",
        ascii_case_insensitive
    )]
    ClassicWrite,

    #[cfg(feature = "append")]
    #[strum(
        serialize = "ClassicAppend",
        serialize = "Append",
        serialize = "A",
        ascii_case_insensitive
    )]
    ClassicAppend,

    #[cfg(feature = "append")]
    #[strum(
        serialize = "AppendToExisting",
        serialize = "Extend",
        serialize = "E",
        ascii_case_insensitive
    )]
    AppendToExisting,
}

impl WriteMode {
    /// Opens a file at the given path using `fs_err::File` with the current write mode.
    ///
    /// # Arguments
    /// * `path` - file path from which the file will be opened.
    ///
    /// # Errors
    /// Returns an `io::Error` if the file can't be opened with the specified options.
    pub fn fe_open<P>(&self, path: P) -> io::Result<fs_err::File>
    where
        P: Into<PathBuf>,
    {
        self.fe_open_options().open(path)
    }

    /// Opens a file at the given path using `std::fs::File` with the current write mode.
    ///
    /// # Arguments
    /// * `path` - file path from which the file will be opened.
    ///
    /// # Errors
    /// Returns an `io::Error` if the file can't be opened with the specified options.
    pub fn std_open<P>(&self, path: P) -> io::Result<std::fs::File>
    where
        P: AsRef<Path>,
    {
        self.std_open_options().open(path)
    }

    /// Returns an `fs_err::OpenOptions` object configured accordingly to the selected `WriteMode`.
    pub fn fe_open_options(&self) -> fs_err::OpenOptions {
        fs_err::OpenOptions::from_options(self.std_open_options())
    }

    /// Returns a `std::fs::OpenOptions` object configured accordingly to the selected `WriteMode`
    pub fn std_open_options(&self) -> std::fs::OpenOptions {
        let mut options = std::fs::File::options();

        options.write(true).read(false);
        match self {
            Self::CreateNew => {
                options.create_new(true).create(true);
            }
            Self::UpdateExisting => {
                options.create_new(false).create(false);
            }
            Self::ClassicWrite => {
                options.create_new(false).create(true);
            }
            #[cfg(feature = "append")]
            Self::ClassicAppend => {
                options.create_new(false).create(true).append(true);
            }
            #[cfg(feature = "append")]
            Self::AppendToExisting => {
                options.create_new(false).create(false).append(true);
            }
        }

        options
    }
}
