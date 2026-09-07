/* Small additions to mdBook's native navigation; content remains usable without JS. */
(() => {
  const root = window.path_to_root || "";
  // mdBook 0.5 uses a label for this control; expose its button behavior to keyboards.
  const toggle = document.querySelector("label#mdbook-sidebar-toggle");
  if (toggle) {
    toggle.setAttribute("role", "button");
    toggle.tabIndex = 0;
    toggle.addEventListener("keydown", (event) => {
      if (event.key === "Enter" || event.key === " ") {
        event.preventDefault();
        toggle.click();
      }
    });
  }
  const page = location.pathname.split("/").pop() || "introduction.html";
  const sidebar = document.querySelector(".sidebar");
  if (sidebar) {
    const brand = document.createElement("a");
    brand.className = "docs-brand";
    brand.href = root + "introduction.html";
    const logo = document.createElement("img");
    logo.src = root + "assets/logo.svg";
    logo.alt = "Problem Reductions";
    brand.setAttribute("aria-label", "Documentation home");
    brand.append(logo);
    sidebar.prepend(brand);
  }
  for (const frame of document.querySelectorAll(".cli-cast")) {
    frame.addEventListener("load", () => {
      const content = frame.contentDocument.querySelector(".wrap");
      if (!content) return;
      const resize = () => {
        frame.style.height = `${Math.ceil(content.getBoundingClientRect().height) + 2}px`;
      };
      new ResizeObserver(resize).observe(content);
      resize();
    });
  }
  const menu = document.querySelector(".menu-bar .right-buttons");
  if (menu && page !== "print.html") {
    const nav = document.createElement("nav");
    nav.className = "docs-tools";
    nav.setAttribute("aria-label", "Documentation resources");
    const links = [
      ["Markdown", "markdown/" + page.replace(/\.html$/, ".md")],
      ["All pages", "markdown/index.md"],
      ["Open atlas ↗", "index.html#atlas"],
    ];
    for (const [label, href] of links) {
      const link = document.createElement("a");
      link.textContent = label;
      link.href = root + href;
      nav.append(link);
    }
    menu.prepend(nav);
    // Resource links replace duplicate repository, edit, and print icon shortcuts.
    for (const child of [...menu.children])
      if (child !== nav) child.hidden = true;
  }
})();
