$ ->
  $.get "http://localhost:8345/scrobbles", (data) ->
    $("#scrobblar-text").text(data)