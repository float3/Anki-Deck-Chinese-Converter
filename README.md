# Anki-Deck-Chinese-Converter

converts an exported Anki deck from Simplified to Traditional Chinese and from Pinyin to Zhuyin. export the notes from Anki with tabs as separators.

```sh
cargo run --release -- path/to/exported_notes.txt
```

optional flags after the path:

- `both` keep the original next to the conversion, separated by `/`
- `no-trad` skip the Traditional step
- `no-zhuyin` skip the Zhuyin step
