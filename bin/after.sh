#!/bin/env bash
#
# this script runs after a star has been earned to tag, merge, and push
#

# this relative path stinks
source bin/functions.sh

mess=$(git status --porcelain=2)
[[ "${mess}" = "" ]] || or_die .... hey you gotta clean up first

full_branch_name=$(git rev-parse --symbolic-full-name HEAD)
branch_name=$(basename ${full_branch_name})
git fetch --all --prune --quiet || or_die fetch

git rebase o/main || or_die rebase

cd "src/${branch_name}"
cargo test || or_die tests failed

git checkout main || or_die checkout

git merge --ff-only o/main "${branch_name}" || or_die merge

git tag "${branch_name}" || or_die tag

git push o main "${branch_name}" || or_die push
