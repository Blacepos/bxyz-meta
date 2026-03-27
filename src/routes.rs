use std::path::Path;

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
                let file_name = entry
                    .file_name()
                    .into_string()
                    .expect("I won't make the filenames weird");

                files.push(MusicDumpFile {
                    name: file_name.clone(),
                    path: format!("{AUDIO_ROUTE}/{file_name}"),
                });
            }
            Err(e) => {
                log::error!(
                    "Failed to read audio directory entry. 
                     Assuming transient and continuing. Error: \"{e}\""
                );
            }
        }
    }

    tera_ctx.insert("files", &files);

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
