#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Checks a built APK against the project's absolute rules:
# - no INTERNET permission;
# - ACCESS_FINE_LOCATION, if present, limited to maxSdkVersion <= 30;
# - no permission outside the allowlist below;
# - no system backup: allowBackup is false, and the data extraction rules
#   exclude every domain from cloud backup and device-to-device transfer.
#
# Usage: check-apk-permissions.sh path/to/app-release.apk
set -euo pipefail

# Every permission the APK may request. Adding one is a deliberate change to
# this list, reviewed with the feature that needs it. NAME, or NAME@N when
# the permission must be limited to maxSdkVersion <= N.
allowed=(
    # Added by androidx.core to protect the app's own unexported receivers.
    "app.samare.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION"
)

# Domains that the data extraction rules must exclude, in both sections.
backup_domains=(root file database sharedpref external
    device_root device_file device_database device_sharedpref)

apk="${1:?usage: $0 path/to/app-release.apk}"

aapt2="$(ls "${ANDROID_HOME:?ANDROID_HOME is not set}"/build-tools/*/aapt2 | sort -V | tail -n 1)"
perms="$("$aapt2" dump permissions "$apk")"
echo "$perms"

status=0
fail() {
    echo "ERROR: $*" >&2
    status=1
}

# Absolute rules, checked even if someone adds these to the allowlist.
if grep -q "name='android.permission.INTERNET'" <<<"$perms"; then
    fail "android.permission.INTERNET is declared in $apk"
fi

fine_location="$(grep "name='android.permission.ACCESS_FINE_LOCATION'" <<<"$perms" || true)"
if [[ -n "$fine_location" ]]; then
    max_sdk="$(sed -n "s/.*maxSdkVersion='\([0-9]*\)'.*/\1/p" <<<"$fine_location")"
    if [[ -z "$max_sdk" || "$max_sdk" -gt 30 ]]; then
        fail "ACCESS_FINE_LOCATION must have maxSdkVersion <= 30"
    fi
fi

# Allowlist. Covers uses-permission and uses-permission-sdk-23.
while IFS= read -r line; do
    name="$(sed -n "s/.*name='\([^']*\)'.*/\1/p" <<<"$line")"
    max_sdk="$(sed -n "s/.*maxSdkVersion='\([0-9]*\)'.*/\1/p" <<<"$line")"
    found=""
    for entry in "${allowed[@]}"; do
        if [[ "${entry%@*}" == "$name" ]]; then
            found="$entry"
            break
        fi
    done
    if [[ -z "$found" ]]; then
        fail "$name is not in the allowlist of $0"
    elif [[ "$found" == *@* ]]; then
        limit="${found#*@}"
        if [[ -z "$max_sdk" || "$max_sdk" -gt "$limit" ]]; then
            fail "$name must have maxSdkVersion <= $limit"
        fi
    fi
done < <(grep "^uses-permission" <<<"$perms" || true)

# Backup.
manifest="$("$aapt2" dump xmltree --file AndroidManifest.xml "$apk")"
application="$(awk '/^ *E: application /{on=1; next} on && /^ *E: /{exit} on' <<<"$manifest")"
if ! grep -q ':allowBackup([0-9a-fx]*)=false$' <<<"$application"; then
    fail "android:allowBackup must be false"
fi

rules_id="$(sed -n 's/.*:dataExtractionRules([0-9a-fx]*)=@\(0x[0-9a-f]*\)$/\1/p' <<<"$application")"
if [[ -z "$rules_id" ]]; then
    fail "android:dataExtractionRules is missing: Android 12+ would allow device-to-device transfer"
else
    # Release builds rename resource files: find the rules through their id.
    rules_file="$("$aapt2" dump resources "$apk" |
        awk -v id="$rules_id" '$1 == "resource" && $2 == id {on=1; next} on && /\(file\)/ {sub(/.*\(file\) /, ""); print $1; exit}')"
    rules="$("$aapt2" dump xmltree --file "$rules_file" "$apk")"
    if grep -q "E: include" <<<"$rules"; then
        fail "the data extraction rules must not include anything"
    fi
    # One "section domain" line per exclude of a whole domain (path ".").
    excluded="$(awk '
        /E: (cloud-backup|device-transfer) / { section = $2 }
        /E: exclude / { domain = ""; path = "" }
        /A: domain=/ { domain = $2; sub(/^domain="/, "", domain); sub(/".*/, "", domain) }
        /A: path=/ { path = $2; sub(/^path="/, "", path); sub(/".*/, "", path)
                     if (path == ".") print section, domain }
    ' <<<"$rules")"
    for section in cloud-backup device-transfer; do
        for domain in "${backup_domains[@]}"; do
            if ! grep -qx "$section $domain" <<<"$excluded"; then
                fail "the data extraction rules must exclude domain $domain from $section"
            fi
        done
    done
fi

if [[ "$status" -eq 0 ]]; then
    echo "OK: permissions in the allowlist, no INTERNET, no system backup"
fi
exit "$status"
