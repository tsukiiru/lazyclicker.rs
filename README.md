# lazyclicker.rs

<sub>by lazy people, for lazy people</sub>

a _lazy_ autoclicker for unix systems that support multiple profiles and holding!!

#### !heads up!

i have only tested on arch linux, no idea about other distro, but they should work!!  
global keybinds should work, if you can bind the cli commands to the keybinds system of your wm/de

#### features

- multiple profiles (each with custom configuration and mode)
- working as a background process
- global keybinds (kinda? not built-in)
- two modes: holding and clicking

#### setting up!

1. Head to [Releases](https://github.com/tsukiiru/lazyclicker.rs/releases) and grab the latest Release!
2. If needed, `chmod +x lazyclicker`
3. Move the _binary file_ (lazyclicker) to somewhere within your **PATH**
4. Run `lazyclicker init` for the first time and it will create a sample configuration file (often at `~/.config/lazyclicker`)

#### configurations

as this uses [toml](https://toml.io/) as the configuration language, so it's super easy to configure!!

Example `config.toml`:

```toml
[[profile]]
name = "clicktest"
interval = 1000
button = "Left"
repeat = 1
mode = "Click"

[[profile]]
name = "holdtest"
button = "Left"
mode = "Hold"
```

- `name`: the name of the profile
- `mode`: clicking mode (can be either `Click` or `Hold`)
- `button`: mouse button to click (can be either `Left` or `Right`)
- `interval`: time between clicks (in milliseconds)
- `repeat`: numbers of repeat each click

> [!WARNING]
> it HAS TO BE capitalized Left, Right, Click, and Hold

> [!NOTE]
> `interval` and `repeat` is optional for `Hold` mode, and compulsory for `Click` mode

#### commands

1. `lazyclicker init`: Initialize the sample configuration file and path
2. `lazyclicker list`: List all available profiles
3. `lazyclicker start [PROFILE_NAME]`: Start / Run a profile
4. `lazyclicker stop [PROFILE_NAME]`: Stop a running profile
5. `lazyclicker config`: Edit the profiles.toml configuration file in your default $EDITOR
