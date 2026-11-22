#!/usr/bin/env python3
"""Generate a 'latest videos' widget for the public README and site.

Public data only: YouTube RSS for OSCILLUNE. No API key, no token, no credentials.
Usage: python3 scripts/update-youtube-widget.py [PUBLIC_DIR] [N]
"""
from __future__ import annotations

import html
import pathlib
import sys
import urllib.request
import xml.etree.ElementTree as ET

CHANNEL = "UCNfdo7Gn4oZmSNviGmJgXYg"
RSS = f"https://www.youtube.com/feeds/videos.xml?channel_id={CHANNEL}"
CHANNEL_URL = "https://www.youtube.com/@oscillune"
NS = {"a": "http://www.w3.org/2005/Atom", "yt": "http://www.youtube.com/xml/schemas/2015"}
START = "<!-- YT-WIDGET:START -->"
END = "<!-- YT-WIDGET:END -->"

root = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else pathlib.Path(__file__).resolve().parents[1]
n = int(sys.argv[2]) if len(sys.argv) > 2 else 3

with urllib.request.urlopen(RSS, timeout=20) as r:
    feed = ET.fromstring(r.read())
entries = feed.findall("a:entry", NS)[:n]
items = []
for e in entries:
    vid = e.find("yt:videoId", NS).text
    title = (e.find("a:title", NS).text or "").replace("|", "-")
    title = " ".join(title.split())  # normalise whitespace
    items.append((vid, title))

md = [START, "**Latest videos — [OSCILLUNE on YouTube](" + CHANNEL_URL + ")**", ""]
md.append('<table><tr>')
for vid, title in items:
    md.append(
        f'<td align="center"><a href="https://www.youtube.com/watch?v={vid}">'
        f'<img src="https://i.ytimg.com/vi/{vid}/hqdefault.jpg" alt="{html.escape(title, quote=True)}" width="180"><br>'
        f'<sub>{html.escape(title[:54])}</sub></a></td>'
    )
md.append("</tr></table>")
md.append(END)
md_block = "\n".join(md)

html_lines = [START, '<section style="max-width:900px;margin:28px auto">',
        f'<h2>Latest videos — <a href="{CHANNEL_URL}">OSCILLUNE on YouTube</a></h2>',
        '<div style="display:flex;gap:16px;flex-wrap:wrap;justify-content:center">']
for vid, title in items:
    html_lines.append(
        f'<a href="https://www.youtube.com/watch?v={vid}" style="text-decoration:none;color:inherit">'
        f'<img src="https://i.ytimg.com/vi/{vid}/hqdefault.jpg" alt="{html.escape(title, quote=True)}" width="260" '
        f'style="border-radius:8px"><div style="font-size:13px;margin:6px 0 0">{html.escape(title[:70])}</div></a>'
    )
html_lines.append("</div></section>")
html_lines.append(END)
html_block = "\n".join(html_lines)


def insert(path: pathlib.Path, block: str, anchor: str) -> None:
    t = path.read_text()
    if START in t and END in t:
        pre, rest = t.split(START, 1)
        _, post = rest.split(END, 1)
        t = pre + block + post
    else:
        assert anchor in t, f"anchor not found in {path}"
        t = t.replace(anchor, block + "\n" + anchor, 1)
    path.write_text(t)


insert(root / "README.md", md_block,
       "[Installation](#installation)")
insert(root / "PROFILE-README.md", md_block,
       "<!-- YT-WIDGET:START -->")
insert(root / "docs/index.html", html_block, "<h2>6. Download</h2>")
print(f"widget updated: {len(items)} videos -> {root}")
