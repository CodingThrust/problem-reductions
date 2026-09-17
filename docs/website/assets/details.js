(() => {
  "use strict";
  const detailCache = new Map();
  let manifest;
  let renderRequest = 0;
  function loadDetail(key) {
    const url = window.GRAPH_DETAILS.entries[key];
    if (!url) return Promise.reject(new Error(`Missing detail file: ${key}`));
    if (!detailCache.has(url)) {
      detailCache.set(url, fetch(url).then((response) => {
        if (!response.ok) throw new Error(`Cannot load documentation (${response.status})`);
        return response.text();
      }).catch((error) => {
        detailCache.delete(url);
        throw error;
      }));
      if (detailCache.size > 24) detailCache.delete(detailCache.keys().next().value);
    } else {
      const cached = detailCache.get(url);
      detailCache.delete(url);
      detailCache.set(url, cached);
    }
    return detailCache.get(url);
  }

  window.renderDetails = async function(detail, title, variant, key, parentKey, onProblem) {
    const request = ++renderRequest;
    detail.replaceChildren();
    detail.onclick = (event) => {
      const link = event.target.closest('a[href^="#"]');
      if (!link) return;
      const anchor = link.hash.slice(1);
      const key = window.GRAPH_DETAILS.anchors[anchor];
      if (key?.startsWith("problem:") && key !== detail.querySelector("article")?.dataset.contentKey) {
        event.preventDefault();
        onProblem(key.slice("problem:".length));
        return;
      }
      if (link.getAttribute("role") === "doc-biblioref") {
        event.preventDefault();
        const references = detail.querySelector(".detail-references");
        references.open = true;
        references.scrollIntoView({ block: "nearest" });
        return;
      }
      const target = detail.querySelector(`[id="${CSS.escape(link.hash.slice(1))}"]`);
      if (!target) {
        if (key === detail.querySelector("article")?.dataset.contentKey) {
          event.preventDefault();
          detail.scrollIntoView({ block: "start" });
        }
        return;
      }
      event.preventDefault();
      const references = target.closest("details");
      if (references) references.open = true;
      target.scrollIntoView({ block: "nearest" });
    };

    if (title) {
      const heading = document.createElement("h2");
      heading.textContent = title;
      detail.append(heading);
    }
    if (variant) {
      const parameters = document.createElement("p");
      parameters.className = "detail-variant";
      parameters.textContent = variant;
      detail.append(parameters);
    }
    const article = document.createElement("article");
    article.className = "typst-detail";
    article.textContent = "Loading documentation…";
    detail.append(article);
    if (!window.GRAPH_DETAILS) {
      try {
        manifest ||= fetch("./assets/graph-details.json").then(response => {
          if (!response.ok) throw new Error("Cannot load documentation index. Reload the page to get the current release.");
          return response.json();
        });
        window.GRAPH_DETAILS = await manifest;
      } catch (error) {
        manifest = null;
        if (request === renderRequest) {
          article.textContent = error.message;
          article.setAttribute("role", "alert");
        }
        return;
      }
    }
    if (request !== renderRequest || !article.isConnected) return;
    const contentKey = window.GRAPH_DETAILS.entries[key] ? key : parentKey;
    const available = Boolean(window.GRAPH_DETAILS.entries[contentKey]);
    if (!available) {
      const note = document.createElement("p");
      note.className = "detail-source";
      note.textContent = "No documentation is available for this entry or its parent family.";
      detail.append(note);
    }
    article.dataset.contentKey = contentKey;
    if (available) {
      article.textContent = "Loading documentation…";
      article.setAttribute("aria-busy", "true");
    }
    if (!available) { article.remove(); return; }
    try {
      const content = await loadDetail(contentKey);
      if (!article.isConnected) return;
      // HTML is generated at build time from the repository's Typst source.
      article.innerHTML = content;
      const citations = new Set([...article.querySelectorAll('a[role="doc-biblioref"]')]
        .map((link) => link.hash.slice(1)));
      if (citations.size) {
        const references = document.createElement("details");
        references.className = "detail-references";
        references.innerHTML = "<summary>References</summary><div></div>";
        article.after(references);
        const body = references.lastElementChild;
        let loaded = false;
        references.addEventListener("toggle", async () => {
          if (!references.open || loaded) return;
          body.textContent = "Loading references…";
          try {
            body.innerHTML = await loadDetail("references");
            body.querySelectorAll("li").forEach((item) => {
              if (!citations.has(item.id)) item.remove();
            });
            loaded = true;
          } catch (error) {
            body.textContent = error.message;
            body.setAttribute("role", "alert");
          }
        });
      }
      if (article.querySelector('[role="doc-noteref"] a')) {
        const notes = document.createElement("aside");
        notes.className = "typst-detail detail-footnotes";
        try {
          notes.innerHTML = await loadDetail("footnotes");
          const used = new Set([...article.querySelectorAll('[role="doc-noteref"] a')].map((link) => link.hash.slice(1)));
          notes.querySelectorAll('li').forEach((item) => {
            if (!used.has(item.id)) item.remove();
          });
        } catch (error) {
          // The article is already rendered; only its footnotes degrade.
          notes.textContent = error.message;
          notes.setAttribute("role", "alert");
        }
        if (!article.isConnected) return;
        article.after(notes);
      }
    } catch (error) {
      if (!article.isConnected) return;
      article.textContent = error.message;
      article.setAttribute("role", "alert");
    } finally {
      article.removeAttribute("aria-busy");
    }
  }

})();
