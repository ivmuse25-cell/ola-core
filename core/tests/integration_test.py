#!/usr/bin/env python3
# tests/integration_test.py
import socket, json, sys, time, os

SOCKET = os.environ.get("TEST_OLA_SOCKET", "/tmp/ola.sock")
TIMEOUT = float(os.environ.get("TEST_TIMEOUT", "5.0"))

def send(method, params=None):
    req = {"id": 1, "method": method, "params": params or {}}
    msg = json.dumps(req) + "\n"

    s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    s.settimeout(TIMEOUT)
    try:
        s.connect(SOCKET)
    except Exception as e:
        return {"error": f"connect_failed: {e}"}
    try:
        s.sendall(msg.encode("utf-8"))
        data = b""
        while True:
            chunk = s.recv(4096)
            if not chunk:
                break
            data += chunk
            if b"\n" in chunk:
                break
        if not data:
            return {"error": "empty_response"}
        line = data.split(b"\n",1)[0]
        return json.loads(line.decode("utf-8"))
    finally:
        s.close()

def assert_ok(resp, name):
    if isinstance(resp, dict) and resp.get("error"):
        print(f"[FAIL] {name}: error field => {resp}")
        sys.exit(2)
    print(f"[OK] {name}: {resp}")

if __name__ == "__main__":
    print("Integration test connecting to:", SOCKET)
    # Give server a second to settle if started just now
    time.sleep(0.5)

    r = send("ping")
    assert_ok(r, "ping")

    r = send("list_cameras")
    assert_ok(r, "list_cameras")

    r = send("capture_thumbnail", {"index": 0})
    assert_ok(r, "capture_thumbnail")

    r = send("verify_once")
    assert_ok(r, "verify_once")

    r = send("status")
    assert_ok(r, "status")

    print("All integration tests passed.")
