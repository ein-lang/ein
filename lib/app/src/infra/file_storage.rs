use super::file_path::FilePath;
use super::repository::Repository;

pub trait FileStorage {
    fn exists(&self, path: &FilePath) -> bool;
    fn glob(
        &self,
        file_extension: &str,
        excluded_directories: &[&FilePath],
    ) -> Result<Vec<FilePath>, Box<dyn std::error::Error>>;
    fn read_repository(
        &self,
        directory_path: &FilePath,
    ) -> Result<Repository, Box<dyn std::error::Error>>;
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

    fn glob(
        &self,
        file_extension: &str,
        excluded_directories: &[&FilePath],
    ) -> Result<Vec<FilePath>, Box<dyn std::error::Error>> {
        let mut paths = self
            .files
            .lock()
            .unwrap()
            .keys()
            .filter(|path| {
                path.has_extension(file_extension)
                    && !excluded_directories
                        .iter()
                        .any(|directory| path.has_prefix(&directory))
            })
            .cloned()
            .collect::<Vec<FilePath>>();

        paths.sort();

        Ok(paths)
    }

    fn read_repository(
        &self,
        directory_path: &FilePath,
    ) -> Result<Repository, Box<dyn std::error::Error>> {
        Ok(Repository::new(
            url::Url::parse(&format!("{}", directory_path))?,
            "v1",
        ))
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
            FileStorageFake::new(Default::default())
                .glob("c", &[])
                .unwrap(),
            vec![]
        );
        assert_eq!(
            FileStorageFake::new(vec![(FilePath::new(&["foo"]), vec![])].drain(..).collect())
                .glob("c", &[])
                .unwrap(),
            vec![]
        );
        assert_eq!(
            FileStorageFake::new(
                vec![(FilePath::new(&["foo.c"]), vec![])]
                    .drain(..)
                    .collect()
            )
            .glob("c", &[])
            .unwrap(),
            vec![FilePath::new(&["foo.c"])]
        );
        assert_eq!(
            FileStorageFake::new(vec![(FilePath::new(&["foo"]), vec![])].drain(..).collect())
                .glob("", &[])
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
            .glob("bar", &[])
            .unwrap(),
            vec![FilePath::new(&["foo.bar"])]
        );
        assert_eq!(
            FileStorageFake::new(
                vec![
                    (FilePath::new(&["foo.bar"]), vec![]),
                    (FilePath::new(&["baz", "blah.bar"]), vec![])
                ]
                .drain(..)
                .collect()
            )
            .glob("bar", &[])
            .unwrap(),
            vec![
                FilePath::new(&["baz", "blah.bar"]),
                FilePath::new(&["foo.bar"]),
            ]
        );
        assert_eq!(
            FileStorageFake::new(
                vec![(FilePath::new(&["foo", "bar.baz"]), vec![])]
                    .drain(..)
                    .collect()
            )
            .glob("baz", &[&FilePath::new(&["foo"])])
            .unwrap(),
            vec![]
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
