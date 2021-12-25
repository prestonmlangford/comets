import {System,Body} from "comets";

const Particle = 0;
const Comet = 1;
const Star = 2;
const Planet = 3;

const system = System.new();
const size = 512;

system.add(
  Body.star()
)

system.add(
  Body.comet()
  .mass(1.0)
  .position(0.3,0.3)
  .velocity(0.0,-0.14)
  .build()
)

const canvas = document.getElementById("comets-canvas");
const ctx = canvas.getContext('2d');
const dpr = window.devicePixelRatio;
const w = window.innerWidth;
const h = window.innerHeight;

canvas.width = w;
canvas.height = h;
let start_drag = [0.0,0.0]
canvas.addEventListener('mousedown', e => {
  start_drag = [e.offsetX,e.offsetY];
});

canvas.addEventListener('mousemove',e => {
  
});

canvas.addEventListener('mouseup',e => {
  let scale = 4.0/size;
  let [x0,y0] = start_drag;
  let x1 = e.offsetX;
  let y1 = e.offsetY;
  
  x0 = (2*x0 - w)/size;
  y0 = (2*y0 - h)/size;
  x1 = (2*x1 - w)/size;
  y1 = (2*y1 - h)/size;
  console.log("%f %f %f %f",x0,y0,x1,y1);
  system.add(
    Body.comet()
    .mass(1.0)
    .position(x0,y0)
    .velocity(x0-x1,y0-y1)
    .build()
  )
});

ctx.translate(w/2,h/2);
ctx.scale(dpr,dpr);

const drawSystem = () => {
  
  ctx.clearRect(-size,-size,2*size,2*size);
  
  const n = system.count();
  
  for(let i = 0; i < n; i++)
  {
    const body = system.body(i);
    const m = body.mass();
    const x = body.qx()*size/4;
    const y = body.qy()*size/4;
    const k = body.kind();
    
    switch(k) {
      case Star:
        var gradient = ctx.createRadialGradient(x,y,10,0,0,30);

        // Add three color stops
        gradient.addColorStop(0, 'yellow');
        gradient.addColorStop(1, 'black');

        // Set the fill style and draw a rectangle
        ctx.fillStyle = gradient;
        ctx.fillRect(-size,-size,2*size,2*size);
        break;

      case Comet:
        ctx.strokeStyle = 'cyan';
        ctx.fillStyle = 'cyan';
        let r = Math.sqrt(m);
        ctx.beginPath();
        ctx.moveTo(x+r,y);
        ctx.arc(x,y,r,0.0,2*Math.PI);
        ctx.fill();
        ctx.stroke();
        break;

      case Particle:
        ctx.fillStyle = 'red';
        let s = 1;
        ctx.fillRect(x,y,s,s);
        
        break;
    }
    
  }
}

const renderLoop = () => {
  system.tick();

  drawSystem();

  requestAnimationFrame(renderLoop);
};

drawSystem();
requestAnimationFrame(renderLoop);