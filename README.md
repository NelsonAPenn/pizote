# `pizote`

The Rusty, code-first, cross-platform application/document framework.

## Design goals

- Code-first
  - LaTeX works great until you try do do something complicated. Third party packages for graphs, trees, etc. often don't work very well.
- Integration with a great package manager
  - Best not to reinvent the wheel. `cargo` is perfectly suited for the job.
- Sidestep HTML/CSS/Javascript
  - Most cross-platform solutions rely on this trio. However, when it comes to the issues below, I often find my self wishing for something else.
- Explicit functionality
  - You tell it what you want. Perhaps this makes it more verbose, but prevents a lot of issues deriving from implicit rules.
- Components are sandboxed, to some degree.
  - If bounds arbitration is used, components cannot draw outside the region you constrain them to.
- Control of flow of information
  - With message passing as a basis for actions performed or information accessed, the component you build has fine control over what its children can see/do.
- Flexible and fully hackable
  - No restrictions on what can be built.
- Message based dynamic operation
- Super fast

## Bounds + Information --> Component

A component receives bounds and information. Then it uses the bounds and information, allocating some to subcomponents. It can optionally give back bounds and information to the parent.

## `pizote` dynamic APIs

The cross-platform component comes in here. `pizote` defines action and information interfaces that can be used by components. It provides implementations of these interfaces for different backends. 

Here are some examples of interfaces:
- Action
  - draw
  - play audio
  - use filesystem
  - make network requests
- Information
  - webcam
  - keyboard input
  - location
  - microphone
  - mouse


If the application is a static document, these would be some appropriate backends:
- PDF
- SVG
- ePUB
- MOBI
- Web

If the application is dynamic, these would be some appropriate backends:
- Web
  - The plan would be to take a canvas-webassembly approach to compile and integrate the code.
- SFML
  - A simple multimedia library perfect for a `pizote` desktop application.
- Android
- IOS

For the purposes of this hackathon, I only worked with the `draw` interface and the `SFML` backend.

## Pizote Drawing APIs

- Cannot draw outside the component's bounds
- SVG-level commands
- Coordinates and units are in terms of the component's bounds (start at 0,0 in the upper left corner)

