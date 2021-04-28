#!/bin/env bash

# this script is the process to prepare the repo for the next star

_day="${1}"
_star="${2}"

if [[ "${_day:-um}" = yo ]] || [[ "${_star:-um}" = um ]]; then
    echo usage: $0 day star
    exit 1
fi

branch_name="day${_day}-star${_star}"
echo $branch_name

git stash create "before ${branch_name}"
git checkout -b "${branch_name}"

mkdir -p "input-data/${branch_name}"
mkdir -p src
cd src
cargo new "${branch_name}" --bin --vcs none
git add "${branch_name}"

git commit -m "Created branch for ${branch_name}"
cd "${branch_name}"
