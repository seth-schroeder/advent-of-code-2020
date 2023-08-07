#!/bin/env bash

# this script is the process to prepare the repo for the next star

_day="${1}"
_star="${2}"

if [[ "${_day:-um}" = um ]] || [[ "${_star:-um}" = um ]]; then
    echo usage: $0 day star
    exit 1
fi

branch_name="go/day${_day}-star${_star}"
echo $branch_name

git stash create "before ${branch_name}"
git checkout -b "${branch_name}"

mkdir -p "input-data/${branch_name}"
mkdir -p src
cd src
# TODO: what is the golang version of this?

mkdir -p "${branch_name}"
cd "${branch_name}"
go mod init "github.com/seth-schroeder/advent-of-code-2020/${_day}/${_star}"
git add .

git commit -m "Created branch for ${branch_name}"
cd "${branch_name}"
