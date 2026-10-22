# NOTED™

Aesthetic notes, drawing and code editor for macOS, Windows, Linux and Android.

## Get the installers
Push this folder to GitHub (branch `main`) or press Actions → Build NOTED → Run workflow. GitHub builds the `.dmg`, `.exe`/`.msi`, `.deb`, `.rpm`, `.AppImage` into a draft Release, and the Android `.apk` appears under the finished run → Artifacts.

## Build on your own Mac
    npm install && npm run icons && npm run tauri build      # .dmg in src-tauri/target/release/bundle/dmg

## Install
- Debian/Ubuntu: `sudo apt install ./NOTED_1.0.0_amd64.deb`
- Fedora: `sudo dnf install ./NOTED-1.0.0-1.x86_64.rpm`
- Arch: `makepkg -si` (with this PKGBUILD) or run the AppImage
- Android: enable "install unknown apps", open the `.apk`

Replace `app-icon.png` (1024×1024) with your logo, then run `npm run icons`.
