import init from './pkg/canvas.js'

const loading = document.getElementById('loading')
let previousTime = 0
let rotate = 0
const animate = time => {
  rotate = (rotate + (time - previousTime) * (30 / 100)) % 360
  previousTime = time
  loading.style.transform = `rotate(${rotate}deg)`
  requestAnimationFrame(animate)
}

requestAnimationFrame(animate);

init();

let sab = new SharedArrayBuffer(1024);
const worker = new Worker(new URL('./worker.js', import.meta.url), { type: 'module' })

worker.addEventListener('message', event => {
  console.log(event.data)
})

worker.postMessage(sab);