# mt-dom

mt-dom is a generic virtual dom implementation which doesn't specify the types of the data that
is being processed. It's up to the library user to specify those types

The goal of this library is to provide virtual dom diffing functionality and return a portable
patches which the user can then use to apply those patches in their respective UI elements.

mt-dom is not limited to be used in html base virtual-dom implementation, but can also be use
for native UI elements.


License: MIT
