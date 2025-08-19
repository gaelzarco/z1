export function initTheme() {
  const html = document.documentElement;
  const existing = html.getAttribute("data-theme");

  if (existing) return existing;

  const saved = localStorage.getItem("theme");
  const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
  const current = saved || (prefersDark ? "dark" : "light");

  html.setAttribute("data-theme", current);

  return current;
}

export function toggleTheme() {
  const el = document.documentElement;
  const theme = el.getAttribute("data-theme") === "dark" ? "light" : "dark";

  el.setAttribute("data-theme", theme);
  localStorage.setItem("theme", theme);

  return theme;
}

export function scrollTop() {
  requestAnimationFrame(() => {
    window.scrollTo({ top: 0, left: 0, behavior: "smooth" });
  });
}
