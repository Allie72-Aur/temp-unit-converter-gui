# How To Write A Basic GUI in Rust

## Project Goal

The objective is to create a simple, functional desktop
application that allows a user to input a temperature value, select a source
unit and a target unit from dropdown menus, and see the converted result.

## Note

I'm not starting this project from scratch, I already wrote a cli that
does just this, the task at hand is to refactor and modify that code
to satisfy the requirements of this project.
This will be done in two steps:

- Seperate the Conversion Logic into its own function
- Make a basic GUI to make the program more user-friendly

## Crates I'm Using

For this project, the following dependencies are needed:

- GUI Library: `egui`
- GUI Framework: `eframe`

## Exactly What are the Minimum Requirements

- **User Interface**
  Display a window containing an input field for a numeric value, two
  dropdown menus for the source and target units, possibly a `Convert`
  action button, and finally an area to display the conversion results
  or error info.

- **Underlying Conversion Logic**
  The code that does the math should produce correct answers. Moreover,
  the code should be well-structured, easy to understand, and easy to
  extend or modify.
  To achieve those goals; the conversion logic should be seperated from
  the UI code.

## Checklist of Tasks

These are the directions that lead to the goal:

- [X] Extract unit conversion logic to a seperate mod
- [ ] Open a basic window
- [ ] Add an input field for a numeric value
- [ ] Add a dropdown menu for choosing the source unit
- [ ] Add a dropdown menu for choosing the target unit
- [ ] Make the conversion, and display the result
      or an error as appropriate
