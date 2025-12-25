use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::thread;

/// The main Pools struct containing all resource pools.
#[derive(Clone, Debug)]
pub struct Pools {
    pub threads: Arc<ThreadPool>,
    pub resources: Arc<ResourcePool>,
    pub network: Arc<NetworkPool>,
}

impl Pools {
    pub fn new() -> Self {
        Self {
            threads: Arc::new(ThreadPool::new()),
            resources: Arc::new(ResourcePool::new()),
            network: Arc::new(NetworkPool::new()),
        }
    }
}

/// Manages thread execution using Rayon.
#[derive(Clone, Debug)]
pub struct ThreadPool {
    pool: Arc<rayon::ThreadPool>,
}

impl ThreadPool {
    pub fn new() -> Self {
        let pool = rayon::ThreadPoolBuilder::new()
            .build()
            .expect("Failed to create thread pool");
        Self {
            pool: Arc::new(pool),
        }
    }

    pub fn spawn<F, T>(&self, f: F)
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.pool.spawn(move || {
            f();
        });
    }

    pub fn install<OP, R>(&self, op: OP) -> R
    where
        OP: FnOnce() -> R + Send,
        R: Send,
    {
        self.pool.install(op)
    }
}

/// Manages file resources and potentially memory buffers.
#[derive(Clone, Debug)]
pub struct ResourcePool {
    // Simple in-memory cache for file contents: Path -> Content
    cache: Arc<RwLock<HashMap<PathBuf, String>>>,
}

impl ResourcePool {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Reads a file, checking the cache first.
    /// Returns the content as a String.
    pub fn open_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<String> {
        let path = path.as_ref().to_path_buf();
        {
            let cache = self.cache.read().unwrap();
            if let Some(content) = cache.get(&path) {
                return Ok(content.clone());
            }
        }

        let content = fs::read_to_string(&path)?;

        {
            let mut cache = self.cache.write().unwrap();
            cache.insert(path, content.clone());
        }

        Ok(content)
    }

    /// Writes to a file and updates the cache.
    pub fn save_file<P: AsRef<Path>>(&self, path: P, content: String) -> std::io::Result<()> {
        let path = path.as_ref().to_path_buf();
        fs::write(&path, &content)?;

        {
            let mut cache = self.cache.write().unwrap();
            cache.insert(path, content);
        }

        Ok(())
    }

    /// Wraps fs::read_dir
    pub fn list_dir<P: AsRef<Path>>(&self, path: P) -> std::io::Result<fs::ReadDir> {
        fs::read_dir(path)
    }
}

/// Manages network connections.
#[derive(Clone, Debug)]
pub struct NetworkPool {
    client: reqwest::blocking::Client,
}

impl NetworkPool {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn get(&self, url: &str) -> reqwest::Result<String> {
        self.client.get(url).send()?.text()
    }

    pub fn post(&self, url: &str, body: String) -> reqwest::Result<String> {
        self.client.post(url).body(body).send()?.text()
    }
}
