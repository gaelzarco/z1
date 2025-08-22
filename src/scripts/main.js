export function initTheme() {
  const html = document.documentElement;
  const existingTheme = html.getAttribute("data-theme");

  if (existingTheme) return existingTheme;

  const savedTheme = localStorage.getItem("theme");
  const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
  const current = savedTheme || (prefersDark ? "dark" : "light");

  document.documentElement.setAttribute("data-theme", theme);
  localStorage.setItem("theme", current);

  return current;
}

export function toggleTheme() {
  const html = document.documentElement;
  const theme = html.getAttribute("data-theme") === "dark" ? "light" : "dark";

  document.documentElement.setAttribute("data-theme", theme);
  localStorage.setItem("theme", theme);

  return theme;
}
