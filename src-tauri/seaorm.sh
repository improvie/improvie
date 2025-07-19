#!/usr/bin/env bash

gen() {
	sea-orm-cli generate entity --database-url sqlite:"$PWD"/dev/data.sql --output-dir improvie-infra/row/src --lib --date-time-crate chrono --with-prelude none "$@"
}

# Check if the script is being run from its own directory
function check_script_directory() {
	SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
	CURRENT_DIR="$(pwd)"

	if [ "$SCRIPT_DIR" != "$CURRENT_DIR" ]; then
		echo "Error: This script must be run from its own directory."
		echo "Current directory: $CURRENT_DIR"
		echo "Script directory: $SCRIPT_DIR"
		exit 1
	fi
}

check_script_directory

arg="$1"

shift

case $arg in
"gen")
	gen "$@"
	;;
*)
	echo "Usage: $0 {gen} [options]"
	exit 1
	;;
esac
