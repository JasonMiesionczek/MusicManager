function getArtistImage() {
  var artist_image = document
    .querySelector("ytmusic-fullbleed-thumbnail-renderer")
    .querySelector("img").src;

  external.invoke(JSON.stringify({ cmd: "artist", data: artist_image }));
}

function start() {
  document.querySelector('a[href*="channel"]').click();
  setTimeout(getArtistImage, 10000);
}

start();
