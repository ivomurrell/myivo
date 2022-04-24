const esbuild = require('esbuild')

const baseOptions = {
  entryPoints: ['ts/main.ts'],
  bundle: true,
  logLevel: "info"
}

if (process.env.ESBUILD_PRODUCTION) {
  esbuild.build({
    ...baseOptions,
    minify: true,
    outfile: 'build/app.min.js',
  }).catch(() => process.exit(1))
} else {
  esbuild.build({
    ...baseOptions,
    sourcemap: true,
    outfile: 'build/app.js',
  }).catch(() => process.exit(1))
}