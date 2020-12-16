# `pizote`

The (not quite yet usable) rusty, code-first, cross-platform application/document framework.

Work in progress, recently started.

## Design goals

- Code-first
  - LaTeX works great until you try do do something complicated. Third party packages for graphs, trees, etc. often don't work very well.
- Cross-platform
  - Applications using the `pizote` APIs should be able to run on a different platform simply by compiling it with a different backend.
- Integration with a great package manager: promotes third-party integrations
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
  - No restrictions on what can be built. Right now, I have no traits restricting built components in any fashion, but I may provide some traits for suggested design patterns.
- Message-based dynamic operation
- Prevents over-inheritance
  - A big issue with CSS - everything tends to inherit styles and properties you don't want them to.
  - Rust is a great choice for this, because of its unique approach on inheritance in the form of traits.
- Super fast
  - In Rust!
  - Asynchronous operation
- Easily swap themes
  - `pizote::theme::Theme` trait will provide applications with a usage-defined color palette
  - Several default themes will be included in the `pizote` standard library.

## Bounds + Information = Component

This is the primary, suggested design pattern for `pizote` applications. If you publish a component, it should accept initial bounds and configuration information, which should include a `dyn pizote::theme::Theme` as part of it if it uses the `Draw` API.

## Information and actions performed through message passing.

Actions and information (described below) are passed between components with a root or destination at one of the backend implementations of the APIs shown below. This helps control flow of information, and prevent over-inheritance or unwanted interactions between components as present in HTML/CSS/Javascript.

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
  - A simple multimedia library perfect for a desktop application.
- Android
- IOS

## Pizote Draw API

- Cannot draw outside the component's bounds
- SVG-level-ish commands
- Coordinates and units are in terms of the component's bounds (start at 0,0 in the upper left corner)

## Roadmap

- Flesh out `draw` API. Test using SFML backend.
- Integrate `pizote` theme closely with the `draw` API.
- Flesh out the other APIs. Test using SFML backend.
- Write other backends incrementally, while revising interfaces. Priority: SFML, web, mobile
- Maintain and improve!

## Contributions and feedback

Feel free to create issues, contribute, and/or provide feedback. See the license and contributing guidelines (the latter is quite sparse at the moment).

Also, I value feedback highly! The project is in its earliest stages now; because of this, feedback is crucial! Please forward any feedback to `pizote@protonmail.com`. This could be anything regarding design, plans, or the like. Once there is enough interest, I may create weekly/biweekly meetings.

## Example components (most likely will be added to the `pizote` standard library)

- Components for graphing (perhaps a component that integrates `matplotlib`)
- Vertex-edge graphs and trees, with labelable nodes
- LaTeX math component
- Text-focused components:
 - Paragraphs, columns, etc.
 - Components for various article / academic journal formats.
 - BibTeX component
- Interactive elements
  - Text inputs
  - VIM component (VIM contained to a text-box on page)
  - `ipynb` style code edit-and-run blocks
- Markdown snippets

