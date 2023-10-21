use crate::model::Index;
use anyhow::{anyhow, Error};
use elasticsearch::Elasticsearch;
use std::fs;
use std::path::PathBuf;

struct VelcroStrap {
    es: Elasticsearch,
}

struct IndexFile {
    index: Index,
    index_name: String,
    path: PathBuf,
}

impl TryFrom<PathBuf> for IndexFile {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self, Error> {
        let maybe_index_name = path.file_stem().map(|s| s.to_string_lossy().to_string());
        let index_name = match maybe_index_name {
            None => return Err(anyhow!("asdf")),
            Some(index_name) => index_name,
        };
        let index = serde_json::from_slice::<Index>(fs::read(&path)?.as_slice())?;
        Ok(IndexFile {
            index_name,
            path,
            index,
        })
    }
}

pub(crate) fn velcro_strap() {
    let read_dir = fs::read_dir("indices").unwrap();
    for dir_entry_result in read_dir {
        let dir_entry = dir_entry_result.unwrap();
        let path = dir_entry.path();
        let extension = path.extension();
        if extension.is_none() || extension.map_or(false, |ext| ext == "json") {
            continue;
        }
        let index_name = dir_entry
            .path()
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();
        println!("{}", index_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velcro_strap() {
        velcro_strap();
    }
}
