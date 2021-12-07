use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use advent_of_code_2021::CARGO_MANIFEST_DIR;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Resource {
    pub path: PathBuf,
}

impl Resource {
    pub fn new(relative_resource_path: &str) -> Self {
        Resource {
            path: Self::root_path().join(relative_resource_path)
        }
    }

    pub fn root_path() -> PathBuf {
        PathBuf::from(CARGO_MANIFEST_DIR).join("resources")
    }

    pub fn reader(&self) -> BufReader<File> {
        let file = File::open(&self.path).unwrap();
        BufReader::new(file)
    }
}
