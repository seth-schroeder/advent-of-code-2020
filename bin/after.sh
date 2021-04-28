#!/bin/env bash

# this script runs after a star has been earned

branch_name=%(git rev-parse --symbolic-full-name HEAD | sed -e 's,heads/,,')
git checkout main
git merge --ff-only ${branch_name}
git tag ${branch_name}
git push o main
