import init, {build_emulator} from "/pkg/nes.js";
let displayMemory = 0;
let debug = 0;
let key = 0;
let emulator;
      async function run() {
        await init();
        emulator = build_emulator();
      }
      let started = false;
      const start = () => {
        if (started) return;
        started = true;

        const delay = 400;
        let last = Date.now();
        function mainLoop() {
          if ((Date.now() - last) > delay) {
            emulator.tick();
            last = Date.now();
          }
          requestAnimationFrame(mainLoop);
        }
        requestAnimationFrame(mainLoop);

        function keyboardControls(event) {
          if (event.keyCode === 87) {
            emulator.test();
          }
          last = Date.now();
        }
        document.addEventListener('keydown', keyboardControls);
      };
      run().then(
        document.getElementById("board"),addEventListener("click", start)
      )
/*const run = async() =>{
    const WIDTH =  256;
    const HEIGHT = 240;
    const res = await fetch('nes.wasm');
    const buffer = await res.arrayBuffer();
    const module = await WebAssembly.compile(buffer);
    const instance = await WebAssembly.instantiate(module)
    const exports = instance.exports;

    displayMemory = new Uint32Array(
        exports.memory.buffer,
        exports.get_display(),
        61440
    );
    const canvas = document.getElementById("canvas");
    const ctx = canvas.getContext("2d");
    ctx.translate(0.5, 0.5);
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, WIDTH, HEIGHT);
    exports.put_pixel(128,120, 0x00ff00);
    debug = exports.get_debug();
    const updateDisplay = () => {
        const imageData = ctx.createImageData(WIDTH, HEIGHT);
        for (let i = 0; i < WIDTH * HEIGHT; i++) {
          imageData.data[i * 4] = displayMemory[i] >> 0 & 0xff; //R
          imageData.data[i * 4 + 1] = displayMemory[i] >> 8 & 0xff; //G 
          imageData.data[i * 4 + 2] = displayMemory[i] >> 16 & 0xff; //B
          imageData.data[i * 4 + 3] = 255; // A
        }
        ctx.putImageData(imageData, 0, 0);
      };

      const updateUI = () => {
        //dumpRegisters();
        updateDisplay();
        //updateProgramCounter();
      };
      const getInput = () => {
          
      }
 

      let PC = document.getElementById("PC");
      let i = 0;
      const runloop = () =>{
          i += 1;
          PC.innerHTML = i;
         
          updateUI();
          window.requestAnimationFrame(runloop);
      }
      window.requestAnimationFrame(runloop);
      
}*/

