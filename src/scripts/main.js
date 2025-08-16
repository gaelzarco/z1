export function fadeContent() {
  document.querySelectorAll(".content")
    .forEach(el => el.classList.remove("fade_in"));
}

export function initTheme() {
  const saved = localStorage.getItem("theme");
  const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
  const current = saved || (prefersDark ? "dark" : "light");
  document.documentElement.setAttribute("data-theme", current);
  return current;
}

export function toggleTheme() {
  const el = document.documentElement;
  const theme = el.getAttribute("data-theme") === "dark" ? "light" : "dark";
  el.setAttribute("data-theme", theme);
  try {
    localStorage.setItem("theme", theme);
  } catch (e) {
    console.log("[ERROR] Unable to update theme...");
  }
  return theme;
}
