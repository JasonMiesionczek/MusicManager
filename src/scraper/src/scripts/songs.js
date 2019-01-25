function getSongData() {
  var data = [];
  var albumImage = document.querySelector(".yt-img-shadow").src;
  var songs = document.querySelector(
    "ytmusic-data-bound-album-release-tracks-shelf-renderer"
  ).data.shelfMold.musicShelfRenderer.contents;
  for (var i = 0; i < songs.length; i++) {
    var songId =
      songs[i].musicListItemRenderer.overlay.musicItemThumbnailOverlayRenderer
        .content.musicPlayButtonRenderer.playNavigationEndpoint.watchEndpoint
        .videoId;
    var albumId =
      songs[i].musicListItemRenderer.overlay.musicItemThumbnailOverlayRenderer
        .content.musicPlayButtonRenderer.playNavigationEndpoint.watchEndpoint
        .playlistId;
    var name =
      songs[i].musicListItemRenderer.overlay.musicItemThumbnailOverlayRenderer
        .content.musicPlayButtonRenderer.accessibilityPlayData.accessibilityData
        .label;
    name = name.substring(5);
    if (typeof songId == "undefined" || songId == "undefined") {
      continue;
    }
    data.push({ id: songId, name: name, num: i + 1, image: albumImage });
  }
  external.invoke(JSON.stringify({ cmd: "songs", data: data }));
}

function start() {
  setTimeout(getSongData, 5000);
}

start();
