#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tokio::main]
async fn main() {
    
    tokio::spawn(async move {
        warp::serve(warp::fs::dir(""))
            .run(([127, 0, 0, 1], 7777))
            .await
    });

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
