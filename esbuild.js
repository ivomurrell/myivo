// @ts-check
const esbuild = require('esbuild')
const sassPlugin = require("esbuild-plugin-sass");

/** @type {esbuild.BuildOptions} */
const baseOptions = {
  entryPoints: ['ts/main.ts', "sass/main.scss"],
  bundle: true,
  logLevel: "info",
  outdir: 'build',
  plugins: [sassPlugin()]
}

if (process.env.ESBUILD_PRODUCTION) {
  esbuild.build({
    ...baseOptions,
    minify: true,
  }).catch(() => process.exit(1))
} else {
  esbuild.build({
    ...baseOptions,
    sourcemap: true,
  }).catch(() => process.exit(1))
}