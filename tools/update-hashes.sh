#!/bin/bash
# update-hashes.sh — (maintainer only) recompute manifest sha256 after
# the spec changes, with a version-bump reminder.
set -eu
cd "$(dirname "$0")/../spec"
python3 - <<'EOF'
import hashlib, json, os
p = ".manifest.json"
with open(p, "r", encoding="utf-8") as f:
    m = json.load(f)
for name, meta in m["files"].items():
    if not meta.get("editable"):
        meta["sha256"] = hashlib.sha256(open(name, "rb").read()).hexdigest()
with open(p, "w", encoding="utf-8") as f:
    json.dump(m, f, indent=2)
    f.write("\n")
n = sum(1 for x in m["files"].values() if not x.get("editable"))
print(f"hashes updated for {n} files in {p}")
print("reminder: bump protocol_version per update_policy (MAJOR/MINOR/PATCH)")
EOF
