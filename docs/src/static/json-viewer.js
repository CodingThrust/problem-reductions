// Simple JSON tree viewer for mdBook
// Converts JSON code blocks into interactive collapsible trees

(function() {
    function createJsonTree(obj, indent = 0) {
        const type = Array.isArray(obj) ? 'array' : typeof obj;

        if (type === 'object' && obj !== null) {
            const entries = Object.entries(obj);
            if (entries.length === 0) return '<span class="json-empty">{}</span>';

            const items = entries.map(([key, value]) => {
                const valueHtml = createJsonTree(value, indent + 1);
                return `<div class="json-item" style="margin-left: ${indent * 16}px">
                    <span class="json-key">"${key}"</span>: ${valueHtml}
                </div>`;
            }).join('');

            return `<span class="json-toggle" onclick="this.parentElement.classList.toggle('collapsed')">{...}</span>
                <div class="json-content">${items}</div>`;
        } else if (type === 'array') {
            if (obj.length === 0) return '<span class="json-empty">[]</span>';

            const items = obj.map((value, i) => {
                const valueHtml = createJsonTree(value, indent + 1);
                return `<div class="json-item" style="margin-left: ${indent * 16}px">
                    <span class="json-index">${i}</span>: ${valueHtml}
                </div>`;
            }).join('');

            return `<span class="json-toggle" onclick="this.parentElement.classList.toggle('collapsed')">[${obj.length}]</span>
                <div class="json-content">${items}</div>`;
        } else if (type === 'string') {
            return `<span class="json-string">"${obj}"</span>`;
        } else if (type === 'number') {
            return `<span class="json-number">${obj}</span>`;
        } else if (type === 'boolean') {
            return `<span class="json-boolean">${obj}</span>`;
        } else {
            return `<span class="json-null">null</span>`;
        }
    }

    function initJsonViewer() {
        // Find all JSON code blocks inside <details> elements
        document.querySelectorAll('details pre code.language-json').forEach(block => {
            try {
                const json = JSON.parse(block.textContent);
                const container = document.createElement('div');
                container.className = 'json-tree';
                container.innerHTML = createJsonTree(json);

                // Replace the pre element with the tree
                const pre = block.parentElement;
                pre.parentElement.insertBefore(container, pre);
                pre.style.display = 'none';

                // Add toggle to show raw JSON
                const toggle = document.createElement('button');
                toggle.className = 'json-raw-toggle';
                toggle.textContent = 'Show raw JSON';
                toggle.onclick = () => {
                    if (pre.style.display === 'none') {
                        pre.style.display = 'block';
                        container.style.display = 'none';
                        toggle.textContent = 'Show tree view';
                    } else {
                        pre.style.display = 'none';
                        container.style.display = 'block';
                        toggle.textContent = 'Show raw JSON';
                    }
                };
                container.parentElement.insertBefore(toggle, container);
            } catch (e) {
                // Not valid JSON, leave as-is
            }
        });
    }

    // Run after DOM is ready
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initJsonViewer);
    } else {
        initJsonViewer();
    }
})();
