#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Builds the website into site/public. The threat model is copied in from
# docs/ at build time, so the site never carries a stale copy of it.
#
# Usage: scripts/build-site.sh [zola build options]

set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
threat_model="$root/docs/THREAT_MODEL.md"
page="$root/site/content/modele-de-menace.md"

if [[ -f "$threat_model" ]]; then
  # The document's first line is its "# Title": it becomes the page title.
  title="$(head -n 1 "$threat_model" | sed 's/^# *//')"
  {
    # The description is what search engines and link previews show.
    printf '+++\ntitle = "%s"\npath = "modele-de-menace"\ndescription = "%s"\n+++\n' "$title" \
      "En langage simple : ce que Samare protège, contre qui et ce qu'elle ne protège pas."
    tail -n +2 "$threat_model"
  } > "$page"
fi

cd "$root/site"
zola build "$@"

# Share cards (og.png, one per page) are drawn with resvg. The CI requires
# them; a local build can go without.
if command -v resvg >/dev/null; then
  python3 "$root/scripts/og-cards.py" "$root/site/public" "$root/site/og/fonts"
elif [[ -n "${CI:-}" ]]; then
  echo "resvg is required to draw the share cards" >&2
  exit 1
else
  echo "resvg not found: share cards (og.png) not drawn" >&2
fi
