#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Checks the permissions of a built APK against the project's absolute rules:
# - no INTERNET permission;
# - ACCESS_FINE_LOCATION, if present, limited to maxSdkVersion <= 30.
#
# Usage: check-apk-permissions.sh path/to/app-release.apk
set -euo pipefail

apk="${1:?usage: $0 path/to/app-release.apk}"

aapt2="$(ls "${ANDROID_HOME:?ANDROID_HOME is not set}"/build-tools/*/aapt2 | sort -V | tail -n 1)"
perms="$("$aapt2" dump permissions "$apk")"
echo "$perms"

status=0

if grep -q "name='android.permission.INTERNET'" <<<"$perms"; then
    echo "ERROR: android.permission.INTERNET is declared in $apk" >&2
    status=1
fi

fine_location="$(grep "name='android.permission.ACCESS_FINE_LOCATION'" <<<"$perms" || true)"
if [[ -n "$fine_location" ]]; then
    max_sdk="$(sed -n "s/.*maxSdkVersion='\([0-9]*\)'.*/\1/p" <<<"$fine_location")"
    if [[ -z "$max_sdk" || "$max_sdk" -gt 30 ]]; then
        echo "ERROR: ACCESS_FINE_LOCATION must have maxSdkVersion <= 30" >&2
        status=1
    fi
fi

exit "$status"
