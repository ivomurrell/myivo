(function() {
  $(function() {
    var toggleHeader;
    toggleHeader = function() {
      var title;
      title = $("#title");
      title.show();
      if (title[0].scrollWidth > Math.ceil(title.innerWidth())) {
        return title.hide();
      }
    };
    window.addEventListener('resize', toggleHeader);
    return toggleHeader();
  });

}).call(this);

//# sourceMappingURL=header.js.map
