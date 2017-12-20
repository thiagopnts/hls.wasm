#[macro_use]
extern crate stdweb;
extern crate m3u8_rs;

mod hls;

use hls::get_playlist;

fn main() {
    stdweb::initialize();
    js!{
        Module.exports.getPlaylist = @{get_playlist};
    };
}
