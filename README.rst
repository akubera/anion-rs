============
Anion (rust)
============

Anion is an implementation of Amazon's `ion data notation`_ format.

`Documentation <https://akubera.github.io/anion-rs/docs/anion/>`_

dev-branch
  |travis-dev| |coveralls-dev|


About
-----

The `standard implementation`_ of Ion is written in java.
It might be nice to have an implementation in a runtime-less language,
and this developer wanted to learn more `rust`_; this project was
created thusly.

The name was chosen to distinguish itself from other projects and any
other uses of the word 'ion', as well as a unique name amongst other
implementations of the spec (in case "specializations" arise).
Also, an *anion* is a negatively charged ion; the anion :math:`\text{OH}^{-}`
may bind with positive iron nulcei :math:`\text{Fe}^{3+}` and form,
of course, rust - reminding all of us the base implementation's language.


Goals
-----

To make a fast and reliable low-level library for use in other projects.


License
-------

This project is licensed under the same license as the original Ion
project, `Apache 2.0`_.


.. _ion data notation: http://amznlabs.github.io/ion-docs/index.html
.. _standard implementation: https://github.com/amznlabs/ion-java/
.. _rust: https://rust-lang.org/
.. _Apache 2.0: http://www.apache.org/licenses/LICENSE-2.0


.. |travis-dev| image:: https://travis-ci.org/akubera/anion-rs.svg?branch=dev
               :target: https://travis-ci.org/akubera/anion-rs
               :alt:    Testing Report (dev branch)

.. |coveralls-dev| image:: https://coveralls.io/repos/github/akubera/anion-rs/badge.svg?branch=dev
                  :target: https://coveralls.io/github/akubera/anion-rs?branch=dev
                  :alt:    Coverage Report (dev branch)
