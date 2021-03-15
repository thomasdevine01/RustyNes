import init, {build_emulator} from "/pkg/nes.js"
let displayMemory = 0;
let ff = 0;
let key = 0;

let emulator;
      async function run(file) {
        const {
          Emulator
        } = await import("/pkg/nes.js");
        await init();
        emulator = build_emulator();
      //  rom = new Uint8Array(file);
        //emulator.loadRom(rom);  
      }
      
      let started = false;
      const start = () => {
        const delay = 400;
        let last = Date.now();
        function mainLoop() {
          if ((Date.now() - last) > delay) {
            emulator.tick(); //Start running when the ROM is loaded
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
  if(event.target.files.length == 0){
    return;
  }
  var input = event.target;
  var reader = new FileReader();
  reader.onload = file => {
    var arrBuff = file.target.result;
    console.log(file.target.result)
    const src = new Uint8Array(arrBuff);
    console.log(src);
    if(!emulator.loadRom(src)){
      console.log("Rom load failed");
      return;
    }
    console.log("Success");
    started = true;
  }
  reader.readAsArrayBuffer(event.target.files[0]);
};