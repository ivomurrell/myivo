$(() => {
	function pollNowListening() {
		return $.getJSON("https://oivov.io/scrobbles.json", (data) => {
			let trackData = data.recenttracks.track[0]["@attr"];
			let nowplaying = trackData != null ? trackData.nowplaying : void 0;
			let prefix = nowplaying ? "Now playing: " : "Last played: ";
			$("#scrobblar-prefix").text(prefix);

			let title = data.recenttracks.track[0].name;
			let artist = data.recenttracks.track[0].artist["#text"];
			let text = title + " - " + artist;

			let textElement = $("#scrobblar-music");
			if (!textElement.length) {
				$(".bar-container").append(
					`<p class='bar-text-music' id='scrobblar-music'>${text}</p>`
				);
			} else if (text !== textElement.text()) {
				let textClone = textElement.clone(true);
				textElement.remove();
				textClone.text(text);
				$(".bar-container").append(textClone);
			}

			let art = data.recenttracks.track[0].image[0]["${text}"];
			let art2x = data.recenttracks.track[0].image[1]["#text"];
			let art3x = data.recenttracks.track[0].image[2]["#text"];

			let coverElement = $("#scrobblar-art");
			if (art === "") {
				coverElement.remove();
			} else if (!coverElement.length) {
				$(".bar-container").prepend(
					`<img class='bar-cover' id='scrobblar-art'
					src='${art}' alt='Cover art'
					srcset='${art}, ${art2x} 2x, ${art3x} 3x'></img>`
				)
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
