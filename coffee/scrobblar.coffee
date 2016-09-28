$ ->
  pollNowListening = () ->
    $.getJSON "https://oivov.io/scrobbles.json", (data) ->
      nowplaying = data.recenttracks.track[0]['@attr']?.nowplaying
      prefix = if nowplaying then "Now playing: " else "Last played: "
      $("#scrobblar-prefix").text(prefix)
      
      title = data.recenttracks.track[0].name
      artist = data.recenttracks.track[0].artist['#text']
      text = "#{title} - #{artist}"

      textElement = $("#scrobblar-music")
      if not textElement.length
        $(".bar-container").append(
          "<p class='bar-text-music' id='scrobblar-music'>#{text}</p>"
        )
      else if text isnt textElement.text()
        textClone = textElement.clone true
        textElement.remove()
        textClone.text text
        $(".bar-container").append textClone

      art = data.recenttracks.track[0].image[0]['#text']

      coverElement = $("#scrobblar-art")
      if art is ""
        coverElement.remove()
      else if not coverElement.length
        $(".bar-container").prepend(
          "<img class='bar-cover' id='scrobblar-art'
            src='#{art}' alt='Cover art'></img>"
        )
      else if art isnt coverElement.attr "src"
        coverClone = coverElement.clone true
        coverElement.remove()
        coverClone.attr "src", art
        $(".bar-container").prepend coverClone

      setTimeout pollNowListening, 10000

  pollNowListening()