import init from "./pkg/canvas.js";

const loading = document.getElementById("loading");
let previousTime = 0;
let rotate = 0;
const animate = (time) => {
  rotate = (rotate + (time - previousTime) * (30 / 100)) % 360;
  previousTime = time;
  loading.style.transform = `rotate(${rotate}deg)`;
  requestAnimationFrame(animate);
};

requestAnimationFrame(animate);

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const WIDTH = canvas.width;
const HEIGHT = canvas.height;

let sab = new SharedArrayBuffer(WIDTH * HEIGHT * 4);
var typedArr = new Uint8ClampedArray(sab);
const worker = new Worker(new URL("./worker.js", import.meta.url), { type: "module" });

[
  "mouseenter",
  "mousedown",
  "mousemove",
  "mouseup",
  "mousedown",
  "wheel",
  "contextmenu",
  "pointerdown",
  "pointermove",
  "pointerup",
  "pointercancel",
  "lostpointercapture",
].forEach((eventName) => {
  canvas.addEventListener(eventName, (e) => {
    worker.postMessage({
      eventName,
      event: {
        x: e.clientX,
        y: e.clientY,
        type: e.type,
      },
    });
  });
});

worker.addEventListener("message", ({ data }) => {
  if (data === "ready") {
    worker.postMessage(sab);
    setTimeout(() => {
      worker.postMessage("draw");
    }, 2000);
  } else if (data === "reload") {
    requestAnimationFrame(() => {
      const start = performance.now();
      const imgData = new ImageData(typedArr.slice(0), WIDTH, HEIGHT);
      ctx.putImageData(imgData, 0, 0);
      const end = performance.now();
      console.log("render", `${(end - start).toFixed(0)}ms`);
    });
  }
});
