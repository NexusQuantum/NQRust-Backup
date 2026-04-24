.. _section-Debugging:

Debugging
=========

.. index::
   single: Crash
   single: Debug; crash

This chapter describes how to debug NQRustBackup when the program crashes. If you are just interested about how to get more information about a running NQRustBackup daemon, please read :ref:`section-debug-messages`.

If you are running on a Linux system, and you have a set of working configuration files, it is very unlikely that NQRustBackup will crash. As with all software, however, it is inevitable that someday, it may crash.

This chapter explains what you should do if one of the three daemons (|dir|, |fd|, |sd|) crashes. When we speak of crashing, we mean that the daemon terminates abnormally because of an error. There are many cases where NQRustBackup detects errors (such as PIPE errors) and will fail a job. These are not considered crashes. In addition, under certain conditions, NQRustBackup will detect a fatal in the configuration, such as lack of permission to read/write the working directory. In that case, NQRustBackup will force itself to crash with a SEGFAULT. However, before crashing, NQRustBackup will normally display a message indicating why. For more details, please read on.


Traceback
---------

.. index::
   single: Traceback

Each of the three NQRustBackup daemons has a built-in exception handler which, in case of an error, will attempt to produce a `traceback`. If successful the `traceback` will be emailed to you and stored into the working directory (usually :file:`/var/lib/nqrustbackup/storage/nqrustbackup.<pid_of_crashed_process>.traceback` on linux systems).

For this to work, you need to ensure that a few things are setup correctly on your system:

#. You must have a version of NQRustBackup with debug information and not stripped of debugging symbols. When using a packaged version of NQRustBackup, this requires to install the NQRustBackup debug packages (**nqrustbackup-debug** on RPM based systems, **nqrustbackup-dbg** on Debian based systems).

#. On Linux, :command:`gdb` (the GNU debugger) must be installed. On some systems such as Solaris, :command:`gdb` may be replaced by :command:`dbx`.

#. By default, :command:`btraceback` uses :command:`nqrustbackup_smtp` to send the `traceback` via email. Therefore it expects a local mail transfer daemon running. It send the `traceback` to root@localhost via :strong:`localhost`.

#. Some Linux distributions, e.g. Ubuntu:index:`\ <single: Platform; Ubuntu; Debug>`\ , disable the possibility to examine the memory of other processes. While this is a good idea for hardening a system, our debug mechanism will fail. To disable this `ptrace protection`, run (as root):

   .. code-block:: shell-session
      :caption: disable ptrace protection to enable debugging (required on Ubuntu Linux)

      root@host:~# test -e /proc/sys/kernel/yama/ptrace_scope && echo 0 > /proc/sys/kernel/yama/ptrace_scope

If all the above conditions are met, the daemon that crashes will produce a traceback report and email it. If the above conditions are not true, you can run the debugger by hand as described below.

Testing The Traceback
---------------------

.. index::
   single: Traceback; Test

To "manually" test the traceback feature, you simply start NQRustBackup then obtain the PID of the main daemon thread (there are multiple threads). The output produced here will look different depending on what OS and what version of the kernel you are running.

.. code-block:: shell-session
   :caption: get the process ID of a running NQRustBackup daemon

   root@host:~# ps fax | grep nqrustbackup-dir
    2103 ?        S      0:00 /usr/sbin/nqrustbackup-dir

which in this case is 2103. Then while NQRustBackup is running, you call the program giving it the path to the NQRustBackup executable and the PID. In this case, it is:

.. code-block:: shell-session
   :caption: get traceback of running NQRustBackup director daemon

   root@host:~# btraceback /usr/sbin/nqrustbackup-dir 2103

It should produce an email showing you the current state of the daemon (in this case the Director), and then exit leaving NQRustBackup running as if nothing happened. If this is not the case, you will need to correct the problem by modifying the :command:`btraceback` script.

Getting A Traceback On Other Operating System
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. index::
   single: Traceback; Other System

It should be possible to produce a similar backtrace on operating systems other than Linux, either using :command:`gdb` or some other debugger.
:index:`Solaris <single: Platform; Solaris; Debug>`\  with :command:`dbx` loaded works quite fine. On other systems, you will need to modify the :command:`btraceback` program to invoke the correct debugger, and possibly correct the :file:`btraceback.gdb` script to have appropriate commands for your debugger.
Please keep in mind that for any debugger to work, it will most likely need to run as root.


Manually Running NQRustBackup Under The Debugger
------------------------------------------

.. index::
   single: gdb NQRustBackup; debugger

If for some reason you cannot get the automatic `traceback`, or if you want to interactively examine the variable contents after a crash, you can run NQRustBackup under the debugger. Assuming you want to run the Storage daemon under the debugger (the technique is the same for the other daemons, only the name changes), you would do the following:

#. The Director and the File daemon should be running but the Storage daemon should not.

#. Start the Storage daemon under the debugger:

   .. code-block:: shell-session
      :caption: run the NQRustBackup Storage daemon in the debugger

      root@host:~# su - nqrustbackup -s /bin/bash
      nqrustbackup@host:~# gdb --args /usr/sbin/nqrustbackup-sd -f -s -d 200
      (gdb) run

   NQRustBackup Parameter:

   -f
      foreground

   -s
      no signals

   -d nnn
      debug level

   See section :ref:`daemon command line options <section-daemon-command-line-options>` for a detailed list of options.

#. At this point, NQRustBackup will be fully operational.

#. In another shell command window, start the Console program and do what is necessary to cause NQRustBackup to die.

#. When NQRustBackup crashes, the gdb shell window will become active and gdb will show you the error that occurred.

#. To get a general traceback of all threads, issue the following command:

   .. code-block:: shell-session
      :caption: NQRustBackup Storage daemon in a debugger session

      (gdb) thread apply all bt

   After that you can issue any debugging command.


Core debugging
--------------

.. index::
   single: Core debugging; core

If a `SEGV` occurs, and you don't have anything installed, then a core file is created. Please follow below instructions to get it debugged on your system (or a clone of it).

For some reason, you may be not able to install the debug symbols nor the debugger tool on your NQRustBackup instance.
By collection the generated core file, you will be able to produce a `traceback` on a similar or cloned system.

#. Get a clone of your operating system.
   This is important as the same version of all installed packages need to present.

#. Install NQRustBackup and the debug symbols packages. see :ref:`appendix/debugging:install-debug-packages`

#. Install the debug tools (:command:`gdb` under Linux for example).

#. Transfer the previously generated core dump.

#. Debug the core.

   .. code-block:: shell-session

      gdb /usr/sbin/nqrustbackup-dir /tmp/core.nqrustbackup-dir.25972
      (gdb) backtrace



.. _appendix/debugging:install-debug-packages:

Installing debug symbols packages
---------------------------------

.. index::
   single: debug symbols package; dbg; debuginfo; debug

Our binaries do not contain debug symbols, but as they are needed for proper debugging, we package the debug symbols separately.

.. code-block:: shell-session
   :caption: Installing nqrustbackup debug symbols package on deb system

   apt install nqrustbackup-dbg gdb


.. code-block:: shell-session
   :caption: Installing all NQRustBackup debug symbols on (RH)EL system

   dnf debuginfo-install nqrustbackup-*-debuginfo


.. code-block:: shell-session
   :caption: Installing all NQRustBackup debug symbols on (open)SUSE system

   zypper --plus-content nqrustbackup-debuginfo --plus-content nqrustbackup-debugsource install nqrustbackup*debuginfo nqrustbackup*debugsource



.. Note:: If you want to debug only a specific daemon, you only need to install its related **-debuginfo** packages. For example with |fd|

.. code-block:: shell-session
   :caption: Installing |fd| debug symbols package on deb system

   apt install nqrustbackup-dbg gdb


.. code-block:: shell-session
   :caption: Installing |fd| debug symbols on (RH)EL system

   dnf debuginfo-install gdb nqrustbackup-debuginfo nqrustbackup-filedaemon*-debuginfo nqrustbackup-common-debuginfo nqrustbackup-debugsource


.. code-block:: shell-session
   :caption: Installing |fd| debug symbols on (open)SUSE system

   zypper --plus-content nqrustbackup-debuginfo --plus-content nqrustbackup-debugsource install gdb nqrustbackup-debuginfo nqrustbackup-filedaemon*-debuginfo nqrustbackup-common-debuginfo nqrustbackup-debugsource


You may encounter a message like `ptrace: Operation not permitted.` in the traceback file if you
have loaded the security module [Yama](https://www.kernel.org/doc/Documentation/security/Yama.txt).
This module restricts processes from inspecting the memory of other processes.
Note that parents may still inspect their children, so  “gdb nqrustbackup-dir” should still work.

   .. code-block:: shell-session
      :caption: To enable debugging running programs, as root do:

      echo 0 > /proc/sys/kernel/yama/ptrace_scope


   .. code-block:: shell-session
      :caption: To enable debugging running programs permanently, as root do:

      echo kernel.yama.ptrace_scope = 0 > /etc/sysctl.d/10-ptrace.conf
