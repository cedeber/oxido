import init from './pkg/canvas.js'

self.onmessage = event => {console.log(event.data)};

self.postMessage("hello from worker");

init();