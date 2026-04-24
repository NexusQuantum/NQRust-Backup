Build the documentation
=======================

This following shell script will show how to build the **NQRustBackup documentation** from source.

.. code-block:: bash
  :caption: Example shell script

  #!/bin/sh

  mkdir nqrustbackup-local-tests
  cd nqrustbackup-local-tests
  git clone https://github.com/nqrustbackup/nqrustbackup.git

  mkdir build-docs
  cd build-docs

  cmake -Ddocs-only=yes ../nqrustbackup
  make
