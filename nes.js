import init, {build_emulator} from "/pkg/nes.js"
let displayMemory = 0;
let ff = 0;
let key = 0;
let regpc = document.getElementById("pc");
let regsp = document.getElementById("sp");
let regx = document.getElementById("x");
let regy = document.getElementById("y");
let rega = document.getElementById("a");
let regr2 = document.getElementById("r2");
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
        const delay = 40;
        let last = Date.now();
        function mainLoop() {
          if ((Date.now() - last) > delay) {
            emulator.tick(); //Start running when the ROM is loaded
            regpc.innerHTML = emulator.status(0);
            regsp.innerHTML = emulator.status(1);
            regx.innerHTML = emulator.status(2);
            regy.innerHTML = emulator.status(3);
            rega.innerHTML = emulator.status(4);
            

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
    if(!emulator.load_rom(src)){
      console.log("Rom load failed");
      return;
    }
    console.log("Success");
    started = true;
  }
  reader.readAsArrayBuffer(event.target.files[0]);
};