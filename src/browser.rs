use stdweb::web::ArrayBuffer;

pub fn log(msg: &str) {
    js!{
        console.log("WASM:", @{msg});
    }
}

pub fn log_fmt(msg: String) {
    js!{
        console.log("WASM:", @{msg});
    }
}

pub fn fetch(url: String, cb: fn(String)) {
    js!{
        var cb = @{cb};
        fetch(@{url})
            .then(response => response.text())
            .then(playlist => {
                cb(playlist);
                cb.drop();
            });
    }
}

pub fn fetch_bytes(url: String, cb: fn(ArrayBuffer)) {
    js!{
        var cb = @{cb};
        fetch(@{url})
            .then(response => response.arrayBuffer())
            .then(buffer => {
                console.log(buffer);
                cb(buffer);
                cb.drop();
            });
    }
}