.. _NQRustBackupSystemtestsChapter:

Systemtests
===========

Systemtests use the locally compiled version of the nqrustbackup binaries
and run tests on them. Preparations also have been made to run the
tests on installed binaries (originating from packages).

Prepare the PostgreSQL database for systemtests
-----------------------------------------------

The |dir| requires a database backend. The default database is |postgresql|.
To run systemtests, some preparations are required.

The system user running the NQRustBackup systemtests
must be given permission to create databases and database user:

.. code-block:: shell-session
   :caption: Configure a PostgreSQL Server for systemtests

   user@host:~$ sudo -u postgres createuser --createdb --createrole $USER

.. _build-for-systemtest:

Build NQRustBackup for Systemtests
----------------------------

This following shell script will show how to build the NQRustBackup test-environment from source.

.. code-block:: bash
   :caption: Example shell script

   #!/bin/sh

   mkdir nqrustbackup-local-tests
   cd nqrustbackup-local-tests
   git clone https://github.com/nqrustbackup/nqrustbackup.git

   mkdir build
   cd build

   # configure build environment
   cmake -Dpostgresql=yes -Dsystemtest_db_user=$USER -Dtraymonitor=yes ../nqrustbackup

   # build NQRustBackup
   make

   # run system tests
   make test


Running Systemtests
-------------------

Tests are structured in directories.
Each test directory either contain
a single test script, named :file:`testrunner`
or multiple scripts named :file:`testrunner-*`
together with optional scripts :file:`test-setup` and :file:`test-cleanup`.
Having multiple test scripts combined reduces the overhead
but makes the scripts slightly more complicated,
as they must be able to run in arbitrary order.

Each test directory is designed to be independent,
so that different tests can be run in parallel without interfering with each other.

Run all system tests
~~~~~~~~~~~~~~~~~~~~

.. code-block:: shell-session
   :caption: List available ctests

   user@host:~$ cd nqrustbackup-local-tests/build
   user@host:~/nqrustbackup-local-tests/build$ ctest --show-only
   Test project ~/nqrustbackup-local-tests/build
     Test   #1: system:acl
     Test   #2: system:ai-consolidate-ignore-duplicate-job
     Test   #3: system:autochanger (Disabled)
     Test   #4: system:nqrustbackup
     Test   #5: system:nqrustbackup-acl
     Test   #6: system:nqrustbackup_console-pam (Disabled)
     Test   #7: system:nqrustbackup_console-status-client
     ...
     Test  #58: system:reload:setup
     Test  #59: system:reload:add-client
     Test  #60: system:reload:add-duplicate-job
     Test  #61: system:reload:add-empty-job
     Test  #62: system:reload:add-second-director
     Test  #63: system:reload:add-uncommented-string
     Test  #64: system:reload:unchanged-config
     Test  #65: system:reload:cleanup
     ...


.. code-block:: shell-session
   :caption: Run all system tests

   user@host:~$ cd nqrustbackup-local-tests/build
   user@host:~/nqrustbackup-local-tests/build$ make test

   Running tests...
   Test project ~/nqrustbackup-local-tests/build
         Start  1: system:acl
    1/88 Test  #1: system:acl ...........   Passed   15.81 sec
         Start  2: system:ai-consolidate-ignore-duplicate-job
   ...


Instead of using :command:`make test`, :command:`ctest` can be directly invoked.
This offers some advantages, like being able to run multiple tests in parallel with
:command:`ctest -j <number of parallel tests>`.
Only jobs with names matching a certain regular expression can be run with
:command:`ctest -R`, and verbose test output can be enabled with :command:`ctest -V`.
Please refer to the ctest documentation.

Run a single system test
~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: shell-session
   :caption: Run a single system test by ctest

   user@host:~$ cd nqrustbackup-local-tests/build
   user@host:~/nqrustbackup-local-tests/build$ ctest --verbose --tests-regex acl
   UpdateCTestConfiguration  from :~/nqrustbackup-local-tests/build/DartConfiguration.tcl
   Parse Config file:~/nqrustbackup-local-tests/build/DartConfiguration.tcl
   UpdateCTestConfiguration  from :~/nqrustbackup-local-tests/build/DartConfiguration.tcl
   Parse Config file:~/nqrustbackup-local-tests/build/DartConfiguration.tcl
   Test project ~/nqrustbackup-local-tests/build
   Constructing a list of tests
   Done constructing a list of tests
   Updating test list for fixtures
   Added 0 tests to meet fixture requirements
   Checking test dependency graph...
   Checking test dependency graph end
   test 1
       Start 1: system:acl

   1: Test command: ~/nqrustbackup-local-tests/build/systemtests/tests/acl/testrunner
   1: Test timeout computed to be: 1500
   1: creating database (postgresql)
   1: running ~/nqrustbackup-local-tests/build/systemtests/scripts/setup
   1:
   1:
   1: === acl: starting at 16:09:46 ===
   1: =
   1: =
   1: =
   1: =
   1: === acl: OK at 16:09:56 ===
   1/1 Test #1: system:acl ........   Passed   10.90 sec

   The following tests passed:
           system:acl

   100% tests passed, 0 tests failed out of 1

   Total Test time (real) =  10.91 sec

or change into a test directory and run :command:`testrunner` directly:

.. code-block:: shell-session
   :caption: Run a single system test by testrunner

   user@host:~$ cd nqrustbackup-local-tests/build
   user@host:~/nqrustbackup-local-tests/build$ cd tests/acl
   user@host:~/nqrustbackup-local-tests/build/tests/acl$ ./testrunner
   creating database (postgresql)
   running ~/nqrustbackup-local-tests/build/systemtests/scripts/setup


   === acl: starting at 15:03:20 ===
   =
   =
   =
   =
   === acl: OK at 15:03:35 ===


For verbose output, set ``export REGRESS_DEBUG=1`` before running :command:`testrunner`.


The test environment persists after running a test.
So to further debug a problem,
the NQRustBackup daemons can be started again,
and a :command:`nqrustbackup_console` session can be used to retrieve information:


.. code-block:: shell-session
   :caption: Doing manual tests in a test-environment

   user@host:~$ cd nqrustbackup-local-tests/build
   user@host:~/nqrustbackup-local-tests/build$ cd tests/acl
   user@host:~/nqrustbackup-local-tests/build/tests/acl$ bin/nqrustbackup status
   nqrustbackup-dir is stopped
   nqrustbackup-sd is stopped
   nqrustbackup-fd is stopped
   user@host:~/nqrustbackup-local-tests/build/tests/acl$ bin/nqrustbackup start
   Starting the  Storage daemon
   Starting the  File daemon
   Starting the  Director daemon
   Checking Configuration and Database connection ...
   user@host:~/nqrustbackup-local-tests/build/tests/acl$ bin/nqrustbackup status
   nqrustbackup-dir (pid 2782) is running...
   nqrustbackup-sd (pid 2761) is running...
   nqrustbackup-fd (pid 2770) is running...
   user@host:~/nqrustbackup-local-tests/build/tests/acl$ bin/nqrustbackup_console
   Connecting to Director localhost:42001
    Encryption: TLS_CHACHA20_POLY1305_SHA256
   1000 OK: nqrustbackup-dir Version: 19.1.2 (01 February 2019)
   self-compiled binary
   self-compiled binaries are UNSUPPORTED by nqrustbackup.com.
   Get official binaries and vendor support on https://www.nqrustbackup.com
   You are connected using the default console

   Enter a period to cancel a command.
   *list jobs
   Automatically selected Catalog: MyCatalog
   Using Catalog "MyCatalog"
   +-------+------------------+-----------+---------------------+------+-------+----------+----------+-----------+
   | JobId | Name             | Client    | StartTime           | Type | Level | JobFiles | JobBytes | JobStatus |
   +-------+------------------+-----------+---------------------+------+-------+----------+----------+-----------+
   | 1     | backup-nqrustbackup-fd | nqrustbackup-fd | 2019-08-15 15:04:37 | B    | F     | 21       | 138399   | T         |
   | 2     | RestoreFiles     | nqrustbackup-fd | 2019-08-15 15:04:41 | R    | F     | 21       | 138399   | T         |
   +-------+------------------+-----------+---------------------+------+-------+----------+----------+-----------+
   *

Add a systemtest
~~~~~~~~~~~~~~~~

If possible extend a systemtest already containing multiple scripts
by adding another :file:`testrunner-*` script to the test directory.

If this is not reasonable, a new systemtest is best created
by copying the existing systemtest
that matches the desired type of the new systemtest most.

The new test directory has to be listed
in :file:`systemtests/tests/CMakeLists.txt`.

Taking into concern system dependencies it could be necessary to disable
a test if the appropriate prerequisites for a test are not met. In this case
the test should be displayed as disabled when running the tests.

Adapt the test configuration and the :file:`testrunner` script to your requirements.

.. note::
   Configuration warnings are treated as errors in system tests.
   If your test relies on e.g. deprecated configuration options, you can disable this by
   passing `IGNORE_CONFIG_WARNINGS` to `create_systemtest` in the system test's `CMakeLists.txt` file.

Directory Structures
~~~~~~~~~~~~~~~~~~~~

Running cmake in the systemtest subdirectory will create the tests in the
build tree that is party symmetrical to the source tree as you can see on the
next diagrams.

Directory Structure (Source)
''''''''''''''''''''''''''''

::

      systemtests/tests/acl/
      |-- etc
      |   `-- nqrustbackup            -- nqrustbackup config for this test
      |       |-- nqrustbackup-dir.d
      |       |   |-- catalog
      |       |   |-- client
      |       |   |-- console
      |       |   |-- director
      |       |   |-- fileset
      |       |   |-- job
      |       |   |-- jobdefs
      |       |   |-- messages
      |       |   |-- pool
      |       |   |-- profile
      |       |   `-- storage
      |       |-- nqrustbackup-fd.d
      |       |   |-- client
      |       |   |-- director
      |       |   `-- messages
      |       |-- nqrustbackup-sd.d
      |       |   |-- device
      |       |   |-- director
      |       |   |-- messages
      |       |   `-- storage
      |       |-- nqrustbackup_console.conf.in
      |       `-- tray-monitor.d
      |           |-- client
      |           |-- director
      |           |-- monitor
      |           `-- storage
      `-- testrunner            -- the main script for this test

      or

      |-- test-cleanup          -- optional, falls back to ../../scripts/cleanup
      |-- test-setup            -- optional, falls back to ../../scripts/start_nqrustbackup.sh
      |-- testrunner-test1      -- script for test1
      |-- testrunner-test2      -- script for test2
      `-- ...                   -- more test scripts possible


Directory Structure (Build)
''''''''''''''''''''''''''''

::

      systemtests/tests/acl/
      |-- bin
      |-- etc
      |   `-- nqrustbackup
      |       |-- nqrustbackup-dir.d
      |       |   |-- additional_test_config
      |       |   |-- catalog
      |       |   |-- client
      |       |   |-- console
      |       |   |-- director
      |       |   |-- fileset
      |       |   |-- job
      |       |   |-- jobdefs
      |       |   |-- messages
      |       |   |-- pool
      |       |   |-- profile
      |       |   `-- storage
      |       |-- nqrustbackup-fd.d
      |       |   |-- client
      |       |   |-- director
      |       |   `-- messages
      |       |-- nqrustbackup-sd.d
      |       |   |-- device
      |       |   |-- director
      |       |   |-- messages
      |       |   `-- storage
      |       `-- tray-monitor.d
      |           |-- client
      |           |-- director
      |           |-- monitor
      |           `-- storage
      |-- log
      |-- python-modules
      |-- sbin
      |-- storage
      |-- tmp
      `-- working
