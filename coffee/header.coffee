$ ->
  toggleHeader = () ->
    title = $("#title")

    # Title must be showing before scrollWidth is calculated.
    title.show()
    title.hide() if title[0].scrollWidth > Math.ceil title.innerWidth()

  window.addEventListener 'resize', toggleHeader

  toggleHeader()