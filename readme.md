# Tablisp

Proof of concept Rust project implementing a table processing tool with a vim-navigation-based GUI and LISP-like language for calculations.

GUI is created using macroquad game library and is composed only out of rectangles and text elements.

Content gets parsed from a CSV-file and cells starting with parantheses will be evaluated using the custom Lisp-interpreter which supports referencing of other cells and ranges.

## Setup

Run:
```
cd tablisp/src

cargo run
```

Currently the CSV file located in assets/text.csv is getting opened at the start.  

## Usage
Navigate:  ``hjkl``  
Edit Cell: ``i``  
Submit: ``Enter``  

### Calculations
Supported operations: ``+, -, \*, /, >, if, max, min``  
Syntax is just standard Lisp: ``(+ (\* 2 2) 3) => (+ 4 3) => 7``  

Cells can be referenced as expected using the collumn letter followed by the row number eg. ``A1, C5``  
Ranges can be written using ':' and will be expanded: ``(+ A1:A3) => (+ A1 A2 A3)``  

Cells will be re-evaluated whenever enter gets pressed.

## Assets
font: https://fontstorage.com/font/google/roboto-mono
