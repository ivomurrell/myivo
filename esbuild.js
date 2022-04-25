// @ts-check
/* eslint-env node */
const autoprefixer = require("autoprefixer");
const browserslist = require("browserslist");
const esbuild = require("esbuild");
const { esbuildPluginBrowserslist } = require("esbuild-plugin-browserslist");
const { sassPlugin } = require("esbuild-sass-plugin");
const postcss = require("postcss");

/** @type {esbuild.BuildOptions} */
const baseOptions = {
  entryPoints: ["src/ts/main.ts"],
  bundle: true,
  logLevel: "info",
  plugins: [
    sassPlugin({
      async transform(source) {
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-ignore postcss type definition seems to be borked
        const { css } = await postcss([autoprefixer]).process(source);
        return css;
      },
    }),
    esbuildPluginBrowserslist(browserslist(), { printUnknownTargets: false }),
  ],
};

if (process.env.ESBUILD_PRODUCTION) {
  esbuild
    .build({
      ...baseOptions,
      minify: true,
      outfile: "build/app.min.js",
    })
    .catch(() => process.exit(1));
} else {
  esbuild
    .build({
      ...baseOptions,
      sourcemap: true,
      outfile: "build/app.js",
    })
    .catch(() => process.exit(1));
}
