#!/bin/sh
set -eu
umask 077  # new files default to 600/700

CONFIG_DIR="${CONFIG_DIR:-/app/services/synapse/src/config}"
CONFIG_FILE="${CONFIG_FILE:-$CONFIG_DIR/synapse-config.json}"
SYNAPSE_PRIVATE_KEY_FILE="${SYNAPSE_PRIVATE_KEY_FILE:-/data/secrets/synapse_private_key}"
GENERATOR="/app/services/synapse/src/utils/initSynapseConfig.js"

UPLOADS_DIR="${UPLOADS_DIR:-/synapse/uploads}"
MAIN="/app/services/synapse/src/core/synapse.js"

mkdir -p "$CONFIG_DIR" "$UPLOADS_DIR"

if [ ! -f "$CONFIG_FILE" ]; then
  if [ "${GENERATE_CONFIG:-true}" != "true" ]; then
    echo "[synapse] ERROR: $CONFIG_FILE missing and GENERATE_CONFIG=false"
    exit 2
  fi
  if [ ! -f "$GENERATOR" ]; then
    echo "[synapse] ERROR: generator not found at $GENERATOR"
    exit 2
  fi
  echo "[synapse] No config found. Generating…"
  node "$GENERATOR" --out "$CONFIG_FILE"
fi

chmod 600 "$CONFIG_FILE" || true
chmod 755 "$UPLOADS_DIR" || true

export CONFIG_FILE
export UPLOADS_DIR

if [ -f "$SYNAPSE_PRIVATE_KEY_FILE" ]; then
  export PRIVATE_KEY="$(cat "$SYNAPSE_PRIVATE_KEY_FILE")"
  echo "[synapse] Synapse private key loaded from $SYNAPSE_PRIVATE_KEY_FILE"
fi

if [ -f /run/secrets/DB_USER_PW ]; then
  export DB_PASSWORD="$(cat /run/secrets/DB_USER_PW)"
  echo "[synapse] MySQL user password loaded from secret."
fi

if [ -f /run/secrets/YOUTUBE_API_KEY ]; then
  export YOUTUBE_API_KEY="$(cat /run/secrets/YOUTUBE_API_KEY)"
  echo "[synapse] YouTube API key loaded from secret."

fi


echo "[synapse] Starting Synapse…"
exec node "$MAIN"
