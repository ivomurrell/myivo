import $ from "jquery";

$(() => {
  function toggleHeader() {
    const title = $("#title");

    // Title must be showing before scrollWidth is calculated.
    title.show();
    if (title[0].scrollWidth > Math.ceil(title.innerWidth() ?? 0)) {
      return title.hide();
    }
  }

  window.addEventListener("resize", toggleHeader);

  return toggleHeader();
});
