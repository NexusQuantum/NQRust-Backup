Hints for Building NQRustBackup
~~~~~~~~~~~~~~~~~~~~~~~~~

Configure (cmake) build settings
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. note::

   To build nqrustbackup, at least **cmake version 3.12** is required.


NQRustBackup cmake configuration allows a lot of different defines to be set.
For the test-environment, we use the minimal defines required to run the tests.

When interested in the cmake defines for the NQRustBackup production packages,
please refer to the corresponding build descriptions:

  * Debian Packages: `debian/rules <https://github.com/nqrustbackup/nqrustbackup/blob/master/debian/rules>`__
  * RPM Packages: `core/platforms/packaging/nqrustbackup.spec <https://github.com/nqrustbackup/nqrustbackup/blob/master/core/platforms/packaging/nqrustbackup.spec>`__


Using ccache (compiler cache)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

NQRustBackup can be built using ccache and using it will improve build times for repeated builds a lot.
Running cmake will autodetect and use ccache if it is available.
To get the most out of ccache, you should configure it to work correctly with NQRustBackup.

base_dir
   Set this to a common directory where your checked out sources and cmake binary-dir live.
   Your homedirectoy is probably a good starting point.
   This setting lets ccache ignore the path to files below this ``base_dir``.
   This makes sure you will get a cache hit even if the path to the source files changes.
hash_dir
   By disabling this, the current working directory will be ignored.
   In case of cmake the working directory does not matter, so ignoring this should be safe and will improve cache hits
sloppiness = file_macro
   This makes sure the value that ``__FILE__`` expands to is ignored when caching.
   You may end up with binaries that contain other paths in ``__FILE__``, but NQRustBackup only uses this to determine a relative path so this should not hurt.
   If you're using a modern compiler that supports ``-ffile-prefix-map`` this should not be required anymore.

.. code-block:: ini
  :caption: Example ccache.conf

  base_dir = /path/to/common/topdir
  hash_dir = false
  sloppiness = file_macro
