import init, { render_func } from './pkg/helloworld.js'

async function run() {
  await init()

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
}

run()