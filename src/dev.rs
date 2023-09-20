use std::path::Path;

use elasticsearch::indices::{IndicesCreateParts, IndicesDeleteParts};
use elasticsearch::Elasticsearch;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::runtime::Handle;

use crate::elasticsearch::es_client;
use crate::util::error_exit;

#[derive(Debug)]
enum VelcroDevApiCommand {
    DeleteMapping(&'static str),
}

struct VelcroDevApi {
    es_client: Elasticsearch,
    runtime: Handle,
}

impl VelcroDevApi {
    fn initialize() -> anyhow::Result<Self> {
        let es_client = es_client()?;
        let runtime = Handle::current();
        Ok(Self { es_client, runtime })
    }

    fn dispatch(&self, cmd: VelcroDevApiCommand) {
        match cmd {
            VelcroDevApiCommand::DeleteMapping(index) => {
                self.runtime.block_on(self.delete_mapping(index))
            }
        }
    }

    #[allow(dead_code)]
    async fn create_mapping(&self, index_name: &str) {
        self.es_client
            .indices()
            .create(IndicesCreateParts::Index(index_name))
            .send()
            .await
            .expect("create index");
    }

    async fn delete_mapping(&self, index_name: &str) {
        self.es_client
            .indices()
            .delete(IndicesDeleteParts::Index(&[index_name]))
            .send()
            .await
            .expect("delete index");
    }
}

pub fn start_dev_mode() {
    let api = VelcroDevApi::initialize().expect("velcro dev api init");
    let mut watcher = match notify::recommended_watcher(move |event_result| {
        let event: Event = match event_result {
            Ok(event) => event,
            Err(e) => error_exit(format!("error on watch event: {:?}", e)),
        };
        if !event.paths.is_empty() {
            error_exit(format!("event {} paths: {}", event.paths.len(), ""));
        }
        match event.kind {
            EventKind::Remove(_) => api.dispatch(VelcroDevApiCommand::DeleteMapping("foo")),
            _ => println!("unhandled event kind: {:?}", event),
        };
    }) {
        Ok(watcher) => watcher,
        Err(err) => error_exit(err.to_string()),
    };

    let check_dirs = vec!["mappings", "templates"];
    let mut watching_dirs = Vec::with_capacity(check_dirs.len());
    for p in check_dirs {
        if Path::new(p).is_dir() {
            add_watch(&mut watcher, "mappings");
            watching_dirs.push(p);
        }
    }

    loop {
        std::thread::sleep(std::time::Duration::from_secs(86000));
    }
}

fn add_watch(watcher: &mut RecommendedWatcher, path: &str) {
    let watch_result = watcher.watch(Path::new(path), RecursiveMode::Recursive);
    if let Err(err) = watch_result {
        error_exit(format!("error adding watch for path `{path}`: {err}"));
    }
}
