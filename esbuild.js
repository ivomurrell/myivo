// @ts-check
/* eslint-env node */
const autoprefixer = require("autoprefixer");
const browserslist = require("browserslist");
const esbuild = require("esbuild");
const { esbuildPluginBrowserslist } = require("esbuild-plugin-browserslist");
const { sassPlugin } = require("esbuild-sass-plugin");
const postcss = require("postcss");

const argv = require("minimist")(process.argv.slice(2));

/** @type {esbuild.BuildOptions} */
const baseOptions = {
  entryPoints: ["src/ts/main.ts"],
  bundle: true,
  logLevel: "info",
  plugins: [
    sassPlugin({
      async transform(source, _resolveDir, filePath) {
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-ignore postcss type definition seems to be borked
        const { css } = await postcss([autoprefixer]).process(source, {
          from: filePath,
        });
        return css;
      },
    }),
    esbuildPluginBrowserslist(browserslist(), { printUnknownTargets: false }),
  ],
};

const prodOptions = {
  ...baseOptions,
  minify: true,
  outfile: "build/app.min.js",
};
if (argv.prod) {
  esbuild.build(prodOptions).catch(() => process.exit(1));
} else if (argv.serve) {
  esbuild
    .serve({ servedir: "." }, prodOptions)
    .then(({ port }) => console.log(`serving at http://localhost:${port}`));
} else {
  esbuild
    .build({
      ...baseOptions,
      sourcemap: true,
      outfile: "build/app.js",
    })
    .catch(() => process.exit(1));
}
