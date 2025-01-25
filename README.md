# ccount

A character frequency counter command-line tool.

This module provides functionality to analyze text and count character
frequencies. It supports various options for sorting and filtering the results,
including:

- Sorting by character or frequency count
- Displaying frequency as percentages
- Showing only top N most frequent characters
- Filtering characters by frequency thresholds
- Full Unicode support for non-ASCII text analysis

## Example

```bash
$ echo "hello world" | ccount --sort-by count --show-top-n 2
l: 3
o: 2
```

The tool can read input either from command line arguments or standard input,
making it flexible for various use cases including pipeline operations. Unicode
support means it can analyze text in any language or script system.
