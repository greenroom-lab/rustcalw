/**
 * Windows Gateway Smoke Test
 *
 * Phase 1 の疎通テスト: gateway 起動 → WebSocket 接続 → health check → shutdown
 * Windows 上でビルド済み TS 版 OpenClaw が正しく動作することを検証する。
 */
import { afterAll, beforeAll, describe, expect, test } from "vitest";
import {
  startGatewayServerHarness,
  type GatewayServerHarness,
} from "../src/gateway/server.e2e-ws-harness.js";
import { installGatewayTestHooks, onceMessage } from "../src/gateway/test-helpers.js";

installGatewayTestHooks({ scope: "suite" });

const SMOKE_TIMEOUT_MS = 30_000;

type GatewayFrame = {
  type?: string;
  id?: string;
  ok?: boolean;
  event?: string;
  method?: string;
  payload?: Record<string, unknown> | null;
};

let harness: GatewayServerHarness;

beforeAll(async () => {
  harness = await startGatewayServerHarness();
}, SMOKE_TIMEOUT_MS);

afterAll(async () => {
  await harness.close();
});

describe("windows gateway smoke", () => {
  test(
    "gateway starts and responds to health check",
    { timeout: SMOKE_TIMEOUT_MS },
    async () => {
      const { ws } = await harness.openClient();

      const healthP = onceMessage<GatewayFrame>(
        ws,
        (o) => o.type === "res" && o.id === "smoke-health",
      );
      ws.send(JSON.stringify({ type: "req", id: "smoke-health", method: "health" }));

      const health = await healthP;
      expect(health.ok).toBe(true);
      expect(health.type).toBe("res");

      ws.close();
    },
  );

  test(
    "gateway responds to status request",
    { timeout: SMOKE_TIMEOUT_MS },
    async () => {
      const { ws } = await harness.openClient();

      const statusP = onceMessage<GatewayFrame>(
        ws,
        (o) => o.type === "res" && o.id === "smoke-status",
      );
      ws.send(JSON.stringify({ type: "req", id: "smoke-status", method: "status" }));

      const status = await statusP;
      expect(status.ok).toBe(true);

      ws.close();
    },
  );

  test(
    "gateway responds to system-presence request",
    { timeout: SMOKE_TIMEOUT_MS },
    async () => {
      const { ws } = await harness.openClient();

      const presenceP = onceMessage<GatewayFrame>(
        ws,
        (o) => o.type === "res" && o.id === "smoke-presence",
      );
      ws.send(JSON.stringify({ type: "req", id: "smoke-presence", method: "system-presence" }));

      const presence = await presenceP;
      expect(presence.ok).toBe(true);
      expect(Array.isArray(presence.payload)).toBe(true);

      ws.close();
    },
  );
});
