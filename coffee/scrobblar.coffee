$ ->
  pollNowListening = () ->
    $.get "http://localhost:8345/scrobbles", (data) ->
      textElement = $("#scrobblar-text")
      if not textElement?
        $(".bar-container").append(
          "<p class='bar-text' id='scrobblar-text'>#{data}</p>"
        )
      else if data isnt textElement.text()
        textClone = textElement.clone true
        textElement.remove()
        textClone.text data
        $(".bar-container").append textClone
      setTimeout pollNowListening, 10000

  pollNowListening()