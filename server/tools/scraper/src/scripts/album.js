var bandName = "";
function getAlbums() {
  var data = [];
  bandName = document.querySelector('[role="heading"]').text.runs[0].text;

  var hasLinks = document.querySelectorAll('[title="See all"]').length;
  var useSimpleAlbum = !hasLinks;
  if (hasLinks) {
    var seeAllLink = document.querySelectorAll('[title="See all"]')[0].parentElement.parentElement.parentElement.parentElement.children[0].text.runs[0].text;
    useSimpleAlbum = seeAllLink !== "Albums";
  }

  //var showAllAlbumsLink = document.querySelectorAll('[title="See all"]');
  if (useSimpleAlbum) {
    var albums = document.querySelector("ytmusic-carousel-shelf-renderer").data
      .contents;
    var albumElements = document
      .querySelector("ytmusic-carousel")
      .querySelectorAll("img");
    for (var i = 0; i < albums.length; i++) {
      var id =
        albums[i].musicTwoRowItemRenderer.doubleTapNavigationEndpoint
          .watchPlaylistEndpoint.playlistId;
      var albumName = albums[i].musicTwoRowItemRenderer.title.runs[0].text;
      var year = albums[i].musicTwoRowItemRenderer.subtitle.runs[2].text;
      var image = albumElements[i].src;
      if (image.startsWith("data")) {
        image = "";
      }
      data.push({
        id: id,
        name: albumName,
        artist: bandName,
        year: year,
        image: image
      });
    }
    external.invoke(JSON.stringify({ cmd: "albums", data: data }));
  } else {
    clickAlbumLink();
  }
}

function clickAlbumLink() {
  document.querySelector('[title="See all"]').click();
  setTimeout(function () {
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
      var year_runs = document
        .querySelectorAll("ytmusic-responsive-list-item-renderer")
      [i].querySelector(".secondary-flex-columns")
        .querySelector("yt-formatted-string").text.runs;
      var year = "0";
      if (year_runs.length > 1) {
        year = document
          .querySelectorAll("ytmusic-responsive-list-item-renderer")
        [i].querySelector(".secondary-flex-columns")
          .querySelector("yt-formatted-string").text.runs[2].text;
      }

      data.push({
        id: id,
        name: albumName,
        artist: bandName,
        image: imageUrl,
        year: year
      });
    }
    external.invoke(JSON.stringify({ cmd: "albums", data: data }));
  }, 10000);
}

function abort() {
  external.invoke(JSON.stringify({ cmd: "abort" }));
}

function start() {
  document.querySelector('a[href*="channel"]').click();
  setTimeout(getAlbums, 10000);
  setTimeout(abort, 60000);
}

start();
