const slider = document.querySelector('.slider');
const slides = document.querySelectorAll('.slide');
const prevBtn = document.querySelector('.prev');
const nextBtn = document.querySelector('.next');
const linkBtn = document.querySelector('.link-button-inline');
const dots = document.querySelectorAll('.dot');
const sliderContainer = document.querySelector('.slider-container');
const projectName = document.getElementById('project-name');

let currentIndex = 0;

function updateDots() {
    dots.forEach((dot, index) => {
        if (index === currentIndex) {
            dot.classList.add('active');
        } else {
            dot.classList.remove('active');
        }
    });
}

function updateProjectName(index) {
    const name = slides[index].dataset.name;
    projectName.textContent = name;
}

function showSlides(index) {
    if (index >= slides.length) {
        currentIndex = 0;
    } else if (index < 0) {
        currentIndex = slides.length - 1;
    } else {
        currentIndex = index;
    }
    slider.style.transform = `translateX(-${currentIndex * 100}%)`;

    updateProjectName(currentIndex);
    updateDots();
}

function nextSlide() {
    showSlides(currentIndex + 1);
}

function prevSlide() {
    showSlides(currentIndex - 1);
}

function openLink() {
    const link = slides[currentIndex].dataset.link;
    window.open(link, '_blank');
}

dots.forEach(dot => {
    dot.addEventListener('click', () => {
        showSlides(parseInt(dot.dataset.index));
    });
});

nextBtn.addEventListener('click', nextSlide);
prevBtn.addEventListener('click', prevSlide);
linkBtn.addEventListener('click', openLink);

updateProjectName(0);
updateDots();
