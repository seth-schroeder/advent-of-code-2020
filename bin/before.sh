#!/bin/env bash

# this script is the process to prepare the repo for the next star

_day="${1}"
_star="${2}"

if [[ "${_day:-um}" = um ]] || [[ "${_star:-um}" = um ]]; then
    echo usage: $0 day star
    exit 1
fi

day_name="day${_day}-star${_star}"
branch_name="go/${day_name}"

git stash create "before ${branch_name}"
git checkout -b "${branch_name}"

mkdir -p "input-data/${day_name}"
mkdir -p src/go
cd src/go

mkdir -p "${day_name}"
cd "${day_name}"
go mod init "github.com/seth-schroeder/advent-of-code-2020/${day_name}"

# eww
cat > main.go <<_SRC
package main

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}
_SRC

git add .
git commit -m "Created branch for ${branch_name}"
