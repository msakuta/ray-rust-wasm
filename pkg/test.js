import init, { render_func } from './helloworld.js'

async function run() {
  await init()

  const canvas = document.getElementById('canvas');
  const canvasSize = canvas.getBoundingClientRect();

  var x = 0;
  var y = -150.;
  var z = -300.;
  var yaw = -90.;
  var pitch = -90.;

  const ctx = canvas.getContext('2d');
  const imageData = ctx.createImageData(canvasSize.width, canvasSize.height);

  function renderCanvas(){
    console.time('Rendering in Rust')
    const buf = render_func(canvasSize.width, canvasSize.height, [x, y, z],
      [0., yaw, pitch].map(deg => deg * Math.PI / 180));
    console.timeEnd('Rendering in Rust')

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

  var label = document.getElementById('label');
  var buttonStates = {
      w: false,
      s: false,
      a: false,
      d: false,
      q: false,
      z: false,
      ArrowRight: false,
      ArrowLeft: false,
      ArrowUp: false,
      ArrowDown: false,
  };
  function updatePos(){
      renderCanvas();
      label.innerHTML = `x=${x}<br>y=${y}<br>z=${z}<br>yaw=${yaw}<br>pitch=${pitch}`;
  }
  function tryUpdate(){
      var ok = false;
      if(buttonStates.a){
          x += 10 * Math.sin(yaw * Math.PI / 180);
          z += 10 * Math.cos(yaw * Math.PI / 180);
          ok = true;
      }
      if(buttonStates.d){
          x -= 10 * Math.sin(yaw * Math.PI / 180);
          z -= 10 * Math.cos(yaw * Math.PI / 180);
          ok = true;
      }
      if(buttonStates.w){
          x += 10 * Math.cos(yaw * Math.PI / 180);
          z -= 10 * Math.sin(yaw * Math.PI / 180);
          ok = true;
      }
      if(buttonStates.s){
          x -= 10 * Math.cos(yaw * Math.PI / 180);
          z += 10 * Math.sin(yaw * Math.PI / 180);
          ok = true;
      }
      if(buttonStates.q){
          y += 10;
          ok = true;
      }
      if(buttonStates.z){
          y -= 10;
          ok = true;
      }
      if(buttonStates.ArrowRight){
          yaw += 5;
          ok = true;
      }
      if(buttonStates.ArrowLeft){
          yaw -= 5;
          ok = true;
      }
      if(buttonStates.ArrowUp){
          pitch -= 5;
          ok = true;
      }
      if(buttonStates.ArrowDown){
          pitch += 5;
          ok = true;
      }
      if(ok){
          updatePos();
          return true;
      }
      return false;
  }
  updatePos();
  window.onkeydown = function(event){
      if(event.key in buttonStates){
          if(!buttonStates[event.key]){
              console.log(`onkeydown x: ${x}, y: ${y}`)
              buttonStates[event.key] = true;
              tryUpdate();
          }
          event.preventDefault();
      }
  }
  window.onkeyup = function(event){
      if(event.key in buttonStates){
          buttonStates[event.key] = false;
          event.preventDefault();
      }
  }
}

run()