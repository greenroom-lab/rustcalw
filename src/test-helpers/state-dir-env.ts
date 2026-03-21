import fsSync from "node:fs";
import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";
import { captureEnv } from "../test-utils/env.js";

export function snapshotStateDirEnv() {
  return captureEnv(["OPENCLAW_STATE_DIR", "CLAWDBOT_STATE_DIR"]);
}

export function restoreStateDirEnv(snapshot: ReturnType<typeof snapshotStateDirEnv>): void {
  snapshot.restore();
}

export function setStateDirEnv(stateDir: string): void {
  process.env.OPENCLAW_STATE_DIR = stateDir;
  delete process.env.CLAWDBOT_STATE_DIR;
}

export async function withStateDirEnv<T>(
  prefix: string,
  fn: (ctx: { tempRoot: string; stateDir: string }) => Promise<T>,
): Promise<T> {
  const snapshot = snapshotStateDirEnv();
  const tempRoot = await fs.mkdtemp(path.join(os.tmpdir(), prefix));
  const stateDir = path.join(tempRoot, "state");
  await fs.mkdir(stateDir, { recursive: true });
  setStateDirEnv(stateDir);
  try {
    return await fn({ tempRoot, stateDir });
  } finally {
    restoreStateDirEnv(snapshot);
    // On Windows the vitest worker process can hold file handles open
    // inside the temp tree (e.g. via loadSubagentRegistryFromDisk),
    // causing the async fs.rm to block indefinitely.  Use synchronous
    // rmSync which completes without awaiting the event loop; ignore
    // errors from locked files -- the OS reclaims temp dirs on reboot.
    try {
      fsSync.rmSync(tempRoot, { recursive: true, force: true });
    } catch {
      // Best-effort: ignore EBUSY / EPERM on Windows.
    }
  }
}
