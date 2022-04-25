import $ from "jquery";

$(() => {
  function pollNowListening() {
    return $.getJSON("https://oivov.io/scrobbles.json", (data) => {
      const trackData = data.recenttracks.track[0]["@attr"];
      const nowplaying = trackData != null ? trackData.nowplaying : 0;
      const prefix = nowplaying ? "Now playing: " : "Last played: ";
      $("#scrobblar-prefix").text(prefix);

      const title: string = data.recenttracks.track[0].name;
      const artist: string = data.recenttracks.track[0].artist["#text"];
      const text = title + " - " + artist;

      const textElement = $("#scrobblar-music");
      if (!textElement.length) {
        $(".bar-container").append(
          `<p class='bar-text-music' id='scrobblar-music'>${text}
					</p>`
        );
      } else if (text !== textElement.text()) {
        const textClone = textElement.clone(true);
        textElement.remove();
        textClone.text(text);
        $(".bar-container").append(textClone);
      }

      const art: string = data.recenttracks.track[0].image[0]["#text"];
      const art2x: string = data.recenttracks.track[0].image[1]["#text"];
      const art3x: string = data.recenttracks.track[0].image[2]["#text"];

      const coverElement = $("#scrobblar-art");
      if (art === "") {
        coverElement.remove();
      } else if (!coverElement.length) {
        $(".bar-container").prepend(
          `<img class='bar-cover' id='scrobblar-art'
					src='${art}' alt='Cover art'
					srcset='${art}, ${art2x} 2x, ${art3x} 3x'></img>`
        );
      } else if (art !== coverElement.attr("src")) {
        const coverClone = coverElement.clone(true);
        coverElement.remove();
        coverClone.attr("src", art);
        $(".bar-container").prepend(coverClone);
      }

      return setTimeout(pollNowListening, 10000);
    });
  }

  return pollNowListening();
});
