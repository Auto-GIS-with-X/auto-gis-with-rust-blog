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
    const dropdownButtons = document.querySelectorAll(".dropdown__button");
    const allDropdownItems = document.querySelectorAll(".dropdown__items");
    const dropdownIcons = document.querySelectorAll(".dropdown__expand");
    dropdownButtons.forEach(dropdownButton => {
        dropdownButton.addEventListener("click", () => {
            const controls = dropdownButton.getAttribute("aria-controls");
            const dropdownItems = document.querySelector(`#${controls}`);
            allDropdownItems.forEach(_dropdownItems => {
                if (_dropdownItems != dropdownItems && _dropdownItems.classList.contains("dropdown__items--open")) {
                    _dropdownItems.classList.remove("dropdown__items--open");
                }
            });
            dropdownItems.classList.toggle("dropdown__items--open");

            const dropdownIcon = document.querySelector(`#${controls.replace("items", "expand")}`);
            dropdownIcons.forEach(_dropdownIcon => {
                if (_dropdownIcon != dropdownIcon && _dropdownIcon.classList.contains("dropdown__expand--open")) {
                    _dropdownIcon.classList.remove("dropdown__expand--open");
                }
            });
            dropdownIcon.classList.toggle("dropdown__expand--open");
        });
    });
}

initialiseDropdown();