function toggleHeader() {
  const title = document.getElementById("title")!;

  // Title must be showing before scrollWidth is calculated.
  title.style.display = "";
  if (title.scrollWidth > title.offsetWidth) {
    title.style.display = "none";
  }
}

window.addEventListener("resize", toggleHeader);

toggleHeader();
