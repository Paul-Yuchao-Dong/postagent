import { spawn } from "node:child_process";
import { resolveBinary } from "./bin-resolve.js";

const binary = resolveBinary();
const child = spawn(binary, process.argv.slice(2), {
  stdio: "inherit",
});

child.on("close", (code) => {
  process.exitCode = code ?? 1;
});
