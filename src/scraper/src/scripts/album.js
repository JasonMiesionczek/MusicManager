var bandName = "";
function getAlbums() {
  var data = [];
  bandName = document.querySelector('[role="heading"]').text.runs[0].text;

  var showAllAlbumsLink = document.querySelector('[title="See all"]');
  if (showAllAlbumsLink == undefined) {
    var albums = document.querySelector("ytmusic-carousel-shelf-renderer").data
      .contents;
    for (var i = 0; i < albums.length; i++) {
      var id =
        albums[i].musicTwoRowItemRenderer.doubleTapNavigationEndpoint
          .watchPlaylistEndpoint.playlistId;
      var albumName = albums[i].musicTwoRowItemRenderer.title.runs[0].text;
      data.push({ id: id, name: albumName, artist: bandName });
    }
    external.invoke(JSON.stringify({ cmd: "albums", data: data }));
  } else {
    clickAlbumLink();
  }
}

function clickAlbumLink() {
  document.querySelector('[title="See all"]').click();
  setTimeout(function() {
    var data = [];
    var albums = document.querySelector("ytmusic-section-list-renderer").data
      .contents[0].musicShelfRenderer.contents;
    var albumElements = document.querySelectorAll(
      "ytmusic-responsive-list-item-renderer"
    );
    for (var i = 0; i < albums.length; i++) {
      var id =
        albums[i].musicResponsiveListItemRenderer.menu.menuRenderer.items[3]
          .menuServiceItemRenderer.serviceEndpoint.queueAddEndpoint.queueTarget
          .playlistId;
      var albumName =
        albums[i].musicResponsiveListItemRenderer.flexColumns[0]
          .musicResponsiveListItemFlexColumnRenderer.text.runs[0].text;
      var imageUrl = albumElements[i].querySelector(".yt-img-shadow").src;
      if (imageUrl.startsWith("data")) {
        imageUrl = "";
      }
      var year = document
        .querySelectorAll("ytmusic-responsive-list-item-renderer")
        [i].querySelector(".secondary-flex-columns")
        .querySelector("yt-formatted-string").text.runs[2].text;
      data.push({
        id: id,
        name: albumName,
        artist: bandName,
        image: imageUrl,
        year: year
      });
    }
    external.invoke(JSON.stringify({ cmd: "albums", data: data }));
  }, 5000);
}

function start() {
  document.querySelector('a[href*="channel"]').click();
  setTimeout(getAlbums, 5000);
}

start();
