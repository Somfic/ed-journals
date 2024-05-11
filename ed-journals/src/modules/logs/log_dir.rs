use std::path::PathBuf;

use thiserror::Error;

use crate::modules::logs::{LogFile, LogFileError};

/// Provides an abstraction for all the log files in the journal directory.
#[derive(Debug)]
pub struct LogDir {
    dir_path: PathBuf,
}

#[derive(Debug, Error)]
pub enum LogDirError {
    #[error("Log path is not a directory")]
    PathIsNotADirectory,

    #[error("Failed to represent OS string")]
    FailedToRepresentOsString,

    #[error(transparent)]
    JournalFileError(#[from] LogFileError),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

impl LogDir {
    pub fn new(dir_path: PathBuf) -> Result<LogDir, LogDirError> {
        if !dir_path.is_dir() {
            return Err(LogDirError::PathIsNotADirectory);
        }

        Ok(LogDir { dir_path })
    }
}

impl LogDir {
    /// Returns a list of journal log files found in the directory in any order.
    pub fn journal_logs(&self) -> Result<Vec<LogFile>, LogDirError> {
        self.dir_path
            .read_dir()?
            .filter_map(|entry| match entry {
                Ok(entry) => match LogFile::try_from(entry) {
                    Ok(journal_file) => Some(Ok(journal_file)),
                    Err(LogFileError::IncorrectFileName) => None,
                    Err(err) => Some(Err(err.into())),
                },
                Err(err) => Some(Err(err.into())),
            })
            .collect::<Result<Vec<LogFile>, LogDirError>>()
    }

    /// Returns a list of journal log files found in the directory, returning the oldest journal
    /// logs first.
    pub fn journal_logs_oldest_first(&self) -> Result<Vec<LogFile>, LogDirError> {
        let mut logs = self.journal_logs()?;
        logs.sort();

        Ok(logs)
    }

    /// Returns a list of journal log files found in the directory, returning the newest journal
    /// logs first.
    pub fn journal_logs_newest_first(&self) -> Result<Vec<LogFile>, LogDirError> {
        let mut logs = self.journal_logs()?;
        logs.sort();
        logs.reverse();

        Ok(logs)
    }
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use crate::logs::LogDir;

    #[test]
    fn journal_files_oldest_first_are_returned_in_the_correct_order() {
        let dir_path = current_dir()
            .unwrap()
            .parent()
            .unwrap()
            .join("test-journals");

        let journal_dir = LogDir::new(dir_path).unwrap();

        let mut journal_files = journal_dir.journal_logs_oldest_first().unwrap().into_iter();

        let journal_1 = journal_files.next().unwrap();
        let journal_2 = journal_files.next().unwrap();

        let date_1 = journal_1.date_time();
        let date_2 = journal_2.date_time();

        assert!(date_2 > date_1);
    }

    #[test]
    fn journal_files_newest_first_are_returned_in_the_correct_order() {
        let dir_path = current_dir()
            .unwrap()
            .parent()
            .unwrap()
            .join("test-journals");

        let journal_dir = LogDir::new(dir_path).unwrap();

        let mut journal_files = journal_dir.journal_logs_newest_first().unwrap().into_iter();

        let journal_1 = journal_files.next().unwrap();
        let journal_2 = journal_files.next().unwrap();

        let date_1 = journal_1.date_time();
        let date_2 = journal_2.date_time();

        assert!(date_2 < date_1);
    }
}