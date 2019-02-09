function getAlbumImage() {
  var albumImage = document.querySelector(".yt-img-shadow").src;
  external.invoke(JSON.stringify({ cmd: "image", data: albumImage }));
}

function start() {
  setTimeout(getAlbumImage, 10000);
}

start();
