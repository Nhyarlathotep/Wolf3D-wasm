# A Wolf3D clone written in Rust for WebAssembly

This is a small sample project I used for learning Rust and experimenting with Webassembly.

Try the [browser version](https://nhyarlathotep.github.io/Wolf3D-react-editor/#/game) to play with it instantly or compile on your desktop!

![][image-1]

| Controls      | Qwerty  | Azerty  |
| ------------- | ------- | ------- |
| Move Forward  | `W`     | `Z`     |
| Look Left     | `A`     | `Q`     |
| Move Backward | `S`     | `S`     |
| Look Right    | `D`     | `D`     |
| Jump          | `SPACE` | `SPACE` |
| Interact      | `F`     | `F`     |

## How to Build
The package can be built using [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).
```sh
wasm-pack build --realease
```
You can create an application:
```sh
npm init wasm-app my-app
```
Then run `npm link` in the `pkg/` directory and run `npm link wasm` in the root of the application.

Add a canvas in the body of the index.html file:
```html
<head>
  <style>
    #canvas {
        width: 640px;
        height: 480px;
    }
  </style>
</head>
<body>
 ...
<canvas id="canvas"></canvas>
...
</body>
```
And add this code the the index.js
```js
import {Game} from "wasm";

const map = {
  "cells": [],
  "portals": [],
  "sprites": []
};

const game = new Game(map, 320, 240);
window.addEventListener("keydown", function (event) {
    if (!event.defaultPrevented && !event.repeat) {
        game.process_event(event.which, true);
    }
});
window.addEventListener("keyup", function (event) {
    game.process_event(event.which, false);
});

function loop() {
    game.update(1 / 30); //30 FPS
    requestAnimationFrame(loop);
}
renderLoop();

```
Then run:
```sh
npm install
npm start
```

[image-1]:	https://raw.githubusercontent.com/Nhyarlathotep/Wolf3D-react-editor/master/doc/anim.gif
