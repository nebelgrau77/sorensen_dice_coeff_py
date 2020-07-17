### Sørensen–Dice coefficient calculator

Calculates the Sørensen–Dice coefficient between two strings:

https://en.wikipedia.org/wiki/Sørensen–Dice_coefficient



Turned into a Python module with PyO3 crate (original Rust program here: https://github.com/nebelgrau77/sorensen_dice_coeff).

More than twice as fast than a pure Python function (timed with %timeit in JupyterLab).



__How to use__:

Copy file: 

* _sdcoeff.so_ (Linux) or
* _sdcoeff.pyd_ (Windows) 

into the folder where you're running Python.

```python
from sdcoeff import coefficient

# syntax: coefficient(wordA, wordB)

c = coefficient('Philadelphia', 'Philladelphia')

```

_Note: if any of the words is shorter than two characters, i.e. doesn't have a single bigram, function will return 0._


TO DO: 
* Currently only compiled for Linux and Windows 10 - compile for MacOS if possible.