use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{
    runtime::Runtime,
    sync::{
        mpsc::{self, Receiver},
        RwLock,
    },
};

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (tx, rx) = mpsc::channel(1);

    let rt = Runtime::new().unwrap();
    let watcher = RecommendedWatcher::new(
        move |res| {
            rt.block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

pub async fn async_watch<P: AsRef<Path>>(
    path: P,
    state: Arc<RwLock<HashSet<PathBuf>>>,
) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.recv().await {
        match res {
            Ok(event) => {
                let mut state = state.write().await;
                use notify::EventKind::*;
                match event.kind {
                    Create(_) => event.paths.into_iter().for_each(|p| {
                        state.insert(p);
                    }),
                    Remove(_) => event.paths.into_iter().for_each(|p| {
                        state.remove(&p);
                    }),
                    _ => {}
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
