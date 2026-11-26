#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-or-later
"""Checks the built website: no script and nothing loaded from another domain.

The site promises to run no code in the visitor's browser and to tell no third
party about the visit. This check fails on:
- a <script>, <iframe>, <frame>, <embed>, <object> or <applet> element;
- an event handler attribute (onclick and the like) or a javascript: URL;
- a JavaScript or WebAssembly file;
- a resource (src, srcset, poster, data, <link href>, SVG href, CSS url() or
  @import, meta refresh) on a host other than the site's own.
Plain links (<a href>) to other sites are allowed: following one is the
visitor's choice.

The site's own host is read from base_url in zola.toml. Standard library only.

Usage: scripts/check-site.py <site dir, holding zola.toml and public/>
"""

import re
import sys
import tomllib
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import urlsplit

FORBIDDEN_TAGS = {"script", "iframe", "frame", "embed", "object", "applet"}
FORBIDDEN_FILES = {".js", ".mjs", ".cjs", ".wasm"}
# Attributes that make the browser fetch something, whatever the element.
RESOURCE_ATTRS = {"src", "poster", "data", "background", "xlink:href"}
CSS_URL = re.compile(r"""url\(\s*(['"]?)(.*?)\1\s*\)|@import\s+(['"])(.*?)\3""", re.I)
REFRESH_URL = re.compile(r"url\s*=\s*['\"]?([^'\";]+)", re.I)


def foreign(url, host):
    """True when url points to another host than the site's own."""
    url = url.strip()
    if url.startswith("//"):
        url = "https:" + url
    parts = urlsplit(url)
    if parts.scheme in ("http", "https"):
        return parts.hostname != host
    return False


class Checker(HTMLParser):
    def __init__(self, path, host, errors):
        super().__init__(convert_charrefs=True)
        self.path, self.host, self.errors = path, host, errors
        self.in_style = False

    def error(self, message):
        line, _ = self.getpos()
        self.errors.append(f"{self.path}:{line}: {message}")

    def check_url(self, url, what):
        if url.strip().lower().startswith("javascript:"):
            self.error(f"javascript: URL in {what}")
        elif foreign(url, self.host):
            self.error(f"{what} loaded from another domain: {url}")

    def check_css(self, css, what):
        for match in CSS_URL.finditer(css):
            self.check_url(match.group(2) or match.group(4), what)

    def handle_starttag(self, tag, attrs):
        if tag in FORBIDDEN_TAGS:
            self.error(f"<{tag}> element")
        if tag == "style":
            self.in_style = True
        attrs = {name: value or "" for name, value in attrs}
        for name, value in attrs.items():
            if name.startswith("on"):
                self.error(f"event handler attribute {name} on <{tag}>")
            if name in RESOURCE_ATTRS:
                self.check_url(value, f"<{tag} {name}>")
            elif name == "href":
                if value.strip().lower().startswith("javascript:"):
                    self.error(f"javascript: URL in <{tag} href>")
                # <a href> is a link the visitor chooses to follow. Anything
                # else (<link>, SVG <use> or <image>, <base>) loads or rewrites.
                elif tag != "a":
                    self.check_url(value, f"<{tag} href>")
            elif name in ("srcset", "imagesrcset"):
                for candidate in value.split(","):
                    if candidate.strip():
                        self.check_url(candidate.split()[0], f"<{tag} {name}>")
            elif name == "style":
                self.check_css(value, f"style attribute on <{tag}>")
        if tag == "meta" and attrs.get("http-equiv", "").lower() == "refresh":
            match = REFRESH_URL.search(attrs.get("content", ""))
            if match:
                self.check_url(match.group(1), "<meta http-equiv=refresh>")
        if tag == "meta" and attrs.get("property", attrs.get("name", "")) in (
            "og:image",
            "twitter:image",
        ):
            self.check_url(attrs.get("content", ""), "share image")

    def handle_endtag(self, tag):
        if tag == "style":
            self.in_style = False

    def handle_data(self, data):
        if self.in_style:
            self.check_css(data, "<style>")


def main():
    if len(sys.argv) != 2:
        sys.exit(__doc__.strip().splitlines()[-1])
    site = Path(sys.argv[1])
    with open(site / "zola.toml", "rb") as config:
        host = urlsplit(tomllib.load(config)["base_url"]).hostname
    root = site / "public"
    if not root.is_dir():
        sys.exit(f"{root} is not a directory: build the site first")
    errors, checked = [], 0
    for path in sorted(p for p in root.rglob("*") if p.is_file()):
        suffix = path.suffix.lower()
        rel = path.relative_to(root)
        if suffix in FORBIDDEN_FILES:
            errors.append(f"{rel}: script file")
        elif suffix in (".html", ".htm", ".svg", ".xhtml"):
            checker = Checker(rel, host, errors)
            checker.feed(path.read_text(encoding="utf-8"))
            checker.close()
            checked += 1
        elif suffix == ".css":
            css = path.read_text(encoding="utf-8")
            for match in CSS_URL.finditer(css):
                url = match.group(2) or match.group(4)
                if foreign(url, host) or url.strip().lower().startswith("javascript:"):
                    line = css.count("\n", 0, match.start()) + 1
                    errors.append(f"{rel}:{line}: CSS resource from another domain: {url}")
            checked += 1
    for error in errors:
        print(f"ERROR: {error}", file=sys.stderr)
    if errors:
        sys.exit(1)
    if checked == 0:
        sys.exit(f"no HTML, SVG or CSS file found in {root}")
    print(f"{checked} files checked: no script, no resource from another domain")


if __name__ == "__main__":
    main()
