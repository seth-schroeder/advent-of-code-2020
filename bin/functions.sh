#!/bin/env bash

function or_die() {
    if [[ $? -ne 0 ]]; then
        echo "${*:-lazy developer provided no error message}"
        exit 99
    fi
}
