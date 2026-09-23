#!/bin/bash
# sync.sh — update a repository's docs/governance/documentation from spec.
# Assembles core protocol and activated profiles based on contracts.md.
# Usage: tools/sync.sh <repo-path> [--force-template]
set -eu
SPEC=$(cd "$(dirname "$0")/../spec" && pwd)
REPO=${1:?usage: sync.sh <repo-path> [--force-template]}
FORCE=${2:-}
GOV=$REPO/docs/governance/documentation
mkdir -p "$GOV"
python3 - "$SPEC" "$GOV" "$FORCE" <<'EOF'
import hashlib, json, os, shutil, sys
spec, gov, force = sys.argv[1], sys.argv[2], sys.argv[3] == "--force-template"

manifest_path = os.path.join(spec, ".manifest.json")
m = json.load(open(manifest_path, "r", encoding="utf-8"))

def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()

def parse_active_profiles(contracts_path):
    if not os.path.exists(contracts_path):
        # Default to all profiles declared in spec contracts.md template
        contracts_path = os.path.join(spec, "contracts.md")
    active = {"core"}
    if os.path.exists(contracts_path):
        with open(contracts_path, "r", encoding="utf-8") as f:
            for line in f:
                line = line.strip()
                if line.startswith("- [x]") or line.startswith("- [X]"):
                    rest = line[5:].strip()
                    p = rest.split("`")[1].strip() if "`" in rest else rest.split()[0].strip()
                    active.add(p)
    return active

# Ensure contracts.md exists first to determine active profiles
contracts_dst = os.path.join(gov, "contracts.md")
contracts_src = os.path.join(spec, "contracts.md")
if not os.path.exists(contracts_dst):
    shutil.copy2(contracts_src, contracts_dst)
    print("+ contracts.md (template initialized)")

active_profiles = parse_active_profiles(contracts_dst)
changed = []

for name, meta in sorted(m["files"].items()):
    profile = meta.get("profile", "core")
    if profile not in active_profiles:
        continue

    src = os.path.join(spec, name)
    dst = os.path.join(gov, name)
    os.makedirs(os.path.dirname(dst), exist_ok=True)

    if not os.path.exists(dst):
        shutil.copy2(src, dst)
        changed.append(f"+ {name}")
        continue

    if sha(src) == sha(dst):
        continue

    if meta.get("editable") and not force:
        print(f"  [kept-local] {name}: local customization retained (use --force-template to overwrite)")
        continue

    shutil.copy2(src, dst)
    changed.append(f"~ {name}")

# Copy hidden manifest
shutil.copy2(manifest_path, os.path.join(gov, ".manifest.json"))

# Clean up obsolete files from previous protocol versions
obsolete_files = [
    "manifest.json",
    "routing.md",
    "style-guide.md",
    "common-patterns.md",
    "review-checklist.md",
    "update-checklist.md",
    "adoption.md",
    "intake-sop.md",
    "index.md",
    "adr-workflow.md",
    "common-docs.md",
    "knowledge-layers.md",
    "core/adoption.md",
    "core/common-patterns.md",
    "core/intake-sop.md",
    "core/review-checklist.md",
    "core/routing.md",
    "core/style-guide.md",
    "core/update-checklist.md",
    "profiles/architecture/adr.pattern.md",
    "profiles/architecture/compaction.pattern.md",
    "profiles/architecture/profile.md",
    "profiles/architecture/rfc.pattern.md",
    "profiles/operations/postmortem.pattern.md",
    "profiles/operations/profile.md",
    "profiles/validation/acceptance.pattern.md",
    "profiles/validation/profile.md",
    "profiles/validation/scenarios.pattern.md",
    "profiles/validation/testing.pattern.md",
]
for old_rel in obsolete_files:
    old_p = os.path.join(gov, old_rel)
    if os.path.exists(old_p):
        os.remove(old_p)

print("\n".join(changed) if changed else "already up to date")
EOF
