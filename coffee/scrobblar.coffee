$ ->
  pollNowListening = () ->
    $.getJSON "http://localhost:8345/scrobbles.json", (data) ->
      title = data.recenttracks.track[0].name
      artist = data.recenttracks.track[0].artist['#text']
      text = "#{title} - #{artist}"
      nowplaying = data.recenttracks.track[0]['@attr']?.nowplaying
      prefix = if nowplaying then "Now playing" else "Last played"
      text = "#{prefix}: #{text}"

      textElement = $("#scrobblar-text")
      if not textElement.length
        $(".bar-container").append(
          "<p class='bar-text' id='scrobblar-text'>#{text}</p>"
        )
      else if text isnt textElement.text()
        textClone = textElement.clone true
        textElement.remove()
        textClone.text text
        $(".bar-container").append textClone
      setTimeout pollNowListening, 10000

  pollNowListening()