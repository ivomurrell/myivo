[parallel]
serve: serve-js serve-rs

[working-directory: 'frontend']
serve-js:
	npm start

[working-directory: 'server']
serve-rs $RUST_LOG=env('RUST_LOG', 'debug'):
	watchexec --restart --ignore "target/**" cargo run
