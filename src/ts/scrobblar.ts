async function pollNowListening() {
  const resp = await fetch("https://oivov.io/scrobbles.json");
  const data = await resp.json();
  const track = data.recenttracks.track[0];
  const trackData = track["@attr"];
  const nowPlaying = trackData?.nowplaying ?? false;
  const prefix = nowPlaying ? "Now playing: " : "Last played: ";
  const scrobblarPrefix = document.getElementById("scrobblar-prefix")!;
  scrobblarPrefix.textContent = prefix;

  const title: string = track.name;
  const artist: string = track.artist["#text"];
  const text = `${title} - ${artist}`;

  const textElement = document.getElementById("scrobblar-music");
  const container = document.getElementById("bar-container")!;
  if (!textElement) {
    const scrobblarMusic = document.createElement("p");
    scrobblarMusic.className = "bar-text-music";
    scrobblarMusic.id = "scrobblar-music";
    scrobblarMusic.appendChild(document.createTextNode(text));
    container.appendChild(scrobblarMusic);
  } else if (text !== textElement.textContent) {
    const textClone = textElement.cloneNode(true);
    textElement.remove();
    textClone.textContent = text;
    container.appendChild(textClone);
  }

  const art: string = track.image[0]["#text"];
  const art2x: string = track.image[1]["#text"];
  const art3x: string = track.image[2]["#text"];

  const coverElement = document.getElementById("scrobblar-art") as
    | HTMLImageElement
    | undefined;
  if (art === "") {
    coverElement?.remove();
  } else if (!coverElement) {
    const scrobblarArt = document.createElement("img");
    scrobblarArt.className = "bar-cover";
    scrobblarArt.id = "scrobblar-art";
    scrobblarArt.src = art;
    scrobblarArt.alt = "Cover art";
    scrobblarArt.srcset = `${art}, ${art2x} 2x, ${art3x} 3x`;
    container.prepend(scrobblarArt);
  } else if (art !== coverElement.src) {
    coverElement.src = art;
    coverElement.srcset = `${art}, ${art2x} 2x, ${art3x} 3x`;
  }

  setTimeout(pollNowListening, 10000);
}

pollNowListening();
