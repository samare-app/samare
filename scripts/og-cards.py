#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-or-later
"""Draws a share card (1200x630 PNG) for every page of the built site.

The text comes from the Open Graph tags that site/templates/base.html puts in
each page, so a card always matches what Mastodon and others show next to it.
Each card is written as og.png beside the page's index.html, where its og:image
tag points. Standard library only; resvg turns the SVG into a PNG.

Usage: scripts/og-cards.py <site/public> <fonts dir> [resvg binary]
"""

import subprocess
import sys
import tempfile
import unicodedata
from datetime import date
from html.parser import HTMLParser
from pathlib import Path
from xml.sax.saxutils import escape

WIDTH, HEIGHT = 1200, 630
MARGIN = 72
TEXT_WIDTH = 790  # the pale samaras take the right-hand side
INK, MUTED, BG, ACCENT, SEED, RULE = "#1B1D1A", "#5A5E57", "#F7F7F3", "#2D6A3E", "#8A5300", "#DCDDD5"
SANS, MONO = "Atkinson Hyperlegible Next", "Atkinson Hyperlegible Mono"

# One samara of the logo, nutlet at the origin, wing hanging down; mirrored for the pair.
SAMARA = "M0 -9 C6 -9 8 -3 7 5 L7 66 C7 77 -5 82 -13 75 C-23 64 -24 38 -14 19 C-10 12 -7 7 -7 0 C-7 -6 -4 -9 0 -9 Z"
SIDES = ("translate(45 15) rotate(24) scale(.92)", "translate(55 15) scale(-1 1) rotate(24) scale(.92)")


class OpenGraph(HTMLParser):
    """Collects the <meta property="og:…"> and article:… values of a page."""

    def __init__(self):
        super().__init__()
        self.meta = {}

    def handle_starttag(self, tag, attrs):
        if tag == "meta":
            a = dict(attrs)
            key = a.get("property") or a.get("name")
            if key and "content" in a:
                self.meta[key] = a["content"]


def advance(ch, bold):
    """Rough width of a character, in em, for Atkinson Hyperlegible Next."""
    base = unicodedata.normalize("NFD", ch)[0]
    if base == " ":
        w = 0.25
    elif base in "ijlI.,:;'!|’":
        w = 0.27
    elif base in "ftr()-«»":
        w = 0.36
    elif base in "mwMW":
        w = 0.86
    elif base.isupper():
        w = 0.66
    elif base.isdigit():
        w = 0.56
    else:
        w = 0.53
    return w * (1.04 if bold else 1.0)


def wrap(text, size, max_lines, bold):
    """Greedy line breaking; the last line ends with an ellipsis if text is left over."""
    limit = TEXT_WIDTH / size
    lines, current = [], ""
    for word in text.split():
        trial = f"{current} {word}".strip()
        if sum(advance(c, bold) for c in trial) <= limit or not current:
            current = trial
        else:
            lines.append(current)
            current = word
    lines.append(current)
    if len(lines) > max_lines:
        last = lines[max_lines - 1]
        while last and sum(advance(c, bold) for c in last + " …") > limit:
            last = last.rsplit(" ", 1)[0] if " " in last else last[:-1]
        lines = lines[: max_lines - 1] + [last.rstrip(" ,;:") + " …"]
    return lines


def fit_title(text):
    """Largest size at which the title holds in three lines, else four lines at the smallest."""
    for size in (58, 52, 46):
        lines = wrap(text, size, 99, True)
        if len(lines) <= 3:
            return size, lines
    return 46, wrap(text, 46, 4, True)


def pair(transform, opacity, veins):
    out = [f'<g transform="{transform}" opacity="{opacity}">']
    for side in SIDES:
        out.append(f'<g transform="{side}"><path d="{SAMARA}" fill="{ACCENT}"/>')
        out.append(f'<ellipse cx="0" cy="-0.5" rx="7" ry="8.5" fill="{SEED}"/>')
        if veins:
            out.append(f'<path d="M-7 8 C-3 11 3 11 7 8 M3 12 C3 32 0 55 -6 71" fill="none" stroke="{BG}" stroke-width="2" stroke-linecap="round"/>')
        out.append("</g>")
    out.append("</g>")
    return "".join(out)


def card(meta, is_home):
    published = meta.get("article:published_time", "")[:10]
    if is_home:
        headline, summary = meta.get("og:description", ""), ""
        kicker, footer = "", "Messagerie sans réseau, en développement"
    else:
        headline, summary = meta.get("og:title", ""), meta.get("og:description", "")
        # A plain space at the start of a tspan would be collapsed away.
        kicker = " · Journal" if published else ""
        footer = "Billet du journal" if published else ""

    size, lines = fit_title(headline)
    leading = round(size * 1.16)
    y = 220
    svg = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{HEIGHT}" viewBox="0 0 {WIDTH} {HEIGHT}">',
        f'<rect width="{WIDTH}" height="{HEIGHT}" fill="{BG}"/>',
        pair("translate(880 150) scale(2.9)", 0.14, False),
        pair("translate(66 52) scale(0.6)", 1, True),
        f'<text x="136" y="96" font-family="{SANS}" font-size="34" font-weight="700" fill="{INK}">Samare'
        f'<tspan fill="{MUTED}" font-weight="400">{escape(kicker)}</tspan></text>',
    ]
    if published:
        day = date.fromisoformat(published).strftime("%d/%m/%Y")
        svg.append(f'<text x="{WIDTH - MARGIN}" y="96" text-anchor="end" font-family="{MONO}" font-size="26" fill="{MUTED}">{day}</text>')
    svg.append(f'<text font-family="{SANS}" font-size="{size}" font-weight="700" fill="{INK}">')
    for i, line in enumerate(lines):
        svg.append(f'<tspan x="{MARGIN}" y="{y + i * leading}">{escape(line)}</tspan>')
    svg.append("</text>")
    if summary:
        top = y + (len(lines) - 1) * leading + 74
        rows = wrap(summary, 27, 2 if len(lines) < 4 else 1, False)
        svg.append(f'<text font-family="{SANS}" font-size="27" fill="{MUTED}">')
        for i, row in enumerate(rows):
            svg.append(f'<tspan x="{MARGIN}" y="{top + i * 38}">{escape(row)}</tspan>')
        svg.append("</text>")
    svg += [
        f'<line x1="{MARGIN}" y1="540" x2="{WIDTH - MARGIN}" y2="540" stroke="{RULE}" stroke-width="2"/>',
        f'<text x="{MARGIN}" y="590" font-family="{SANS}" font-size="26" fill="{MUTED}">{escape(footer)}</text>',
        f'<text x="{WIDTH - MARGIN}" y="590" text-anchor="end" font-family="{SANS}" font-size="26" font-weight="700" fill="{ACCENT}">samare.app</text>',
        "</svg>",
    ]
    return "\n".join(svg)


def main():
    public, fonts = Path(sys.argv[1]), Path(sys.argv[2])
    resvg = sys.argv[3] if len(sys.argv) > 3 else "resvg"
    font_args = [arg for f in sorted(fonts.glob("*.ttf")) for arg in ("--use-font-file", str(f))]
    count = 0
    with tempfile.TemporaryDirectory() as tmp:
        for page in sorted(public.rglob("index.html")):
            parser = OpenGraph()
            parser.feed(page.read_text(encoding="utf-8"))
            if not parser.meta.get("og:image", "").endswith("/og.png"):
                continue
            source = Path(tmp) / f"card{count}.svg"
            source.write_text(card(parser.meta, page.parent == public), encoding="utf-8")
            subprocess.run([resvg, *font_args, str(source), str(page.parent / "og.png")], check=True)
            count += 1
    print(f"{count} share cards drawn")


if __name__ == "__main__":
    main()
