import init, { well } from "./pkg/canvas.js";

init().then(() => {
  let sab;

  self.onmessage = ({ data }) => {
    console.log(data);

    if (typeof data !== "string" && !data.eventName) {
      sab = data;
    } else if (data.eventName === "pointermove") {
      // console.log(data.event);
      well(sab, 1720, 800, Math.floor(data.event.x), Math.floor(data.event.y));
      self.postMessage("reload");
    } else if (!data.eventName) {
      well(sab, 1720, 800, 0, 0);
      self.postMessage("reload");
    }
  };

  self.postMessage("ready");
});
