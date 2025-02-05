use futures::StreamExt;
use itertools::{Chunk, Itertools};
use meilisearch_sdk::client::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::ReadDir;
use std::sync::atomic::{AtomicUsize, Ordering};


fn get_id() -> usize {
    static COUNTER:AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
#[tokio::main] // Use Tokio's main macro to start the runtime
async fn main() {
    println!("Start indexing!");
    let api_key = Some("dZzLAoGuTcErWP6o6rbDmJWOXbdYnxIs9_iD6Z__1T4");
    let client = Client::new("http://localhost:7700", api_key).unwrap();
    let path_to_notes = "../../projects/notes/".to_owned();
    let journals = path_to_notes.clone() + "/journals/";
    let pages = path_to_notes.clone() + "/pages/";

    let journals_paths = fs::read_dir(journals).unwrap();
    let pages_paths = fs::read_dir(pages).unwrap();
    add_notes(&client, pages_paths).await;
    add_notes(&client, journals_paths).await;
}

async fn add_notes(client: &Client, paths: ReadDir) {
    let batch_size = 30;
    futures::stream::iter(paths.chunks(batch_size).into_iter()).for_each_concurrent(None, |g| {
        let client = client.clone();
        async move {
            let notes = get_notes_batch(g);
            client.index("Notes").add_documents(&notes, None).await.unwrap();
            println!("new chunk of {} files has been indexed",batch_size);
        }
    }).await;
}

fn get_notes_batch(g: Chunk<ReadDir>) -> Vec<Note> {
    let notes: Vec<Note> = g.map(|p| {
        let path = p.unwrap().path();
        let content = fs::read_to_string(&path);
        let note = Note {
            id: get_id(),
            title: path.file_name().unwrap().to_str().unwrap().to_string(),
            path: path.to_str().unwrap().to_string(),
            contents: content.unwrap()
        };
        note
    }).collect();
    notes
}

#[derive(Serialize, Deserialize)]
struct Note {
    id:usize,
    title: String,
    path: String,
    contents: String,
}