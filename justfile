run day part *var:
    #!/usr/bin/env bash
    set -e
    if ! [ -d inputs ]; then
        mkdir inputs
    fi
    if ! [ -f inputs/day{{day}}.txt ]; then
        curl -sSL -H "Cookie: session=$(cat cookie)" "https://adventofcode.com/2025/day/{{day}}/input" -o inputs/day{{day}}.txt
    fi
    cargo run -- d{{day}}p{{part}}{{var}}

render-day4:
    cargo run -p render-day4 -- $(realpath inputs/day4.txt)
