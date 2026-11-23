// StratIQ for Windows: a window onto https://stratiq.win.
// The page gets no access to this PC (no Tauri APIs are enabled for it), and the Riot API key never ships in the app:
// it stays on the StratIQ server. Only StratIQ and Riot sign-in pages open inside the window; every other link opens
// in your normal browser.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Url, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;

const START_URL: &str = "https://stratiq.win/app";

fn allowed_inside(url: &Url) -> bool {
    match url.scheme() {
        "https" => {
            let host = url.host_str().unwrap_or("");
            host == "stratiq.win"
                || host == "www.stratiq.win"
                || host == "riotgames.com"
                || host.ends_with(".riotgames.com") // Riot sign-in (RSO)
        }
        "about" | "data" | "blob" => true, // blank pages and in-page downloads
        _ => false,
    }
}

fn main() {
    tauri::Builder::default()
        // opening StratIQ again just brings the open window to the front
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.unminimize();
                let _ = w.show();
                let _ = w.set_focus();
            }
        }))
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(START_URL.parse().expect("valid start URL")))
                .title("StratIQ")
                .inner_size(1440.0, 900.0)
                .min_inner_size(1024.0, 680.0)
                .center()
                .on_navigation(move |url| {
                    if allowed_inside(url) {
                        return true;
                    }
                    if url.scheme() == "https" || url.scheme() == "http" || url.scheme() == "mailto" {
                        let _ = handle.opener().open_url(url.as_str(), None::<&str>);
                    }
                    false
                })
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("StratIQ failed to start");
}
