const NAVIGATION_KEYS = {
  ArrowLeft: { action: "prevSlide" },
  ArrowRight: { action: "nextSlide" },
  1: { elementId: "projects-link" },
  2: { elementId: "experience-link" },
  3: { elementId: "education-link" },
  4: { elementId: "links-link" },
};

document.addEventListener("keydown", function (event) {
  const config = NAVIGATION_KEYS[event.key];
  if (!config) return;

  event.preventDefault();

  if (config.action) {
    window[config.action]?.();
  } else if (config.elementId) {
    document.getElementById(config.elementId)?.click();
  }
});
