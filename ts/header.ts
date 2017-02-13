$(() => {
    function toggleHeader() {
        let title = $("#title");

        // Title must be showing before scrollWidth is calculated.
        title.show();
        if (title[0].scrollWidth > Math.ceil(title.innerWidth())) {
            return title.hide();
        }
    }

    window.addEventListener("resize", toggleHeader);

    return toggleHeader();
});
