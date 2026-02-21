const navLinks = document.querySelectorAll(".nav__link");
const sections = document.querySelectorAll(".content-section");

navLinks.forEach((link) => {
  link.addEventListener("click", (e) => {
    e.preventDefault();

    navLinks.forEach((l) => l.classList.remove("active"));
    link.classList.add("active");

    sections.forEach((s) => s.classList.remove("active"));

    const targetId = link.getAttribute("data-section");
    document.getElementById(targetId)?.classList.add("active");
  });
});

navLinks[0]?.click();

// Tab / Shift+Tab cycles through nav sections
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
  }
});
