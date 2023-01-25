import "@github/relative-time-element";

function onImageError(ev) {
  ev.preventDefault();

  ev.target.src = ev.target.getAttribute("data-alt-image");
}

function initOnImageError() {
  document
    .querySelectorAll("img[data-alt-image]")
    .forEach((el) => el.addEventListener("error", onImageError));
}

function main() {
  initOnImageError();
}

document.addEventListener("load", main);
