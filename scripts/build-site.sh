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
    printf '+++\ntitle = "%s"\npath = "modele-de-menace"\n+++\n' "$title"
    tail -n +2 "$threat_model"
  } > "$page"
fi

cd "$root/site"
zola build "$@"
