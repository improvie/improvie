#!/usr/bin/env bash

#!/usr/bin/env bash

# This script updates the version number in package.json, Cargo.toml, Cargo.lock, tauri.conf.json, project.yml, Info.plist and README.md.
# Usage: ./scripts/update_version.sh <new_version>

set -euo pipefail

cd "$(dirname "$0")/.." || exit

# Check if the new version is provided
if [ $# -ne 1 ]; then
	echo "Usage: $0 <new_version>"
	exit 1
fi

NEW_VERSION=$1

# Check if jq is installed
if ! command -v jq &>/dev/null; then
	echo "❌ jq is not installed. Please install jq to run this script."
	exit 1
fi

# Extract old version from package.json
OLD_VERSION=$(jq -r '.version' package.json)

if [ "$OLD_VERSION" == "$NEW_VERSION" ]; then
	echo "✅ Version is already set to $NEW_VERSION"
	exit 0
fi

echo "🔧 Updating version from $OLD_VERSION to $NEW_VERSION"

# Update package.json
jq --arg new_version "$NEW_VERSION" '.version = $new_version' package.json >package.json.tmp && mv package.json.tmp package.json
echo "✅ Updated package.json"

(
	cd src-tauri || exit
	# Update Cargo.toml
	sed -i.bak "s/version = \"$OLD_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml && rm Cargo.toml.bak
	echo "✅ Updated Cargo.toml"
	cargo update --workspace &>/dev/null
	echo "✅ Updated Cargo.lock"

	# Update tauri.conf.json
	jq --arg new_version "$NEW_VERSION" '.version = $new_version' tauri.conf.json >tauri.conf.json.tmp && mv tauri.conf.json.tmp tauri.conf.json
	echo "✅ Updated tauri.conf.json"
)

# Escape dots in OLD_VERSION for safety in sed
SAFE_OLD_VERSION=$(printf '%s\n' "$OLD_VERSION" | sed 's/[.[\*^$]/\\&/g')

# Update relation of mobile

sed -i.bak "s/CFBundleShortVersionString: $SAFE_OLD_VERSION/CFBundleShortVersionString: $NEW_VERSION/" src-tauri/gen/apple/project.yml && rm src-tauri/gen/apple/project.yml.bak
sed -i.bak "s/CFBundleVersion: $SAFE_OLD_VERSION/CFBundleVersion: $NEW_VERSION/" src-tauri/gen/apple/project.yml && rm src-tauri/gen/apple/project.yml.bak
sed -i.bak "s/<string>$SAFE_OLD_VERSION<\/string>/<string>$NEW_VERSION<\/string>/" src-tauri/gen/apple/improvie_iOS/Info.plist && rm src-tauri/gen/apple/improvie_iOS/Info.plist.bak

echo "✅ Updated project.yml"

# Update README.md

if [[ "$OSTYPE" == "darwin"* ]]; then
	sed -i '' "s/$SAFE_OLD_VERSION/$NEW_VERSION/g" README.md
else
	sed -i "s/$SAFE_OLD_VERSION/$NEW_VERSION/g" README.md
fi

echo "✅ Updated README.md"

echo "🎉 Done!"
