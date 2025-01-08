// @ts-check
/* eslint-env node */
const browserslist = require("browserslist");
const esbuild = require("esbuild");
const { esbuildPluginBrowserslist } = require("esbuild-plugin-browserslist");

const argv = require("minimist")(process.argv.slice(2));

/** @type {esbuild.BuildOptions} */
const baseOptions = {
  entryPoints: ["src/ts/main.ts"],
  bundle: true,
  logLevel: "info",
  plugins: [
    esbuildPluginBrowserslist(browserslist(), { printUnknownTargets: false }),
  ],
};

const devOptions = {
  ...baseOptions,
  sourcemap: true,
  outfile: "build/app.js",
};
if (argv.prod) {
  esbuild
    .build({
      ...baseOptions,
      minify: true,
      outfile: "build/app.min.js",
    })
    .catch(() => process.exit(1));
} else if (argv.serve) {
  esbuild
    .context(devOptions)
    .then((context) => context.serve({ servedir: "." }))
    .then(({ port }) => console.log(`serving at http://localhost:${port}`));
} else {
  esbuild.build(devOptions).catch(() => process.exit(1));
}
