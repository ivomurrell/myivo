scene = new THREE.Scene()
camera = new THREE.PerspectiveCamera 40, 25 / 10, 0.1, 1000
camera.zoom = 0.05

canvas = document.getElementById "spinningCanvas"
renderer = new THREE.WebGLRenderer {canvas: canvas, antialias: true}
renderer.setClearColor 0xffffff

fLoader = new THREE.FontLoader()
text = null
fLoader.load(
  "fonts/helvetiker_regular.typeface.json"
  (font) ->
    geometry = new THREE.TextGeometry "Ivo", {size: 5, font: font, height: 2.5}
    material = new THREE.MeshBasicMaterial { color: 0xff0000 }
    text = new THREE.Mesh geometry, material
    scene.add text
)

camera.position.y = 1.9
camera.position.z = 15

render = () ->
  requestAnimationFrame render

  text.rotation.y += 0.1 if text

  renderer.render scene, camera

render()