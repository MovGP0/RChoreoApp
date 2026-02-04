import http.server
import socketserver
import subprocess
import threading
import webbrowser
from functools import partial
from pathlib import Path


def main() -> int:
    root_dir = Path(__file__).resolve().parent.parent
    wasm_dir = root_dir / "apps" / "wasm"

    subprocess.run(
        [
            "wasm-pack",
            "build",
            "--release",
            "--target",
            "web",
            "apps/wasm",
        ],
        cwd=root_dir,
        check=True,
    )

    port = 8000
    handler = partial(http.server.SimpleHTTPRequestHandler, directory=str(wasm_dir))

    with socketserver.TCPServer(("127.0.0.1", port), handler) as httpd:
        def open_browser() -> None:
            webbrowser.open(f"http://127.0.0.1:{port}/index.html", new=1)

        threading.Thread(target=open_browser, daemon=True).start()
        print(f"Serving {wasm_dir} on http://127.0.0.1:{port}/index.html")
        httpd.serve_forever()

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
