const js = import('./hls');

js.then(js => {
  console.log('ola');
  js.get_playlist('/static/master.m3u8');
})
