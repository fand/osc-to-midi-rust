# osc-to-midi-rust

> Convert OSC messages to MIDI and send them to MIDI devices

![Example GIF](https://user-images.githubusercontent.com/1403842/45039659-0087f980-b09f-11e8-94ee-a0e7614b3053.gif)


## Usage

```sh
$ list-ports
0: IAC Driver IAC Bus 1
1: Launchpad Mini

$ osc-to-midi 127.0.0.1:9000 1
```

Then OSC messages sent to `127.0.0.1:9000` will be converted to MIDI messages and sent to `Launchpad Mini`.
For example, send an OSC message using [oscer](https://github.com/aike/oscer):

```
$ oscer localhost 9000 /midi 144 1 127
```

then a MIDI message `0x90 0x01 0x7F` will be sent to `Launchpad Mini`.

# LICENCE
MIT
