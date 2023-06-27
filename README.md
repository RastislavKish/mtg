# mtg

A utility for toggling the monitor state on Linux. Currently supports X through xrandr.

## Usage

To check the monitor status:

```
mtg status
```

Toggle the state between on and off:

```
mtg toggle
```

Apply a specific option:

```
mtg toggle --off
```

Make the program output through desktop notifications (useful for shortcut bindings):

```
mtg toggle -q --notify
```

See:

```
mtg --help
```

For more details.

### A note about multimonitor setups

mtg does not properly support multiple monitors at this point. If more than 1 connected devices are detected, the first-one found is chosen as the primary.

User feedback is necessary to asses the practical situations and determine the approach the program should use in similar scenarios.

## Build

Make sure to have [Rust](https://rust-lang.org) installed. Then just do:

```
git clone https://github.com/RastislavKish/mtg
cd mtg
cargo build --release -q
```

## License

Copyright (C) 2023 Rastislav Kish

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, version 3.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.

