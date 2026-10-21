#!/bin/sh
# Resolve os arquivos estáticos e o banco padrão em relação à instalação, não ao terminal.
set -eu

instalacao=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
cd "$instalacao"
export RUST_LOG="${RUST_LOG:-info}"
exec ./feed-your-cow "$@"
