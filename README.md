# pizote, the Rusty, code-first, cross-platform platform 

## Design goals

- Do away with HTML/CSS/Javascript and their jankiness
- Full control
- Deterministic
- Components cannot interact with siblings
- Control of flow of information
- Fully hackable
- Message based dynamic operation
- Super fast

## Bounds + Information --> Component

A component receives bounds and information. Then it uses the bounds and information, allocating some to subcomponents. It can optionally give back bounds and information to the parent.

## Pizote Dynamic APIs

- Location
- Camera
- Mouse events
- Keyboard events,
- etc.

## Pizote Drawing APIs

- Cannot draw outside the component's bounds
- SVG-level commands
- Coordinates and units are in terms of the component's bounds (start at 0,0 in the upper left corner)


## Backends

Here is the cross-platform part.

Changing the backend swaps out APIs with identical interfaces, which work for different platforms.

- SVG
- SFML
  - good choice for desktop client
- PDF
- Web
  - one canvas, webassembly integration
- Android?
- IOS?
