# Rust Audio Switcher

A simple tool for listing and switching audio devices on Windows.

## Build

```bash
cargo build --release
```

## Usage

List Devices

```bash
./audio-switcher
```

```plain
eRender  | Default | {0.0.0.00000000}.{00000000-0000-0000-0000-000000000000} | Speakers
eRender  | -       | {0.0.0.00000000}.{00000000-0000-0000-0000-000000000001} | Headphones
eCapture | Default | {0.0.1.00000000}.{00000000-0000-0000-0000-000000000000} | Microphone
eCapture | -       | {0.0.1.00000000}.{00000000-0000-0000-0000-000000000001} | Stereo Mix
```

Set Default Device

```bash
./audio-switcher "{0.0.0.00000000}.{00000000-0000-0000-0000-000000000001}"
```

## License

[MIT License](https://choosealicense.com/licenses/mit/)
