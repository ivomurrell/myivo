import * as THREE from "three";

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(40, 25 / 10, 0.1, 1000);
camera.zoom = 0.05;

const canvas = <HTMLCanvasElement>document.getElementById("spinningCanvas");
const renderer = new THREE.WebGLRenderer({
  canvas: canvas,
  antialias: true,
});
const ratio = window.devicePixelRatio != null ? window.devicePixelRatio : 1;
renderer.setPixelRatio(ratio);
renderer.setClearColor(0xffffff);

const fLoader = new THREE.FontLoader();
let text: THREE.Mesh;
fLoader.load("fonts/helvetiker_regular.typeface.json", (font) => {
  const geometry = new THREE.TextGeometry("Ivo", {
    size: 5,
    // incorrect type from FontLoader definition
    font: font as unknown as THREE.Font,
    height: 2.5,
  });
  const material = new THREE.MeshBasicMaterial({
    color: 0xff0000,
  });
  text = new THREE.Mesh(geometry, material);
  return scene.add(text);
});

camera.position.y = 1.9;
camera.position.z = 15;

function render() {
  requestAnimationFrame(render);

  if (text) {
    text.rotation.y += 0.1;
  }

  return renderer.render(scene, camera);
}

render();
