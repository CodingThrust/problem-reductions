# Website

Home/Atlas and the reduction graph use the Rust registry, WASM engine and Typst
articles. Page code lives in `src/pages/`, the engine worker in `src/workers/`,
and styles in `src/styles/`. Static data is validated during the build; generated
TypeScript types describe the data consumed by the browser.

Build and serve locally:

```sh
make website
python3 -m http.server 3001 --directory book
```

Frontend checks:

```sh
npm ci
npm run typecheck
npm run lint:frontend
npm run test:frontend-contracts
```

Types and Rust schemas are regenerated automatically by the checks and website
build into the ignored `src/generated/` directory. The five static-data schemas
are maintained here; build-only validation runs in `scripts/website_contracts.mjs`.

For a fresh production build tested in Chromium, Firefox and WebKit:

```sh
uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt python -m playwright install chromium firefox webkit
uv run --python 3.12.14 --no-project --with-requirements scripts/website-requirements.txt python scripts/check_website.py
```

The verifier prints its temporary output directory. Use `--output /tmp/new-site`
to choose one, or `--site /path/to/site` to test an existing build. The optional
Open Question Board is supplied with `--open-questions /path/to/board`.
Deploy the complete generated site so HTML, data, scripts and WASM stay together.
