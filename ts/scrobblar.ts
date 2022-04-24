import $ from "jquery";

$(() => {
  function pollNowListening() {
    return $.getJSON("https://oivov.io/scrobbles.json", (data) => {
      let trackData = data.recenttracks.track[0]["@attr"];
      let nowplaying = trackData != null ? trackData.nowplaying : 0;
      let prefix = nowplaying ? "Now playing: " : "Last played: ";
      $("#scrobblar-prefix").text(prefix);

      let title: string = data.recenttracks.track[0].name;
      let artist: string = data.recenttracks.track[0].artist["#text"];
      let text = title + " - " + artist;

      let textElement = $("#scrobblar-music");
      if (!textElement.length) {
        $(".bar-container").append(
          `<p class='bar-text-music' id='scrobblar-music'>${text}
					</p>`
        );
      } else if (text !== textElement.text()) {
        let textClone = textElement.clone(true);
        textElement.remove();
        textClone.text(text);
        $(".bar-container").append(textClone);
      }

      let art: string = data.recenttracks.track[0].image[0]["#text"];
      let art2x: string = data.recenttracks.track[0].image[1]["#text"];
      let art3x: string = data.recenttracks.track[0].image[2]["#text"];

      let coverElement = $("#scrobblar-art");
      if (art === "") {
        coverElement.remove();
      } else if (!coverElement.length) {
        $(".bar-container").prepend(
          `<img class='bar-cover' id='scrobblar-art'
					src='${art}' alt='Cover art'
					srcset='${art}, ${art2x} 2x, ${art3x} 3x'></img>`
        );
      } else if (art !== coverElement.attr("src")) {
        let coverClone = coverElement.clone(true);
        coverElement.remove();
        coverClone.attr("src", art);
        $(".bar-container").prepend(coverClone);
      }

      return setTimeout(pollNowListening, 10000);
    });
  }

  return pollNowListening();
});
