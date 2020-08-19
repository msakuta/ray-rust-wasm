import init, { fibonacci, render_func } from './pkg/helloworld.js'

function fibonacciInJs(n) {
  if (n <= 1) return n
  return fibonacciInJs(n - 1) + fibonacciInJs(n - 2)
}

async function run() {
  await init()
  const num = 20

  console.time('Fibonnaci in rust')
  const fibRust = fibonacci(num)
  console.timeEnd('Fibonnaci in rust')

  console.time('Fibonnaci in JS')
  const fibJS = fibonacciInJs(num)
  console.timeEnd('Fibonnaci in JS')

  const canvas = document.getElementById('canvas');
  const canvasSize = canvas.getBoundingClientRect();

  console.time('Rendering in Rust')
  const buf = render_func(canvasSize.width, canvasSize.height);
  console.timeEnd('Rendering in Rust')

  console.log(buf)

    const ctx = canvas.getContext('2d');
    const imageData = ctx.createImageData(canvasSize.width, canvasSize.height);

    // Iterate through every pixel
    for (let i = 0; i < imageData.data.length; i += 1) {
    // Modify pixel data
    imageData.data[4 * i + 0] = buf[3 * i + 0];  // R value
    imageData.data[4 * i + 1] = buf[3 * i + 1];    // G value
    imageData.data[4 * i + 2] = buf[3 * i + 2];  // B value
    imageData.data[4 * i + 3] = 255;  // A value
    }

    // Draw image data to the canvas
    ctx.putImageData(imageData, 0, 0);

  document.getElementById("text").innerHTML = `Fib ${num}:  Rust ${fibRust} - JS ${fibJS}`
}

run()