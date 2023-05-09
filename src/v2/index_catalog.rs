use std::clone::Clone;
use std::fs;
use std::path::{PathBuf, MAIN_SEPARATOR};

use dashmap::DashMap;
use tantivy::schema::Schema;
use tantivy::Index;

use lily_types::{Catalog, Error};

use super::LocalIndex;
use super::Settings;
use crate::Result;

pub struct IndexCatalog {
    settings: Settings,
    base_path: PathBuf,
    local_handles: DashMap<String, LocalIndex>,
}

impl IndexCatalog {
    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }
}

#[async_trait::async_trait]
impl Catalog for IndexCatalog {
    type Handle = LocalIndex;

    fn base_path(&self) -> String {
        format!("{}", self.base_path.display())
    }

    fn get_collection(&self) -> &DashMap<String, Self::Handle> {
        &self.local_handles
    }

    async fn add_index(&self, name: &str, schema: Schema) -> Result<()> {
        let handle = LocalIndex::new(
            self.base_path.clone(),
            name,
            schema,
            self.settings.writer_memory,
            self.settings.get_merge_policy(),
        )?;
        self.local_handles.insert(name.to_string(), handle);
        Ok(())
    }

    async fn delete_index(&self, name: &str) -> Result<()> {
        let mut base_path = self.base_path();
        base_path.push_str("/");
        base_path.push_str(name);
        let base_path = PathBuf::from(&base_path);
        if !base_path.exists() {
            return Err(Error::QueryError("Base path does not exist".into()));
        }
        self.local_handles.remove(name);
        Ok(std::fs::remove_dir_all(base_path)?)
    }

    async fn list_indexes(&self) -> Vec<String> {
        let mut local_keys: Vec<String> = self.local_handles.iter().map(|e| e.key().to_owned()).collect();
        local_keys.sort();
        local_keys.dedup();
        local_keys
    }

    fn get_index(&self, name: &str) -> Result<Self::Handle> {
        self.local_handles.get(name).map(|r| r.value().to_owned()).ok_or_else(|| {
            let _ = &name;
            Error::UnknownIndex(name.into())
        })
    }

    fn exists(&self, index: &str) -> bool {
        self.get_collection().contains_key(index)
    }
}

impl IndexCatalog {
    pub fn new(settings: Settings) -> Result<Self> {
        let local_idxs = DashMap::new();
        let path = PathBuf::from(&settings.path);
        let index_cat = IndexCatalog {
            settings,
            base_path: path,
            local_handles: local_idxs,
        };

        Ok(index_cat)
    }

    pub fn load_index(path: &str) -> Result<Index> {
        let p = PathBuf::from(path);
        if p.exists() {
            Index::open_in_dir(&p).map_err(|_| Error::UnknownIndex(p.display().to_string()))
        } else {
            Err(Error::UnknownIndex(path.to_string()))
        }
    }

    pub fn get_mut_collection(&mut self) -> &mut DashMap<String, LocalIndex> {
        &mut self.local_handles
    }

    pub async fn refresh_catalog(&mut self) -> Result<()> {
        self.local_handles.clear();

        for dir in fs::read_dir(self.base_path.clone())? {
            let entry = dir?.path();
            if let Some(entry_str) = entry.to_str() {
                if entry.exists() {
                    if !entry_str.ends_with(".node_id") {
                        let pth: String = entry_str.rsplit(MAIN_SEPARATOR).take(1).collect();
                        log::debug!("Loading Path: {} - {}", pth, entry_str);

                        let idx = IndexCatalog::load_index(entry_str)?;
                        self.add_index(&pth, idx.schema()).await?;
                    }
                } else {
                    return Err(Error::UnknownIndex(format!("Path {}", entry.display())));
                }
            } else {
                return Err(Error::UnknownIndex(format!("Path {} is not a valid unicode path", entry.display())));
            }
        }
        Ok(())
    }

    pub async fn clear(&self) {
        self.local_handles.clear();
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    pub fn from_index(name: String, index: Index) -> Result<Self> {
        let map = DashMap::new();
        let settings = Settings {
            json_parsing_threads: 1,
            ..Default::default()
        };
        let new_index = LocalIndex::from_existing(name.clone(), index)
            .unwrap_or_else(|e| panic!("Unable to open index: {} because it's locked: {:?}", name, e));

        map.insert(name, new_index);

        Ok(IndexCatalog {
            settings,
            base_path: PathBuf::new(),
            local_handles: map,
        })
    }
}
