import init, {build_emulator} from "/pkg/nes.js";
let displayMemory = 0;
let ff = 0;
let key = 0;
let rom = 0;
let emulator;
      async function run(file) {
        await init();
        emulator = build_emulator();
        rom = new Uint8Array(file);
        emulator.loadRom(rom);  
      }
      let started = false;
      const start = () => {
        if (started) return;
        started = true;  
        const delay = 400;
        let last = Date.now();
        function mainLoop() {
          if ((Date.now() - last) > delay) {
            emulator.tick(rom != 0); //Start running when the ROM is loaded
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

var fileInput = document.getElementById("romload");
fileInput.addEventListener('change', function(e){
  openFile(e);
  });
var openFile = function(event) {
  var input = event.target;

  var reader = new FileReader();
  reader.onload = function(){
    var dataURL = reader.result;
    run(reader.result);
  };
  reader.readAsArrayBuffer(input.files[0]);
};