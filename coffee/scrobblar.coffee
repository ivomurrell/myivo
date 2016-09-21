$ ->
  pollNowListening = () ->
    $.get "http://localhost:8345/scrobbles", (data) ->
      if data isnt $("#scrobblar-text").text()
        $("#scrobblar-text").text(data)
      setTimeout pollNowListening, 10000

  pollNowListening()