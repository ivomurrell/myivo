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
        $(".bar-container").prepend(
          "<p class='bar-text' id='scrobblar-text'>#{text}</p>"
        )
      else if text isnt textElement.text()
        textClone = textElement.clone true
        textElement.remove()
        textClone.text text
        $(".bar-container").prepend textClone

      art = data.recenttracks.track[0].image[0]['#text']

      coverElement = $("#scrobblar-art")
      if not coverElement.length
        $(".bar-container").append(
          "<img class='bar-cover' id='scrobblar-art'
            src='#{art}' alt='Cover art'></img>"
        )
      else if art isnt coverElement.attr "src"
        coverClone = coverElement.clone true
        coverElement.remove()
        coverClone.attr "src", art
        $(".bar-container").append coverClone

      setTimeout pollNowListening, 10000

  pollNowListening()