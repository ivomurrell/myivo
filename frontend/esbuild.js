// @ts-check
/* eslint-env node */
import browserslist from "browserslist";
import esbuild from "esbuild";
import { esbuildPluginBrowserslist } from "esbuild-plugin-browserslist";
import minimist from "minimist";

const argv = minimist(process.argv.slice(2));

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
  await esbuild.build({
    ...baseOptions,
    minify: true,
    outfile: "build/app.min.js",
  });
} else if (argv.serve) {
  const context = await esbuild.context(devOptions);
  const { port } = await context.serve({ servedir: "." });
  console.log(`serving at http://localhost:${port}`);
} else {
  await esbuild.build(devOptions);
}
