$(() => {
    function pollNowListening() {
        return $.getJSON("https://oivov.io/scrobbles.json", (data) => {
            var art, art2x, art3x, artist, coverClone, coverElement, nowplaying, prefix, text, textClone, textElement, title, _ref;
            nowplaying = (_ref = data.recenttracks.track[0]["@attr"]) != null ? _ref.nowplaying : void 0;
            prefix = nowplaying ? "Now playing: " : "Last played: ";
            $("#scrobblar-prefix").text(prefix);

            title = data.recenttracks.track[0].name;
            artist = data.recenttracks.track[0].artist["#text"];  //text']
            text = title + " - " + artist;

            textElement = $("#scrobblar-music");
            if (!textElement.length) {
                $(".bar-container").append("<p class='bar-text-music' id='scrobblar-music'>" + text + "</p>");
            } else if (text !== textElement.text()) {
                textClone = textElement.clone(true);
                textElement.remove();
                textClone.text(text);
                $(".bar-container").append(textClone);
            }

            art = data.recenttracks.track[0].image[0]["#text"];  //text']
            art2x = data.recenttracks.track[0].image[1]["#text"];  //text']
            art3x = data.recenttracks.track[0].image[2]["#text"];  //text']

            coverElement = $("#scrobblar-art");
            if (art === "") {
                coverElement.remove();
            } else if (!coverElement.length) {
                $(".bar-container").prepend("<img class='bar-cover' id='scrobblar-art'            src='" + art + "' alt='Cover art'            srcset='" + art + ", " + art2x + " 2x, " + art3x + " 3x'></img>");  //{art}' alt='Cover art'  //{art}, #{art2x} 2x, #{art3x} 3x'></img>"
            } else if (art !== coverElement.attr("src")) {
                coverClone = coverElement.clone(true);
                coverElement.remove();
                coverClone.attr("src", art);
                $(".bar-container").prepend(coverClone);
            }

            return setTimeout(pollNowListening, 10000);
        });
    }

    return pollNowListening();
});
