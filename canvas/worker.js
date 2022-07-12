import init, { well } from "./pkg/canvas.js";

init().then(() => {
  self.onmessage = ({ data }) => {
    // console.log("worker", data);

    const arr = new Int8Array(data);
    // console.log(data, arr);

    arr[5] = 4;

    well(data);

    self.postMessage("reload");
  };

  self.postMessage("ready");
});
