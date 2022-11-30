year := "2022"
set dotenv-load := true

run *DAYS:
  #!/usr/bin/env bash
  set -euo pipefail
  if [[ "{{DAYS}}" == "" ]]; then \
    cargo run --release; \
  else \
    for day in {{DAYS}}; do echo ""; cargo run --release --bin $day; done; \
  fi

get DAY:
  #!/usr/bin/env bash
  set -euo pipefail
  
  if [[ ! '{{DAY}}' =~ ^(0[1-9]|1[0-9]|2[0-5])$ ]]; then \
    echo "Argument {{DAY}} is not a valid day."; \
    exit 1; \
  fi

  if [[ -z "${AOC_SESSION-""}" ]]; then \
    echo "No session token set in \$AOC_SESSION."; \
    exit 1; \
  fi

  URL="https://adventofcode.com/{{year}}/day/$(("10#{{DAY}}" + 0))/input"
  mkdir -p inputs
  mkdir -p input_examples
  curl "$URL" --cookie "session=$AOC_SESSION" -s | tee "inputs/{{DAY}}.in"
  touch 'input_examples/{{DAY}}.in'
  cp -n "src/template.rs" "src/bin/{{DAY}}.rs"
  sed -i "s/xx/{{DAY}}/g" "src/bin/{{DAY}}.rs"
