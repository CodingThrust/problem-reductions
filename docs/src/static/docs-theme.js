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
  const legacy = {
    "cli.html": {
      installation: "install.html",
      "ilp-backend": "install.html#optional-features",
      "quick-start": "cli.html",
      "global-flags": "cli-automation.html",
      commands: "cli-catalog.html",
      "shell-completions": "cli-completions.html",
      "json-output": "cli-automation.html",
      "problem-name-aliases": "cli-variants.html",
      "pred-list--list-all-problem-types": "cli-catalog.html",
      "pred-show--inspect-a-problem": "cli-catalog.html",
      "pred-to--explore-incoming-neighbors": "cli-paths.html",
      "pred-from--explore-outgoing-neighbors": "cli-paths.html",
      "pred-path--find-a-reduction-path": "cli-paths.html",
      "pred-export-graph--export-the-reduction-graph":
        "cli-automation.html#registry-exports",
      "pred-create--create-a-problem-instance": "cli-create.html",
      "pred-evaluate--evaluate-a-configuration": "cli-inspect.html",
      "pred-inspect--inspect-a-problem-file": "cli-inspect.html",
      "pred-reduce--reduce-a-problem": "cli-reduce.html",
      "pred-solve--solve-a-problem": "cli-solve.html",
    },
    "introduction.html": {
      "reduction-graph": "reduction-graph.html",
      "our-vision": "introduction.html#research-scope",
      "call-for-contributions": "contributing.html",
      authorship: "contributing.html#authorship-and-license",
    },
    "design.html": {
      "module-architecture": "design.html#module-map",
      "problem-model": "design-problem.html",
      "variant-system": "design-variants.html",
      "reduction-rules": "design-reductions.html",
      "reduction-graph": "design-paths.html",
      "path-finding": "design-paths.html",
      "executable-paths": "rust-paths.html",
      solvers: "rust-solvers.html",
      "json-serialization": "design-serialization.html",
      contributing: "contributing.html",
    },
    "getting-started.html": {
      installation: "getting-started.html",
      solvers: "rust-solvers.html",
      "the-reduction-workflow": "rust-reduction.html",
      "json-resources": "cli-automation.html#registry-exports",
      "example-1-direct-reduction--set-packing-to-ilp": "rust-reduction.html",
      "example-2-reduction-path-search--integer-factoring-to-spin-glass":
        "rust-paths.html",
    },
    "mcp.html": {
      setup: "mcp.html#install-with-mcp-support",
      walkthrough: "mcp-walkthrough.html",
      "available-tools": "mcp-tools.html",
      "graph-query-tools": "mcp-tools.html#graph-queries",
      "instance-tools": "mcp-tools.html#instances",
      "available-prompts": "mcp-tools.html#prompt-templates",
    },
  };
  const redirect = legacy[page]?.[location.hash.slice(1)];
  if (redirect) {
    location.replace(root + redirect);
    return;
  }
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
