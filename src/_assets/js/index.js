const initialiseHamburger = () => {
    hamburgerButton = document.querySelector("#hamburger-button");
    hamburgerButton.addEventListener("click", (e) => {
        const siteNavItems = document.querySelector("#site-nav-items");
        siteNavItems.classList.toggle("site-nav__items--open");
        hamburgerButton.classList.toggle("hamburger--open")
    });
}

initialiseHamburger();

const initialiseDropdown = () => {
    dropdownButton = document.querySelector("#problems-dropdown-button");
    dropdownButton.addEventListener("click", (e) => {
        const dropdownItems = document.querySelector("#problems-dropdown-items");
        dropdownItems.classList.toggle("dropdown__items--open");
        const dropdownIcon = document.querySelector("#problems-dropdown-expand");
        dropdownIcon.classList.toggle("dropdown__expand--open");
    });
}

initialiseDropdown();