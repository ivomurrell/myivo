$ ->
  pollNowListening = () ->
    $.getJSON "http://localhost:8345/scrobbles", (data) ->
      textElement = $("#scrobblar-text")
      title = data.recenttracks.track[0].name
      if not textElement?
        $(".bar-container").append(
          "<p class='bar-text' id='scrobblar-text'>#{title}</p>"
        )
      else if title isnt textElement.text()
        textClone = textElement.clone true
        textElement.remove()
        textClone.text title
        $(".bar-container").append textClone
      setTimeout pollNowListening, 10000

  pollNowListening()