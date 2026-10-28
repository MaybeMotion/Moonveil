#!/bin/zsh
set -eu
project_dir="$(cd "$(dirname "$0")/.." && pwd)"
cd "$project_dir"
cargo build --release --locked
bundle="$project_dir/Moonveil.app"
mkdir -p "$bundle/Contents/MacOS" "$bundle/Contents/Resources"
mkdir -p "$bundle/Contents/Resources/Licenses"
cp assets/Manrope-OFL.txt assets/Cormorant-OFL.txt "$bundle/Contents/Resources/Licenses/"
cp target/release/moonveil "$bundle/Contents/MacOS/moonveil"
cp assets/Moonveil.icns "$bundle/Contents/Resources/Moonveil.icns"
cat > "$bundle/Contents/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleName</key><string>Moonveil</string>
<key>CFBundleDisplayName</key><string>Moonveil</string>
<key>CFBundleIdentifier</key><string>games.moonveil.idle</string>
<key>CFBundleExecutable</key><string>moonveil</string>
<key>CFBundleIconFile</key><string>Moonveil.icns</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleVersion</key><string>2</string>
<key>CFBundleShortVersionString</key><string>1.1.0</string>
<key>LSMinimumSystemVersion</key><string>11.0</string>
<key>LSApplicationCategoryType</key><string>public.app-category.role-playing-games</string>
<key>NSHighResolutionCapable</key><true/>
<key>NSSupportsAutomaticGraphicsSwitching</key><true/>
</dict></plist>
PLIST
codesign --force --deep --sign - "$bundle"
printf 'Built %s\n' "$bundle"

