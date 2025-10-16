#!/bin/bash
set -euo pipefail

LIB_PATH="$1"

export OVERLOAD_LOG=off

redirected_test() {
    VAR=$1
    echo "redirected_test started - $VAR"

    export OVERLOAD_CMD="bash -c 'cat; echo stdout; echo stderr >&2'"
    OVERLOAD_STDIN="$(mktemp)"
    OVERLOAD_STDOUT="$(mktemp)"
    OVERLOAD_STDERR="$(mktemp)"
    export OVERLOAD_STDIN OVERLOAD_STDOUT OVERLOAD_STDERR
    echo stdin > "$OVERLOAD_STDIN"
    env "$VAR=$LIB_PATH" /usr/bin/sleep 0
    if ! grep -q stdout "$OVERLOAD_STDOUT"; then
        echo "stdout redirect did not work" >&2
        exit 1
    fi
    if ! grep -q stderr "$OVERLOAD_STDERR"; then
        echo "stderr redirect did not work" >&2
        exit 1
    fi
    if ! grep -q stdin "$OVERLOAD_STDOUT"; then
        echo "stdin redirect did not work" >&2
        exit 1
    fi

    echo "redirected_test succeeded - $VAR"
}

inherited_test() {
    VAR=$1
    echo "inherited_test started - $VAR"

    export OVERLOAD_CMD="bash -c 'cat; echo stdout123; echo stderr123 >&2'"
    STDOUT="$(mktemp)"
    STDERR="$(mktemp)"
    echo stdin123 | env "$VAR=$LIB_PATH" /usr/bin/sleep 0 2>"$STDERR" >"$STDOUT"
    if ! grep -q stdout123 "$STDOUT"; then
        echo "stdout redirect did not work" >&2
        exit 1
    fi
    if ! grep -q stderr123 "$STDERR"; then
        echo "stderr redirect did not work" >&2
        exit 1
    fi
    if ! grep -q stdin123 "$STDOUT"; then
        echo "stdin redirect did not work" >&2
        exit 1
    fi

    echo "inherited_test succeeded - $VAR"
}

# subshells are used on purpose, so that exports don't cross-pollute tests
(redirected_test "LD_AUDIT")
(redirected_test "LD_PRELOAD")
(inherited_test "LD_AUDIT")
(inherited_test "LD_PRELOAD")
