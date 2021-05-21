function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function main() {
    const { memory } = await import(
      "/pkg/nes_bg.wasm"
    );
    const {
      WasmEmulator,
      KeyEvent,
    } = await import(
      "/pkg/nes.js"
    );
    const SCREEN_WIDTH = 256;
    const SCREEN_HEIGHT = 240;
    const NUM_OF_COLORS = 3
    const emu = new WasmEmulator();
    emu.reset();
    const rustBuf = new Uint8Array(memory.buffer);
    const fbBasePtr = emu.get_fb_ptr();

    function draw() {
      const canvas = document.getElementById("fb");
      const ctx = canvas.getContext("2d");
      const imageData = ctx.getImageData(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
      for (let j = 0; j < SCREEN_HEIGHT; j++) {
        for (let i = 0; i < SCREEN_WIDTH; i++) {
          const imageDataPtr = j * (SCREEN_WIDTH * 4) + i * 4;
          const rustDataPtr =
            fbBasePtr + j * (SCREEN_WIDTH * NUM_OF_COLORS) + i * NUM_OF_COLORS;
          imageData.data[imageDataPtr + 0] = rustBuf[rustDataPtr + 0]; // red
          imageData.data[imageDataPtr + 1] = rustBuf[rustDataPtr + 1]; // green
          imageData.data[imageDataPtr + 2] = rustBuf[rustDataPtr + 2]; // blue
          imageData.data[imageDataPtr + 3] = 255; //alpha
        }
      }
      ctx.putImageData(imageData, 0, 0);
    }
  
    
    const emulateFps = 60;
    const emulateInterval = 1000.0 / emulateFps;
    let isEmulateEnable = false;
  
   
    function emulate_loop() {
      const start = performance.now()
      if (isEmulateEnable) {
        emu.step_line();
      }
      const elapsed = (performance.now() - start);
      const diffTime = emulateInterval - elapsed;
    
      const sleepTime = diffTime < 0 ? 0 : diffTime;
      setTimeout(emulate_loop, sleepTime);
    }
    
    function draw_loop() {
      if (isEmulateEnable) {
        draw();
      }
      requestAnimationFrame(draw_loop);
    }
    emulate_loop();
    draw_loop();
  
    function release_key(key) {
      if (isEmulateEnable) {
        switch (key) {
          case "j":
            emu.update_key(KeyEvent.ReleaseA);
            break;
          case "k":
            emu.update_key(KeyEvent.ReleaseB);
            break;
          case "u":
            emu.update_key(KeyEvent.ReleaseSelect);
            break;
          case "i":
            emu.update_key(KeyEvent.ReleaseStart);
            break;
          case "w":
            emu.update_key(KeyEvent.ReleaseUp);
            break;
          case "s":
            emu.update_key(KeyEvent.ReleaseDown);
            break;
          case "a":
            emu.update_key(KeyEvent.ReleaseLeft);
            break;
          case "d":
            emu.update_key(KeyEvent.ReleaseRight);
            break;
        }
      }
    }
    function press_key(key) {
      if (isEmulateEnable) {
        switch (key) {
          case "j":
            emu.update_key(KeyEvent.PressA);
            break;
          case "k":
            emu.update_key(KeyEvent.PressB);
            break;
          case "u":
            emu.update_key(KeyEvent.PressSelect);
            break;
          case "i":
            emu.update_key(KeyEvent.PressStart);
            break;
          case "w":
            emu.update_key(KeyEvent.PressUp);
            break;
          case "s":
            emu.update_key(KeyEvent.PressDown);
            break;
          case "a":
            emu.update_key(KeyEvent.PressLeft);
            break;
          case "d":
            emu.update_key(KeyEvent.PressRight);
            break;
        }
      }
    }
  
    ELEMENT.locale("en", ELEMENT.lang.en);
    const app = new Vue({
      el: "#app",
      data: {
        navbarVisible: true,
        loadRomVisible: false,
        keyconfigVisible: false,
        gamepadVisible: false,
        keyconfig: [
          { key: "A", info: "Left" },
          { key: "W", info: "Up" },
          { key: "S", info: "Down" },
          { key: "D", info: "Right" },
          { key: "J", info: "A" },
          { key: "K", info: "B" },
          { key: "U", info: "Select" },
          { key: "I", info: "Start" }
        ]
      },
      methods: {
        romSelect(e) {
          if (e.target.files.length == 0) return;
          const reader = new FileReader();
          reader.onload = file => {
            const arrayBuf = file.target.result;
            const src = new Uint8Array(arrayBuf);
            sleep(1000);
            isEmulateEnable = false;
            
            if (!emu.load(src)) {
             
              this.$notify({
                title: "Load ROM Error"
              });
              return;
            }
           
            const h = this.$createElement;
            this.$notify({
              title: "Load ROM Success",
              message: h("i", { style: "color: teal" }, e.target.files[0].name)
            });
           
            emu.reset();
            isEmulateEnable = true;
          };
         
          reader.readAsArrayBuffer(e.target.files[0]);
        },
        reset() {
         
          if (isEmulateEnable) {
            isEmulateEnable = false;
            emu.reset();
           
            this.$notify({
              title: "Emulator Reset"
            });
           
            isEmulateEnable = true;
          }
        },
        press_key(key) {
          console.log("press", key);
          press_key(key);
        },
        release_key(key) {
          console.log("release", key);
          release_key(key);
        }
      },
      mounted() {
        window.addEventListener("keyup", e => {
          release_key(e.key);
        });
        window.addEventListener("keydown", e => {
          press_key(e.key);
        });
      }
    });
  }
  
  main();