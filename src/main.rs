#[macro_use]
extern crate stdweb;
extern crate m3u8_rs;

mod hls;
mod browser;
mod ts_demuxer;

use hls::get_playlist;

fn main() {
    stdweb::initialize();
    js!{
        Module.exports.getPlaylist = @{get_playlist};
    };
}
