import { PerspectiveCamera } from "three/src/cameras/PerspectiveCamera.js";
import { MeshBasicMaterial } from "three/src/materials/MeshBasicMaterial.js";
import { Mesh } from "three/src/objects/Mesh.js";
import { WebGLRenderer } from "three/src/renderers/WebGLRenderer.js";
import { Scene } from "three/src/scenes/Scene.js";

import { FontLoader } from "./three/FontLoader.js";
import { TextGeometry } from "./three/TextGeometry.js";

let previous: DOMHighResTimeStamp;

const scene = new Scene();
const camera = new PerspectiveCamera(40, 25 / 10, 0.1, 1000);
camera.zoom = 0.05;

const canvas = document.getElementById("spinningCanvas") as HTMLCanvasElement;
const renderer = new WebGLRenderer({
  canvas,
  antialias: true,
});
const ratio = window.devicePixelRatio ?? 1;
renderer.setPixelRatio(ratio);
renderer.setClearColor(0xffffff);

const fLoader = new FontLoader();
let text: THREE.Mesh;
fLoader.load("fonts/helvetiker_regular.typeface.json", (font) => {
  const geometry = new TextGeometry("Ivo", {
    size: 5,
    font,
    height: 2.5,
  });
  const material = new MeshBasicMaterial({
    color: 0xff0000,
  });
  text = new Mesh(geometry, material);
  return scene.add(text);
});

camera.position.y = 1.9;
camera.position.z = 15;

function render(now: DOMHighResTimeStamp) {
  const elapsed = now - (previous ?? now);
  previous = now;

  if (text) {
    text.rotation.y += elapsed * 0.006;
  }

  renderer.render(scene, camera);

  requestAnimationFrame(render);
}

requestAnimationFrame(render);
