# `pizote`

The (not quite yet usable) rusty, code-first, cross-platform application/document framework.

Work in progress, recently started.

## Design goals

- Code-first
  - LaTeX works great until you try do do something complicated. Third party packages for graphs, trees, etc. often don't work very well.
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
  - No restrictions on what can be built.
- Message-based dynamic operation
- Prevents over-inheritance
  - A big issue with CSS - everything tends to inherit styles and properties you don't want them to.
  - Rust is a great choice for this, because of its unique approach on inheritance in the form of traits.
- Super fast
- Easily swap themes
  - `pizote::theme::Theme` trait will provide applications with a usage-defined color palette
  - Several default themes will be included in the `pizote` standard library.

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
  - A simple multimedia library perfect for a `pizote` desktop application.
- Android
- IOS

## Pizote Draw API

- Cannot draw outside the component's bounds
- SVG-level commands
- Coordinates and units are in terms of the component's bounds (start at 0,0 in the upper left corner)

## Example components (most likely will be added to the `pizote` standard library)

- Components for graphing
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

