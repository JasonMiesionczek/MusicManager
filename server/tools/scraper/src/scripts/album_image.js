function getAlbumImage() {
  var albumImage = document.querySelector("#img").src;
  external.invoke(JSON.stringify({ cmd: "image", data: albumImage }));
}

function start() {
  setTimeout(getAlbumImage, 10000);
}

start();
