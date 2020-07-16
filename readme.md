### Sørensen–Dice coefficient calculator

https://en.wikipedia.org/wiki/Sørensen–Dice_coefficient

Calculates the Sørensen–Dice coefficient between two strings.

Made into a Python module with PyO3 crate (original Rust program here: https://github.com/nebelgrau77/sorensen_dice_coeff)

Currently only compiled for Linux (_sdcoeff.so_ file).


__Usage__: copy _sdcoeff.so_ into the folder where you're running Python.

```python
from sdcoeff import coefficient

# usage: coefficient(wordA, wordB)

c = coefficient('Philadelphia', 'Philladelphia')

```

Note: in case of both words being shorter than two characters, e.g. "I" and "a", function will return a 'nan' value.