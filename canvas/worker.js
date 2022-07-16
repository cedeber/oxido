import init, { well } from "./pkg/canvas.js";

init().then(() => {
  let sab;

  self.onmessage = ({ data }) => {
    if (typeof data !== "string") {
      sab = data;
    } else {
      well(sab, 1720, 800);
      self.postMessage("reload");
    }
  };

  self.postMessage("ready");
});
