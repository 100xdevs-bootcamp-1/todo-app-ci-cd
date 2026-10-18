import { afterAll, beforeAll, describe, expect, test } from "bun:test";
import type { Server } from "node:http";
import type { AddressInfo } from "node:net";
import { createApp } from "./app";

let server: Server;
let base: string;

beforeAll(() => {
  server = createApp().listen(0);
  base = `http://localhost:${(server.address() as AddressInfo).port}`;
});

afterAll(() => server.close());

const json = (method: string, body?: unknown) => ({
  method,
  headers: { "content-type": "application/json" },
  body: body === undefined ? undefined : JSON.stringify(body),
});

describe("todo api", () => {
  test("health", async () => {
    const res = await fetch(`${base}/health`);
    expect(res.status).toBe(200);
  });

  test("create, list, update, delete", async () => {
    const created = await fetch(`${base}/todos`, json("POST", { title: "write ci" }));
    expect(created.status).toBe(201);
    const todo = await created.json();
    expect(todo).toEqual({ id: 1, title: "write ci", done: false });

    const list = await (await fetch(`${base}/todos`)).json();
    expect(list).toHaveLength(1);

    const updated = await (await fetch(`${base}/todos/1`, json("PATCH", { done: true }))).json();
    expect(updated.done).toBe(true);

    const deleted = await fetch(`${base}/todos/1`, { method: "DELETE" });
    expect(deleted.status).toBe(204);
    expect(await (await fetch(`${base}/todos`)).json()).toEqual([]);
  });

  test("rejects empty title", async () => {
    const res = await fetch(`${base}/todos`, json("POST", { title: "" }));
    expect(res.status).toBe(400);
  });

  test("404 on missing todo", async () => {
    const res = await fetch(`${base}/todos/999`, { method: "DELETE" });
    expect(res.status).toBe(404);
  });
});
