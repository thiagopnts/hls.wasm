use m3u8_rs::parse_playlist_res;
use m3u8_rs::playlist::Playlist;

pub fn get_playlist(url: String) {
    let parse = |playlist: String| {
        let parsed = parse_playlist_res(&playlist.as_bytes());
        match parsed {
            Ok(Playlist::MasterPlaylist(master)) => {
                if let Some(_) = master.variants.get(0) {
                    js!{ console.log("parsed master playlist"); };
                }
            },
            Ok(Playlist::MediaPlaylist(_)) => {
                js!{console.log("parsed media playlist")};
            },
            Err(_) => {
                js!{console.log("error parsing playlist")};
            },
        }
    };
    js!{
        var parsePlaylist = @{parse};
        fetch(@{url})
            .then(response => response.text())
            .then(playlist => {
                parsePlaylist(playlist);
                parsePlaylist.drop();
            });
    }
}
