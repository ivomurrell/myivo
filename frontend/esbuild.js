// @ts-check
import browserslist from "browserslist";
import esbuild from "esbuild";
import { esbuildPluginBrowserslist } from "esbuild-plugin-browserslist";
import minimist from "minimist";

import child_process from "node:child_process";
import http from "node:http";

const argv = minimist(process.argv.slice(2));

/** @type {esbuild.BuildOptions} */
const baseOptions = {
  entryPoints: ["src/ts/main.ts"],
  bundle: true,
  format: "esm",
  logLevel: "info",
  plugins: [
    esbuildPluginBrowserslist(browserslist(), { printUnknownTargets: false }),
  ],
};

const tailwindBin = "node_modules/.bin/tailwindcss";
const tailwindArgs = [
  "-i",
  "src/css/tailwind.css",
  "-o",
  "src/css/tailwind-out.css",
];

const devOptions = {
  ...baseOptions,
  sourcemap: true,
  outfile: "build/app.js",
};
if (argv.serve) {
  child_process.fork(tailwindBin, [...tailwindArgs, "-w"]);

  const context = await esbuild.context(devOptions);
  const { hosts, port } = await context.serve({ servedir: "." });

  const proxyPort = 3000;
  console.log(`serving at http://localhost:${proxyPort}`);
  http
    .createServer((req, res) => {
      const options = {
        path: req.url,
        method: req.method,
        headers: req.headers,
      };
      const url = new URL(`http://localhost${req.url}`);
      const route =
        url.pathname === "/" || url.pathname === "/scrobbles"
          ? { hostname: "127.0.0.1", port: 8080 }
          : { hostname: hosts[0], port };
      const routedOptions = { ...options, ...route };

      const proxyReq = http.request(routedOptions, (proxyRes) => {
        if (proxyRes.statusCode) {
          res.writeHead(proxyRes.statusCode, proxyRes.headers);
        }
        proxyRes.pipe(res, { end: true });
      });
      proxyReq.on("error", (e) => {
        console.error(`problem with request: ${e.message}`);
      });
      req.pipe(proxyReq, { end: true });
    })
    .listen(proxyPort);
} else {
  const tailwindChild = child_process.fork(tailwindBin, tailwindArgs);
  await new Promise((resolve) => tailwindChild.on("exit", resolve));

  if (argv.prod) {
    await esbuild.build({
      ...baseOptions,
      minify: true,
      outfile: "build/app.min.js",
    });
  } else {
    await esbuild.build(devOptions);
  }
}
