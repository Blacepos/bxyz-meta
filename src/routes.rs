use std::{cmp::Ordering, fs::DirEntry, path::Path};

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use serde::Serialize;
use tokio::fs;

use crate::paths::{AUDIO_DIR, CONTENT_DIR};

const AUDIO_ROUTE: &str = "/meta/content/audio";

#[derive(Serialize)]
struct MusicDumpFile {
    name: String,
    path: String,
}

pub async fn route_index() -> Response {
    Html(
        fs::read(Path::new(CONTENT_DIR).join("pages/index.html"))
            .await
            .expect("index.html exists"),
    )
    .into_response()
}

pub async fn route_audio(State(tera_eng): State<tera::Tera>) -> (StatusCode, Response) {
    let mut tera_ctx = tera::Context::new();

    let mut files = Vec::new();

    for res in std::fs::read_dir(AUDIO_DIR).expect("Audio directory exists") {
        match res {
            Ok(entry) => {
                files.push(entry);
            }
            Err(e) => {
                log::error!(
                    "Failed to read audio directory entry. 
                     Assuming transient and continuing. Error: \"{e}\""
                );
            }
        }
    }

    files.sort_by(compare_dir_entries);

    let template_structs: Vec<MusicDumpFile> = files
        .iter()
        .map(|d| {
            let file_name = d
                .file_name()
                .into_string()
                .expect("I won't make the filenames weird, I promise");

            MusicDumpFile {
                name: file_name.clone(),
                path: format!("{AUDIO_ROUTE}/{file_name}"),
            }
        })
        .rev() // Reverse, so "larger" entries (newer) are first
        .collect();

    tera_ctx.insert("files", &template_structs);

    match tera_eng.render("music_dump.html", &tera_ctx) {
        Ok(html) => (StatusCode::OK, Html(html).into_response()),
        Err(e) => {
            log::error!("Failed to render music dump template: \"{e}\"");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to display page".into_response(),
            )
        }
    }
}

fn compare_dir_entries(a: &DirEntry, b: &DirEntry) -> Ordering {
    let a_maybe_created = a.metadata().and_then(|m| m.created()).ok();
    let b_maybe_created = b.metadata().and_then(|m| m.created()).ok();

    match (a_maybe_created, b_maybe_created) {
        (Some(a_created), Some(b_created)) => Ord::cmp(&a_created, &b_created),
        // Files with bad metadata are "older" (will appear at the end of the list)
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        // Both have bad metadata, sort alphabetically.
        // A "lower" name (i.e., alphabetically first), should be "newer"/greater
        (None, None) => Ord::cmp(&a.file_name(), &b.file_name()).reverse(),
    }
}
