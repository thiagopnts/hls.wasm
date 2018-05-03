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

const SYNC_MASK: u32 = 0xff000000;
const TEI_MASK: u32 = 0x800000;
const PUSI_MASK: u32 = 0x400000;
const TP_MASK: u32 = 0x200000;
const PID_MASK: u32 = 0x1fff00;

fn transmux(array_buffer: ArrayBuffer) {
    browser::log_fmt(format!("got segment, buffer size: {}", array_buffer.len()));
    let mut buffer: Vec<u8> = Vec::from(array_buffer);
    browser::log_fmt(format!("converted ArrayBuffer to Vec<u8>, vec size: {}", buffer.len()));
    browser::log("4 byte Transport Stream Header:");
    let mut header = 0u32;
    // assemble the 4 bytes from 8 bits buffer
    let vec =  buffer.drain(..4).collect::<Vec<u8>>();
    browser::log_fmt(format!("drained bytes: {}", vec.len()));
    for i in 0..vec.len() {
        browser::log_fmt(format!("joining byte: {:#010b}", vec[i]));
        header = (header << 8) | (vec[i] as u32);
        browser::log_fmt(format!("header: {:#034b}", header));
    }
    browser::log_fmt(format!("Sync byte: 0x{:X}", header & SYNC_MASK));
    browser::log_fmt(format!("Transport Error Indicator: {}", header & TEI_MASK));
    browser::log_fmt(format!("Payload Unit Start Indicator: {}", header & PUSI_MASK));
    browser::log_fmt(format!("Transport Priority: {}", header & TP_MASK));
    browser::log_fmt(format!("PID: {}", header & PID_MASK));
}

pub fn get_playlist(url: String) {
    browser::fetch(url, parse_playlist);
}
