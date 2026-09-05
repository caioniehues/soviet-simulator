#!/usr/bin/env bash
# Refresh the code-review-graph derived artifacts that the watch daemon does NOT
# maintain: vector embeddings, the community wiki, and graph.html.
#
# The daemon (systemd user unit crg-watch.service) keeps nodes, edges, flows and
# communities current on every file change. It never embeds — daemon.py contains
# no embedding code — so semantic search would silently miss every symbol written
# after the last manual embed. This hook closes that gap at session start.
#
# Runs detached: session startup must not block on a GPU batch job.

set -u

REPO=/home/caio/soviet-simulator
CRG="$REPO/.venv/bin/code-review-graph"
PY="$REPO/.venv/bin/python"
LOCK=/tmp/crg-refresh.lock
LOG="$HOME/.code-review-graph/logs/refresh.log"

[ -x "$CRG" ] || exit 0

mkdir -p "$(dirname "$LOG")"

# flock -n: a refresh already in flight wins; a second session start is a no-op.
exec 9>"$LOCK"
flock -n 9 || exit 0

{
  echo "=== $(date -Is) refresh start ==="
  "$PY" - <<'EOF'
import os
from code_review_graph.tools.docs import embed_graph
print(embed_graph(
    repo_root="/home/caio/soviet-simulator",
    provider="local",
    model=os.environ["CRG_EMBEDDING_MODEL"],
))
EOF
  "$CRG" wiki --repo "$REPO"
  "$CRG" visualize --repo "$REPO"
  echo "=== $(date -Is) refresh done ==="
} >>"$LOG" 2>&1
