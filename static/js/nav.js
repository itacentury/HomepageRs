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
