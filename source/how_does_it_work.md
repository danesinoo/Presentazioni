## General operation:

The program takes as input a folder containing Markdown files and transforms 
them into HTML pages. Each Markdown section of the header type (indicated by #, 
##, ####, etc.) becomes a slide in the resulting HTML page. The content under 
each header constitutes the content of the slide.

## Program Startup:
The program starts by executing its executable followed by two flags:
- ``-s path/to/source/directory`` specifies the input folder.
- o ``path/to/output/directory`` specifies the output folder.

## Tip:
It is advisable to copy the "assets" folder containing the necessary CSS and 
JavaScript files to make the HTML pages dynamic as described above.
