import socket
import json
import sys
import os
import time

# Default to systemd path, but allow override for dev
SOCKET_PATH = os.environ.get("OLA_SOCKET", "/run/ola/ola.sock")
DEFAULT_TIMEOUT = float(os.environ.get("OLA_CLIENT_TIMEOUT", "5.0"))

import itertools

_id_gen = itertools.count(1)

class OlaClient:
    def __init__(self, socket_path=SOCKET_PATH):
        self.socket_path = socket_path

    def _send(self, method, params=None):
        req = {
            "id": next(_id_gen),
            "method": method,
            "params": params or {}
        }
        
        s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        s.settimeout(DEFAULT_TIMEOUT) # 2 second timeout

        try:
            s.connect(self.socket_path)
        except Exception as e:
            # Standardize error response
            return {"id": req.get("id"), "result": None, "error": f"Connection failed: {str(e)}"}

        try:
            # Send newline-delimited JSON
            msg = json.dumps(req) + "\n"
            s.sendall(msg.encode("utf-8"))
            
            # Read response (line-based)
            data = b""
            while True:
                chunk = s.recv(4096)
                if not chunk:
                    break
                data += chunk
                if b"\n" in chunk:
                    break
            
            if not data:
                 return {"id": req["id"], "result": None, "error": "Empty response from server"}
            
            line = data.split(b"\n", 1)[0]
            return json.loads(line.decode("utf-8"))
            
        except socket.timeout:
             return {"id": req["id"], "result": None, "error": "Request timed out"}
        except Exception as e:
             return {"id": req["id"], "result": None, "error": f"Client error: {e}"}
        finally:
            s.close()

    def ping(self):
        return self._send("ping")

    def list_cameras(self):
        return self._send("list_cameras")

    def capture_thumbnail(self, index=0):
        return self._send("capture_thumbnail", {"index": index})

    def verify_once(self):
        return self._send("verify_once")

    def status(self):
        return self._send("status")

def main():
    client = OlaClient()
    if len(sys.argv) > 1:
        cmd = sys.argv[1]
        if cmd == "ping":
            print(client.ping())
        elif cmd == "list_cameras":
            print(client.list_cameras())
        elif cmd == "capture_thumbnail":
            idx = int(sys.argv[2]) if len(sys.argv) > 2 else 0
            print(client.capture_thumbnail(idx))
        elif cmd == "verify_once":
            print(client.verify_once())
        elif cmd == "status":
            print(client.status())
        else:
            print(f"Unknown command: {cmd}")
    else:
        print("Usage: python3 ola_client.py [ping|list_cameras|capture_thumbnail|verify_once|status]")

if __name__ == "__main__":
    main()
