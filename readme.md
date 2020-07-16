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

_Note: in case of both words being shorter than two characters, e.g. "I" and "a", function will return a 'nan' value._


TO DO: 
* Catch 1-letter words in the function itself and return an error message.
* Currently only compiled for Linux and Windows 10 - compile for MacOS if possible.