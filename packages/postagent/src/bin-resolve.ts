import path from "node:path";
import { existsSync } from "node:fs";
import { fileURLToPath } from "node:url";

const PLATFORM_MAP: Record<string, string> = {
  "darwin-arm64": "postagent-core-darwin-arm64",
  "darwin-x64": "postagent-core-darwin-x64",
  "linux-arm64": "postagent-core-linux-arm64",
  "linux-x64": "postagent-core-linux-x64",
  "win32-x64": "postagent-core-win32-x64.exe",
};

export function resolveBinary(): string {
  const __dirname = path.dirname(fileURLToPath(import.meta.url));

  // Dev mode: use local cargo build output from workspace root target/
  if (process.env.POSTAGENT_DEV) {
    // From src/ or dist/: go up to packages/postagent, then up to workspace root
    const workspaceRoot = path.resolve(__dirname, "..", "..", "..");
    const devBinary = path.join(workspaceRoot, "target", "debug", "postagent-core");
    if (existsSync(devBinary)) {
      return devBinary;
    }
    // Also check the package-local target (non-workspace build)
    const localBinary = path.join(
      __dirname,
      "..",
      "..",
      "postagent-core",
      "target",
      "debug",
      "postagent-core",
    );
    if (existsSync(localBinary)) {
      return localBinary;
    }
  }

  const key = `${process.platform}-${process.arch}`;
  const name = PLATFORM_MAP[key];
  if (!name) {
    throw new Error(`Unsupported platform: ${key}`);
  }
  return path.join(__dirname, "..", "bin", name);
}
