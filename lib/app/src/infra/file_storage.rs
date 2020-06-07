use super::file_path::FilePath;
use super::repository::Repository;

pub trait FileStorage {
    fn exists(&self, path: &FilePath) -> bool;
    fn is_directory(&self, path: &FilePath) -> bool;
    fn read_directory(&self, path: &FilePath) -> Result<Vec<FilePath>, Box<dyn std::error::Error>>;
    fn read_repository(
        &self,
        directory_path: &FilePath,
    ) -> Result<Option<Repository>, Box<dyn std::error::Error>>;
    fn read_to_string(&self, path: &FilePath) -> Result<String, Box<dyn std::error::Error>>;
    fn read_to_vec(&self, path: &FilePath) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn write(&self, path: &FilePath, data: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
}

#[cfg(test)]
pub struct FakeFileStorage {
    files: std::sync::Mutex<std::collections::HashMap<FilePath, Vec<u8>>>,
}

#[cfg(test)]
impl FakeFileStorage {
    pub fn new(files: std::collections::HashMap<FilePath, Vec<u8>>) -> Self {
        Self {
            files: files.into(),
        }
    }
}

#[cfg(test)]
impl FileStorage for FakeFileStorage {
    fn exists(&self, path: &FilePath) -> bool {
        self.files.lock().unwrap().contains_key(path)
    }

    fn is_directory(&self, _: &FilePath) -> bool {
        unimplemented!()
    }

    fn read_directory(&self, _: &FilePath) -> Result<Vec<FilePath>, Box<dyn std::error::Error>> {
        unimplemented!()
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
            FakeFileStorage::new(vec![(FilePath::new(&["foo"]), vec![])].drain(..).collect())
                .exists(&FilePath::new(&["foo"]))
        );
        assert!(!FakeFileStorage::new(Default::default()).exists(&FilePath::new(&["foo"])));
    }

    #[test]
    fn read_to_string() {
        assert_eq!(
            FakeFileStorage::new(vec![(FilePath::new(&["foo"]), vec![])].drain(..).collect())
                .read_to_string(&FilePath::new(&["foo"]))
                .unwrap(),
            ""
        );
        assert!(FakeFileStorage::new(Default::default())
            .read_to_string(&FilePath::new(&["foo"]))
            .is_err());
    }

    #[test]
    fn read_to_vec() {
        assert_eq!(
            FakeFileStorage::new(vec![(FilePath::new(&["foo"]), vec![])].drain(..).collect())
                .read_to_vec(&FilePath::new(&["foo"]))
                .unwrap(),
            Vec::<u8>::new()
        );
        assert!(FakeFileStorage::new(Default::default())
            .read_to_vec(&FilePath::new(&["foo"]))
            .is_err());
    }

    #[test]
    fn write() {
        let file_storage = FakeFileStorage::new(Default::default());

        file_storage.write(&FilePath::new(&["foo"]), &[]).unwrap();
        file_storage
            .read_to_string(&FilePath::new(&["foo"]))
            .unwrap();

        FakeFileStorage::new(vec![(FilePath::new(&["foo"]), vec![])].drain(..).collect())
            .write(&FilePath::new(&["foo"]), &[])
            .unwrap();
    }
}
