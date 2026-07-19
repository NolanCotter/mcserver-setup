#!/usr/bin/env sh
# Install mcserver-setup from a source checkout.
set -eu

cargo install --path . --locked

CARGO_BIN_DIR="${CARGO_HOME:-$HOME/.cargo}/bin"

case ":$PATH:" in
  *":$CARGO_BIN_DIR:"*)
    printf '%s\n' 'Installed. Start the wizard with: mcserver-setup'
    ;;
  *)
    printf 'Installed to %s/mcserver-setup.\n' "$CARGO_BIN_DIR"
    printf '%s\n' 'Add it to your PATH, then start the wizard with: mcserver-setup'
    printf 'For zsh: echo '\''export PATH="%s:$PATH"'\'' >> ~/.zshrc && source ~/.zshrc\n' "$CARGO_BIN_DIR"
    ;;
esac
