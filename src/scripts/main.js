export function fadeContent() {
  document.querySelectorAll(".content")
    .forEach(el => el.classList.remove("fade_in"));
}
