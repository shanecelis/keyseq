# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

## v0.5.0
- Update to bevy 0.15.

## v0.4.1
- Use curly braces for macros throughout code and documentation.

- Add curly brace example code for macros for [better autocompletion] (https://github.com/not-elm/bevy-input-sequence/pull/10#issuecomment-2525100631).

## v0.4.0
- Capitalize modifiers:`Ctrl-A` instead of `ctrl-A`.

Why? Because the majority of hotkey notations use uppercase. Also modifiers like
"super" are reserved keywords in rust and although permitted inside a macro,
they are often highlighted as though they are reserved words.

- Add "permit-plus" feature so `Ctrl+A` is acceptable.

The majority of hotkey notations use plus, so I bend to their use.

- Add logical key macros for Bevy.

- Update to winit 0.30.

## v0.3.0
- Update to bevy 0.14.

## v0.2.3

- Implement Display for Modifiers.
- Derive Reflect for Modifiers when "bevy" feature is present.

## v0.2.2

- Update to bevy 0.13.

## v0.2.1

### Refactor

- Remove debug `eprintln!` in macro code.

## v0.2.0

### Features

- Add support for bevy 0.13.

## v0.1.1

### Refactor

- Remove debug `eprintln!` in macro code.

## 0.1.0

- Initial release
