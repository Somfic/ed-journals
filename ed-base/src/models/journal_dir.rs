use crate::models::journal_file::{JournalFile, JournalFileError};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EDLogDirError {
    #[error("Elite dangerous log path is not a directory")]
    PathIsNotADirectory,

    #[error("Failed to represent OS string")]
    FailedToRepresentOsString,

    #[error(transparent)]
    JournalFileError(#[from] JournalFileError),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

pub struct EDLogDir {
    dir_path: PathBuf,
}

impl TryFrom<PathBuf> for EDLogDir {
    type Error = EDLogDirError;

    fn try_from(dir_path: PathBuf) -> Result<Self, Self::Error> {
        if !dir_path.is_dir() {
            return Err(EDLogDirError::PathIsNotADirectory);
        }

        Ok(EDLogDir { dir_path })
    }
}

impl EDLogDir {
    pub fn journal_logs(&self) -> Result<Vec<JournalFile>, EDLogDirError> {
        self.dir_path
            .read_dir()?
            .filter_map(|entry| match entry {
                Ok(entry) => match JournalFile::try_from(entry) {
                    Ok(journal_file) => Some(Ok(journal_file)),
                    Err(JournalFileError::IncorrectFileName) => None,
                    Err(err) => Some(Err(err.into())),
                },
                Err(err) => Some(Err(err.into())),
            })
            .collect::<Result<Vec<JournalFile>, EDLogDirError>>()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn journal_files_are_returned_in_the_correct_order() {

    }
}