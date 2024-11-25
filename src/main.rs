use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use tokio_stream::StreamExt;
use crossword_generator::{
    crossword::Crossword, 
    generator::{CrosswordGenerationRequest, CrosswordGenerator, CrosswordGeneratorSettings}, 
    word::Word,
};
use std::sync::Mutex;
use actix_files as fs;

// Request payload for submitting words
#[derive(Deserialize)]
struct WordListRequest {
    words: Vec<String>,
}

// Response structure for crosswords
#[derive(Serialize)]
struct CrosswordResponse {
    id: usize,                 // Unique ID for the crossword form
    crossword: String,         // Serialized crossword representation
    preview: Vec<Vec<char>>,   // 2D grid of characters for the crossword
}

// Shared generator state
struct AppState {
    generator: Mutex<CrosswordGenerator<u8, String>>,
}

// Endpoint to handle word submission and generate crosswords
async fn submit_words(
    data: web::Data<AppState>,
    req: web::Json<WordListRequest>,
) -> impl Responder {
    let words = req.words.clone();

    // Lock the generator to safely update its state
    let mut generator = data.generator.lock().unwrap();
    generator.words = words.into_iter()
        .map(|s| Word::new(s.to_lowercase(), None))
        .collect();

    generator.settings = CrosswordGeneratorSettings::default();

    // Create a crossword stream
    let mut stream = generator.crossword_stream_sorted(|s| {
        String::from_utf8(s.to_owned()).expect("The word is not in proper utf8 format")
    });

    stream.request_crossword(CrosswordGenerationRequest::All).await;

    let mut responses = Vec::new();
    let mut id = 0;

    // Collect crosswords and serialize responses
    while let Some(cw) = stream.next().await {
        let preview = generate_char_table_preview(&cw);
        let serialized = serde_json::to_string(&cw).unwrap();
        responses.push(CrosswordResponse {
            id,
            crossword: serialized,
            preview,
        });
        id += 1;
    }

    HttpResponse::Ok().json(responses) // Return as JSON response
}

// Helper function to generate a character table preview for the crossword
fn generate_char_table_preview(cw: &Crossword<u8, String>) -> Vec<Vec<char>> {
    let table = cw.generate_char_table();
    table
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cell| if cell != 0 { cell as char } else { ' ' }) // Use ' ' for black squares
                .collect()
        })
        .collect()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the crossword generator
    let generator = CrosswordGenerator::<u8, String>::default();
    let state = web::Data::new(AppState {
        generator: Mutex::new(generator),
    });

    // Set up the Actix Web server
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone()) // Shared generator state
            .route("/submit-words", web::post().to(submit_words)) // Word submission endpoint
            .service(fs::Files::new("/", "./static").index_file("index.html")) // Serve static files
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
