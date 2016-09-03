var scene = new THREE.Scene();
var camera = new THREE.PerspectiveCamera( 75, 16 / 9, 0.1, 1000 );
camera.zoom = 0.05;

var renderer = new THREE.WebGLRenderer();
renderer.setSize( window.innerWidth, window.innerWidth * 9 / 16 );
renderer.setClearColor( 0xffffff );
document.body.appendChild( renderer.domElement );

var fLoader = new THREE.FontLoader()
var text;
fLoader.load(
	"fonts/helvetiker_regular.typeface.json",
	function(font) {
		var geometry = new THREE.TextGeometry( "Ivo", {size: 5, font: font, height: 2.5} );
		var material = new THREE.MeshBasicMaterial( { color: 0xff0000 } );
		text = new THREE.Mesh( geometry, material );
		scene.add( text );
	}
)

camera.position.y = 2;
camera.position.z = 11;

function render() {
	requestAnimationFrame( render );

	if (text) {
		text.rotation.y += 0.1;
	}

	renderer.render( scene, camera );
}
render();