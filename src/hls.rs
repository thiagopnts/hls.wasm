use m3u8_rs::parse_playlist_res;
use m3u8_rs::playlist::Playlist;
use stdweb::web::{ArrayBuffer, TypedArray};

use browser;

fn parse_playlist(playlist: String) {
        let parsed = parse_playlist_res(&playlist.as_bytes());
        match parsed {
            Ok(Playlist::MasterPlaylist(master)) => {
                if let Some(variant) = master.variants.get(0) {
                    browser::log("parsed master playlist, fetching first variant");
                    get_playlist(variant.uri.clone());
                }
            },
            Ok(Playlist::MediaPlaylist(media)) => {
                browser::log_fmt(format!("parsed media playlist with {} segments", media.segments.len()));
                if let Some(segment) = media.segments.get(0) {
                    browser::log("fetching first segment");
                    browser::fetch_bytes(segment.uri.clone(), transmux);
                }
            },
            Err(_) => {
                browser::log("error parsing playlist");
            },
        }

}


fn transmux(array_buffer: ArrayBuffer) {
    browser::log_fmt(format!("got segment, buffer size: {}", array_buffer.len()));
    let mut buffer: Vec<u8> = Vec::from(array_buffer);
    browser::log_fmt(format!("converted ArrayBuffer to Vec<u8>, vec size: {}", buffer.len()));
    if let Some(a) = buffer.get(0) {
        browser::log_fmt(format!("got first byte {}", a));
    }
}

pub fn get_playlist(url: String) {
    browser::fetch(url, parse_playlist);
}