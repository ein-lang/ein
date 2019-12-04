use super::file_path::FilePath;

pub trait FileStorage {
    fn exists(&self, path: &FilePath) -> bool;
    fn glob(&self, pattern: &str) -> Result<Vec<FilePath>, Box<dyn std::error::Error>>;
    fn read_to_string(&self, path: &FilePath) -> Result<String, Box<dyn std::error::Error>>;
    fn read_to_vec(&self, path: &FilePath) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn write(&self, path: &FilePath, data: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
}

#[cfg(test)]
pub struct FileStorageFake {
    files: std::sync::Mutex<std::collections::HashMap<FilePath, Vec<u8>>>,
}

#[cfg(test)]
impl FileStorageFake {
    pub fn new(files: std::collections::HashMap<FilePath, Vec<u8>>) -> Self {
        Self {
            files: files.into(),
        }
    }
}

#[cfg(test)]
impl FileStorage for FileStorageFake {
    fn exists(&self, path: &FilePath) -> bool {
        self.files.lock().unwrap().contains_key(path)
    }

    // TODO Interpret patterns more strictly.
    fn glob(&self, pattern: &str) -> Result<Vec<FilePath>, Box<dyn std::error::Error>> {
        let pattern = regex::Regex::new(&format!(
            "^{}$",
            regex::Regex::new(r"\*\*/\*")?.replace(pattern, ".*")
        ))?;

        let mut paths = self
            .files
            .lock()
            .unwrap()
            .keys()
            .filter(|path| pattern.is_match(&format!("{}", path)))
            .cloned()
            .collect::<Vec<FilePath>>();

        paths.sort();

        Ok(paths)
    }

    fn read_to_string(&self, path: &FilePath) -> Result<String, Box<dyn std::error::Error>> {
        Ok(String::from_utf8(
            self.files
                .lock()
                .unwrap()
                .get(path)
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, ""))?
                .clone(),
        )?)
    }

    fn read_to_vec(&self, path: &FilePath) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(self
            .files
            .lock()
            .unwrap()
            .get(path)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, ""))?
            .clone())
    }

    fn write(&self, path: &FilePath, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.files.lock().unwrap().insert(path.clone(), data.into());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exists() {
        assert!(
            FileStorageFake::new(vec![(FilePath::new(&["foo"]), vec![])].drain(..).collect())
                .exists(&FilePath::new(&["foo"]))
        );
        assert!(!FileStorageFake::new(Default::default()).exists(&FilePath::new(&["foo"])));
    }

    #[test]
    fn glob() {
        assert_eq!(
            FileStorageFake::new(Default::default()).glob("").unwrap(),
            vec![]
        );
        assert_eq!(
            FileStorageFake::new(vec![(FilePath::new(&["foo"]), vec![])].drain(..).collect())
                .glob("")
                .unwrap(),
            vec![]
        );
        assert_eq!(
            FileStorageFake::new(vec![(FilePath::new(&["foo"]), vec![])].drain(..).collect())
                .glob("foo")
                .unwrap(),
            vec![FilePath::new(&["foo"])]
        );
        assert_eq!(
            FileStorageFake::new(vec![(FilePath::new(&["foo"]), vec![])].drain(..).collect())
                .glob("**/*")
                .unwrap(),
            vec![FilePath::new(&["foo"])]
        );
        assert_eq!(
            FileStorageFake::new(
                vec![
                    (FilePath::new(&["foo.bar"]), vec![]),
                    (FilePath::new(&["foo.baz"]), vec![])
                ]
                .drain(..)
                .collect()
            )
            .glob("**/*.bar")
            .unwrap(),
            vec![FilePath::new(&["foo.bar"])]
        );
        assert_eq!(
            FileStorageFake::new(
                vec![
                    (FilePath::new(&["foo.bar"]), vec![]),
                    (FilePath::new(&["baz/blah.bar"]), vec![])
                ]
                .drain(..)
                .collect()
            )
            .glob("**/*.bar")
            .unwrap(),
            vec![
                FilePath::new(&["baz/blah.bar"]),
                FilePath::new(&["foo.bar"]),
            ]
        );
    }

    #[test]
    fn read_to_string() {
        assert_eq!(
            FileStorageFake::new(vec![(FilePath::new(&["foo"]), vec![])].drain(..).collect())
                .read_to_string(&FilePath::new(&["foo"]))
                .unwrap(),
            ""
        );
        assert!(FileStorageFake::new(Default::default())
            .read_to_string(&FilePath::new(&["foo"]))
            .is_err());
    }

    #[test]
    fn read_to_vec() {
        assert_eq!(
            FileStorageFake::new(vec![(FilePath::new(&["foo"]), vec![])].drain(..).collect())
                .read_to_vec(&FilePath::new(&["foo"]))
                .unwrap(),
            Vec::<u8>::new()
        );
        assert!(FileStorageFake::new(Default::default())
            .read_to_vec(&FilePath::new(&["foo"]))
            .is_err());
    }

    #[test]
    fn write() {
        let file_storage = FileStorageFake::new(Default::default());

        file_storage.write(&FilePath::new(&["foo"]), &[]).unwrap();
        file_storage
            .read_to_string(&FilePath::new(&["foo"]))
            .unwrap();

        FileStorageFake::new(vec![(FilePath::new(&["foo"]), vec![])].drain(..).collect())
            .write(&FilePath::new(&["foo"]), &[])
            .unwrap();
    }
}
