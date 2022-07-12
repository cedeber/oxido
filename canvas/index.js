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

let sab = new SharedArrayBuffer(1024);
var typedArr = new Int8Array(sab);
const worker = new Worker(new URL("./worker.js", import.meta.url), { type: "module" });

worker.addEventListener("message", ({ data }) => {
  if (data === "ready") {
    worker.postMessage(sab);
  } else if (data === "reload") {
    console.log("update on main thread?", typedArr);
  }
});
