<p align="center">
  <img alt="AiT" src="assets/app-icon/ait-icon-1024.png" width="180">
</p>

<h1 align="center">Illustrator by Tonynehihi (AiT)</h1>

<p align="center">
  <b>A personal fork of <a href="https://github.com/storytold/vectorcraft">VectorCraft</a> — rebranded and customized by Tony for his own use.</b><br>
  Not an official release. Personal use only.
</p>

## Origin

All of the original source code in this project is [VectorCraft](https://github.com/storytold/vectorcraft) by **storytold** — a vector illustration application written in Rust, released under the **MIT / Apache-2.0** licenses (see `LICENSE-MIT` and `LICENSE-APACHE` in this repo, kept unchanged from the original).

This repository is **not** the official VectorCraft and is not a public product release. It is a copy that Tony took, renamed to **Illustrator by Tonynehihi**, and adjusted for his own personal workflow. All credit for the original software belongs to storytold and the VectorCraft contributors.

> "Illustrator" and "Adobe Illustrator" are trademarks of Adobe. This personal project is not affiliated with or endorsed by Adobe in any way; the name is used only for Tony's private use.

## What Tony changed

- **Name & identity**: the app is called "Illustrator by Tonynehihi", with its own **AiT** icon (Adobe-style: orange letters on a dark tile), About screen and macOS bundle (`AiT.app`).
- **Built-in self-update**: the app checks the Releases of this repo for the newest version — once at launch (with the release's changelog on screen when something new exists) and any time from *Help → Check for Updates*. On macOS it then downloads, builds and installs the update by itself; the current version is only swapped out for a few seconds at the very end, and the previous version is kept as a backup.
- **Clean Help menu**: Check for Updates / Keyboard Shortcuts / Search Commands / About Illustrator by Tonynehihi — and **no links back to the upstream project anywhere in the app** (the Discord button in the title bar, the start screen's Community section and every website / GitHub link are gone).

Settings, documents and data folders keep their original names, so nothing already on the machine is orphaned by the rename.

## Installing on macOS

The Mac build is a source build, compiled with Cargo on Tony's own machine. In short:

```bash
cargo build --release -p vectorcraft
packaging/macos/package.sh --arch aarch64
```

The DMG lands in `dist/release/`. Full instructions (clean install, removing the old build, Gatekeeper troubleshooting) are in `packaging/macos/HUONG-DAN.md` (in Vietnamese).

Once AiT is installed, there is no need to repeat these steps for each new version: just open the app and use **Check for Updates**.

## License

Kept from the original project: **MIT OR Apache-2.0**. Tony's modifications in this repo are offered under the same terms, with the attribution above.
