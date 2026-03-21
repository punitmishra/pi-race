#!/usr/bin/env bash

# ─────────────────────────────────────────────────────────────────

# push-to-github.sh

# Creates the ‘pi-race’ repo on GitHub and pushes all files.

# Usage: GH_TOKEN=ghp_xxx GH_USER=yourname bash push-to-github.sh

# ─────────────────────────────────────────────────────────────────

set -euo pipefail

GH_TOKEN=”${GH_TOKEN:?Set GH_TOKEN=ghp_xxx}”
GH_USER=”${GH_USER:?Set GH_USER=your-github-username}”
REPO=“pi-race”
API=“https://api.github.com”

echo “▶ Creating repo ${GH_USER}/${REPO} on GitHub…”
HTTP=$(curl -s -o /tmp/gh_create.json -w “%{http_code}”   
-X POST “${API}/user/repos”   
-H “Authorization: Bearer ${GH_TOKEN}”   
-H “Accept: application/vnd.github+json”   
-H “X-GitHub-Api-Version: 2022-11-28”   
-d “{
"name": "${REPO}",
"description": "🔵 The Race to π — Ramanujan vs Euler vs Wallis vs Leibniz in Rust",
"private": false,
"auto_init": false,
"has_issues": true,
"has_projects": false,
"has_wiki": false
}”)

if [[ “$HTTP” == “201” ]]; then
echo “✓ Repo created successfully”
elif [[ “$HTTP” == “422” ]]; then
echo “⚠ Repo already exists — will push to existing repo”
else
echo “✗ GitHub API error ($HTTP):”
cat /tmp/gh_create.json
exit 1
fi

REMOTE=“https://${GH_TOKEN}@github.com/${GH_USER}/${REPO}.git”

SCRIPT_DIR=”$(cd “$(dirname “${BASH_SOURCE[0]}”)” && pwd)”
cd “$SCRIPT_DIR”

echo “▶ Initializing git…”
git init -b main
git config user.email “pi-race@example.com”
git config user.name “${GH_USER}”

git add .
git commit -m “🔵 Initial commit: The Race to π

Four legendary π algorithms in zero-dependency Rust:

- Ramanujan (1914) — ~8 digits per term
- Euler/Basel (1735) — O(1/n²)
- Wallis (1655) — O(1/√n)
- Leibniz (1676) — O(1/n)

Includes GitHub Actions CI: build, run, benchmark, cross-compile.”

echo “▶ Pushing to github.com/${GH_USER}/${REPO}…”
git remote add origin “$REMOTE” 2>/dev/null || git remote set-url origin “$REMOTE”
git push -u origin main –force

# Clean token from remote URL immediately after push

git remote set-url origin “https://github.com/${GH_USER}/${REPO}.git”

echo “”
echo “✅ Done! Your repo is live:”
echo “   https://github.com/${GH_USER}/${REPO}”
echo “   Actions: https://github.com/${GH_USER}/${REPO}/actions”
echo “”
echo “The CI workflow will start automatically. Watch it at the Actions URL above.”
