# Hayate Widget Gallery

A showcase of every stable widget in [Hayate UI](../GUI_kit). This is
the flagship demo app: open it to see what the toolkit can do, and use
the source as a reference for how to build widgets, theme them, and
wire them into a tab-navigable layout.

## Run

```bash
# Default (dark theme)
cargo run --release

# Win95 theme
cargo run --release -- --win95

# Mac OS 9 theme
cargo run --release -- --macos9

# Japanese UI
cargo run --release -- --ja
```

## Tabs

| Tab          | Widgets shown                                           |
|--------------|----------------------------------------------------------|
| **Basic**    | Button (16 theme presets), Checkbox, Switch, Text        |
| **Input**    | TextInput, TextArea, Slider, SpinButton, Dropdown,       |
|              | RadioGroup, ColorPicker                                  |
| **Layout**   | VStack, HStack, GridLayout (3 cols), nested stacks       |
| **Nav**      | Breadcrumb, NavigationList, nested TabView               |
| **Display**  | ProgressBar, Image, CanvasView (custom draw), ListView,  |
|              | ThumbnailList, StatusBar                                 |
| **Overlay**  | Toast                                                    |

## Theme switching

`--win95` activates the full Win95 skin: grey `#C0C0C0` background,
blue gradient title bar, black text, 3D bevels, sunken LED-style
progress bars, 1-pixel-thick cell borders. The `WindowFrameTheme::win95()`
draws the CSD chrome; `AppTheme::win95()` applies matching per-widget
themes.

## What this proves

Gallery is the stress test for:

- All 20+ stable widgets rendering correctly in one window.
- Tab switching without memory regression (see
  `STATUS.md` in the toolkit for the 32 GB bug we fixed here).
- `WIN95_THEME` applied as the app-wide clear colour, with every
  widget respecting it via its theme struct.
- CJK rendering (Korean, Simplified Chinese, Traditional Chinese
  sample lines in the Basic tab).

If any widget breaks, gallery surfaces it first.

## License

MIT OR Apache-2.0
