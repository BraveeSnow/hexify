# Hexify

Hexify is a command line tool to convert images into your favorite palette.
It's written in rust.

A list of todos:
- [ ] Implement color smoothing so image isn't pixel-like
- [ ] Efficient image processing (Better than O(n^3))
    - With release builds, this takes no time with a 2160x1080 image so might
      not be worth it
- [ ] Output flag as optional, write to source image instead (?)
- [ ] GUI for those who prefer them

## Running

Running hexify is quite simple. But before we start processing, a palette must
be created. An example is given below and is contained in `palette.json`
(based on [Catppuccin Frappe](https://github.com/catppuccin/catppuccin)):

```json
{
  "palette": [
    "#f2d5cf",
    "#eebebe",
    "#f4b8e4",
    "#ca9ee6",
    "#e78284",
    "#ea999c",
    "#ef9f76",
    "#e5c890",
    "#a6d189",
    "#81c8be",
    "#99d1db",
    "#85c1dc",
    "#8caaee",
    "#babbf1",
    "#c6d0f5",
    "#b5bfe2",
    "#a5adce",
    "#949cbb",
    "#838ba7",
    "#737994",
    "#626880",
    "#51576d",
    "#414559",
    "#303446",
    "#292c3c",
    "#232634"
  ]
}
```

Now that we have our palette ready, pass an image that you want to process as
well as the output file name to hexify:

`hexify -i source.png -o out.png -p palette.json`

After it finishes, your source image has successfully converted all colors to
the ones specified in the palette!

## Building

Simply run `cargo build --release` and you'll find the binary at
`./target/release/hexify`.
