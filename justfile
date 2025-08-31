set dotenv-load := true

[parallel]
serve: serve-js serve-rs

[working-directory: 'frontend']
serve-js:
	watchexec --restart --watch esbuild.js npm start

[working-directory: 'server']
serve-rs $RUST_LOG=env('RUST_LOG', 'debug,selectors=warn,html5ever=warn'):
	watchexec --restart --ignore "target/**" cargo run
