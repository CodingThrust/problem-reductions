import init, { select_paths } from "./path-selection.js";

self.onmessage = async ({data}) => {
  try {
    await init();
    self.postMessage({result: JSON.parse(select_paths(JSON.stringify(data)))});
  } catch (error) {
    self.postMessage({error: error.message});
  }
};
