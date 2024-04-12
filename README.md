# augment-wallman

Manages wallpapers for Hyprland.

## Installation

### Cargo

    sudo cargo install augment-wallman --root /usr/local

> [!IMPORTANT]
> `augment-wallman` must be installed outside the user home directory due permission pupposes.

## Usage

You can just run `augment-wallman` right away but it won't start at boot. Add `exec-once = augment-wallman` to your `~/.config/hypr/hyprland.conf` file.

```
# ~/.config/hypr/hyprland.conf

...
exec-once = augment-wallman
...
```

To change the wallpaper, you can replace `~/.config/background` with any wallpaper you want. The config for `augment-wallman` can be found in `~/.config/augment/wallman.toml`.

## License

This is licensed under the GPL-3.0 License.
