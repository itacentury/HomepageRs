const NAV_LINK_IDS = [
  "projects-link",
  "experience-link",
  "education-link",
  "links-link",
];

function cycleNav(direction) {
  const currentIndex = NAV_LINK_IDS.findIndex((id) => {
    return document.getElementById(id)?.classList.contains("active");
  });

  const nextIndex =
    (currentIndex + direction + NAV_LINK_IDS.length) % NAV_LINK_IDS.length;
  document.getElementById(NAV_LINK_IDS[nextIndex])?.click();
}

document.addEventListener("keydown", function (event) {
  if (event.key === "Tab") {
    event.preventDefault();
    cycleNav(event.shiftKey ? -1 : 1);
    return;
  }

  if (event.key === "ArrowLeft") {
    event.preventDefault();
    window.prevSlide?.();
  } else if (event.key === "ArrowRight") {
    event.preventDefault();
    window.nextSlide?.();
  }
});
