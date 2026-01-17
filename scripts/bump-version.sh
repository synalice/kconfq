#!/usr/bin/env bash
#
# SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
#
# SPDX-License-Identifier: MIT

set -o errexit
set -o pipefail
set -o nounset

usage() {
  cat >&2 <<EOF
Usage: $0 <version>

  version    new version (for example "1.0.0")
EOF
  exit 1
}

(( $# == 1 )) || usage

prek run --all-files

TAG="v$1"

git checkout main
cargo set-version $1
git add -A
git commit --no-verify -m "Bump version to $TAG"
git tag -a "$TAG" -m "Release $TAG"
git push origin tag "$TAG"
git push origin main
cargo publish
