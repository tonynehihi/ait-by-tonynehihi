# Changelog

## Illustrator by Tonynehihi 1.2.0

Syncs the app with upstream VectorCraft 0.16.0 (AiT 1.1.0 was based on 0.11.0), plus fork repairs found during the merge.

- **PERCEPTUAL GRADIENTS**: gradients can now mix their colours in OKLab — pick Classic or Perceptual in the Gradient panel's new Method menu. Colour emoji fonts and COLR colour fonts draw on canvas and export as vector art.
- **NEW RASTER EFFECTS**: Sketch › Note Paper, Bas Relief, Charcoal, Photocopy and Stamp, plus Artistic › Smudge Stick, Sponge and Watercolor.
- **SHAPE BUILDER**: Shift-drag draws a marquee and merges every region it reaches, and a click on a stroke splits the path right there. The Transform effect gains Transform Objects, Transform Patterns and Scale Strokes & Effects options.
- **PLUG-INS GROW UP**: live effects read their object's fills and strokes, plug-in exporters can write file formats of their own, and filters keep their data with the document.
- **ARTBOARDS**: Width, Height, Preset and Orientation apply to every selected artboard at once, and a copied .ai or .ait file now pastes the objects of its editing data.
- **EXPORT WARNINGS**: Save for Web, Export for Screens and Save for Office warn when an image couldn't be decoded or a font is missing, instead of silently shipping a broken file.
- **FIXED IN THE FORK**: the app binary itself (broken by a bad merge), the keyboard-shortcut export that silently dropped its file dialog answer, and the translation catalogs in every language, restored in full — the complete test suite passes 2013/2013 on a Mac.

— tonynehihi

## Illustrator by Tonynehihi 1.1.0

First release of Illustrator by Tonynehihi (AiT), a personal build based on VectorCraft 0.11.0 by storytold.

- **NEW NAME AND ICON**: the app is now "Illustrator by Tonynehihi" everywhere — window title, menus, About screen and the macOS bundle **AiT.app** — with the Adobe-style AiT icon (orange letters on a dark tile). Settings and document folders keep their original names, so existing data is untouched.
- **SELF-UPDATE BUILT IN**: Help → **Check for Updates** (and a one-time check at launch) asks this repo's Releases for a newer version and shows its changelog in the app. On macOS the update downloads, builds and installs itself; the app closes only for a few seconds at the end to swap versions, keeps the previous version as a backup, and reopens on its own.
- **UPSTREAM LINKS REMOVED**: the Help menu is now exactly Check for Updates / Keyboard Shortcuts / Search Commands / About. The Discord button in the title bar, the start screen's Community section with its Discord / website / GitHub links, and the upstream documentation link in the graphics error message are gone — nothing in the app points back to the upstream project anymore.
- Everything else is VectorCraft 0.11.0 as released by storytold; AiT tracks the upstream project and folds its new features into future releases.

— tonynehihi
