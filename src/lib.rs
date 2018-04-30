#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate m3u8_rs;

use wasm_bindgen::prelude::*;
use stdweb::web::ArrayBuffer;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn fetch(url: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello {}!", name));
}

use m3u8_rs::parse_playlist_res;
use m3u8_rs::playlist::Playlist;

#[wasm_bindgen]
pub fn parse_playlist(playlist: &str) {
        let parsed = parse_playlist_res(playlist.as_bytes());
        match parsed {
            Ok(Playlist::MasterPlaylist(master)) => {
                if let Some(variant) = master.variants.get(0) {
                    log("parsed master playlist, fetching first variant");
//                    get_playlist(variant.uri.clone());
                }
            },
            Ok(Playlist::MediaPlaylist(media)) => {
                log(&format!("parsed media playlist with {} segments", media.segments.len()));
                if let Some(segment) = media.segments.get(0) {
                    log("fetching first segment");
 //                   browser::fetch_bytes(segment.uri.clone(), transmux);
                }
            },
            Err(_) => {
                log("error parsing playlist");
            },
        }

}


fn transmux(array_buffer: ArrayBuffer) {
    log(&format!("got segment, buffer size: {}", array_buffer.len()));
    let mut buffer: Vec<u8> = Vec::from(array_buffer);
    log(format!("converted ArrayBuffer to Vec<u8>, vec size: {}", buffer.len()));
    if let Some(a) = buffer.get(0) {
        log(&format!("got first byte {}", a));
    }
}

//#[wasm_bindgen]
//pub fn get_playlist(url: &str) {
//    fetch(url);
//}
