set dotenv-load := true

post DAY:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo run -q --release --bin post {{DAY}}

run *DAYS:
  #!/usr/bin/env bash
  set -euo pipefail
  if [[ "{{DAYS}}" == "" ]]; then \
    cargo run -q --release; \
  else \
    for day in {{DAYS}}; do echo ""; cargo run -q --release --bin $day; done; \
  fi

get *DAYS:
  #!/usr/bin/env bash
  set -euo pipefail
  for day in {{DAYS}}; do just _get $day; done;

_get DAY:
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

  URL="https://adventofcode.com/$YEAR/day/$(("10#{{DAY}}" + 0))/input"
  mkdir -p inputs
  mkdir -p input_examples
  mkdir -p answers
  curl "$URL" --cookie "session=$AOC_SESSION" -s | tee "inputs/{{DAY}}.in"
  touch "input_examples/{{DAY}}.in"
  echo -e "part one: \npart two: " > "answers/{{DAY}}.sol"
  cp -n "src/template.rs" "src/bin/{{DAY}}.rs"
  sed -i "s/xx/{{DAY}}/g" "src/bin/{{DAY}}.rs"
