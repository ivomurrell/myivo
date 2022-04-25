// @ts-check
const autoprefixer = require("autoprefixer");
const esbuild = require("esbuild");
const { sassPlugin } = require("esbuild-sass-plugin");
const postcss = require("postcss");

/** @type {esbuild.BuildOptions} */
const baseOptions = {
  entryPoints: ["ts/main.ts"],
  bundle: true,
  logLevel: "info",
  plugins: [
    sassPlugin({
      async transform(source) {
        // @ts-ignore postcss type definition seems to be borked
        const { css } = await postcss([autoprefixer]).process(source);
        return css;
      },
    }),
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
