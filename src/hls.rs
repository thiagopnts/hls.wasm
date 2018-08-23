use m3u8_rs::parse_playlist_res;
use m3u8_rs::playlist::Playlist;
use stdweb::web::{ArrayBuffer, TypedArray};

use browser;
use ts_demuxer;


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
                if let Some(segment) = media.segments.get(1) {
                    browser::log("fetching first segment");
                    browser::fetch_bytes(segment.uri.clone(), transmux);
                }
            },
            Err(_) => {
                browser::log("error parsing playlist");
            },
        }

}

const SYNC_MASK: u32 = 0xFF000000;
const TEI_MASK: u32 = 0x8000;
const PUSI_MASK: u32 = 0x4000;
const TP_MASK: u32 = 0x2000;
const PID_MASK: u32 = 0x1FFF;

const TSC_MASK: u32 = 0xc0;
const AFC_MASK: u32 = 0x3;
const CC_MASK: u32 = 0xF;

#[inline]
fn mask_for(size: u8, pos: u8) -> u32 {
    return ((1 << size) - 1) << (32 - size - pos);
}

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
    browser::log_fmt(format!("Sync byte: 0x{:X}", header & mask_for(8, 0)));
    browser::log_fmt(format!("Transport Error Indicator: {}", header & TEI_MASK));
    browser::log_fmt(format!("Transport Error Indicator: {}", header & mask_for(1, 8)));
    browser::log_fmt(format!("Payload Unit Start Indicator: {:b}", header & PUSI_MASK));
    browser::log_fmt(format!("Payload Unit Start Indicator: {:b}", header & mask_for(1, 9)));
    browser::log_fmt(format!("Transport Priority: {}", header & TP_MASK));
    browser::log_fmt(format!("Transport Priority: {}", header & mask_for(1, 10)));
    browser::log_fmt(format!("PID: {}", header & mask_for(13, 11)));
    browser::log_fmt(format!("Transport Scrambling Control: {:b}", header & mask_for(2, 24)));
    browser::log_fmt(format!("Transport Scrambling Control: {:b}", header & mask_for(2, 26)));
    browser::log_fmt(format!("Adaptation field control: {:#04b}", header & mask_for(2, 28)));
    browser::log_fmt(format!("Continuity counter: {:b}", header & 0xF));
}

pub fn get_playlist(url: String) {
    browser::fetch(url, parse_playlist);
}
