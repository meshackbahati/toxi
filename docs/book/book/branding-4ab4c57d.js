(() => {
  const root = typeof window.path_to_root === "string" ? window.path_to_root : "";

  function addBrand() {
    const leftButtons = document.querySelector("#mdbook-menu-bar .left-buttons");
    if (!leftButtons || leftButtons.querySelector(".toxi-docs-brand")) {
      return;
    }

    const brandLink = document.createElement("a");
    brandLink.className = "toxi-docs-brand";
    brandLink.href = `${root}index.html`;
    brandLink.setAttribute("aria-label", "Toxi Documentation Home");

    const logo = document.createElement("img");
    logo.src = `${root}assets/toxi.svg`;
    logo.alt = "Toxi logo";
    logo.loading = "eager";

    const label = document.createElement("span");
    label.textContent = "Toxi";

    brandLink.appendChild(logo);
    brandLink.appendChild(label);
    leftButtons.insertBefore(brandLink, leftButtons.firstChild);
  }

  function updateLabels() {
    const title = document.querySelector("#mdbook-menu-bar .menu-title");
    if (title) {
      title.textContent = "Toxi Documentation";
    }

    const search = document.getElementById("mdbook-searchbar");
    if (search) {
      search.setAttribute("placeholder", "Search Toxi documentation ...");
    }
  }

  document.addEventListener("DOMContentLoaded", () => {
    addBrand();
    updateLabels();
  });
})();
