.. _appendix-chapter-faq:

FAQ
===

General Questions
-----------------

How do I report a security issue?
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
The process for handling security-related problems is described in our GitHub `security policy`_.

.. _security policy: https://github.com/nqrustbackup/nqrustbackup/security/policy

.. _nqrustbackup-1920-updatefaq:

Upgrade of NQRustBackup 19.2 with installed Python plugin on Debian
-------------------------------------------------------------

.. index::
   single: Platform; Debian
   single: nqrustbackup-19.2; Upgrade with Python packages on Debian

On Debian platform, if you update from NQRustBackup <=19.2 to NQRustBackup >= 20 with any Python plugin installed, you will face some difficulties with :command:`apt upgrade`.
This is due to the renaming of Python plugin packages into python2 packages and introducing the python3 packages. apt/dpkg is not able to handle this situation alone.


.. code-block:: shell-session
   :caption: Error upgrading from 19.2 with apt upgrade

   apt upgrade nqrustbackup nqrustbackup-director nqrustbackup-filedaemon
   Reading package lists... Done
   Building dependency tree
   Reading state information... Done
   Calculating upgrade... Done
   Some packages could not be installed. This may mean that you have
   requested an impossible situation or if you are using the unstable
   distribution that some required packages have not yet been created
   or been moved out of Incoming.
   The following information may help to resolve the situation:

   The following packages have unmet dependencies:
      nqrustbackup-director-python-plugin : Depends: nqrustbackup-common (= 19.2.12-2) but 21.1.2-1 is to be installed
      nqrustbackup-filedaemon-python-plugin : Depends: nqrustbackup-common (= 19.2.12-2) but 21.1.2-1 is to be installed
   E: Broken packages


In this case, it is advised to use :command:`apt` with `full-upgrade` and directly move to new recommended python3 plugin.

.. code-block:: shell-session
   :caption: for full upgrade to python3

   apt full-upgrade nqrustbackup-director-python3-plugin nqrustbackup-filedaemon-python3-plugin


.. _nqrustbackup-1825-updatefaq:

NQRustBackup 18.2.5 FAQ
--------------------

What is the important feature introduced in NQRustBackup 18.2?
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

#. A new network protocol was introduced where TLS is immediately used.

  * When no certificates are configured, the network connection will still be
    encrypted using TLS-PSK.
  * When certificates are configured, NQRustBackup will configure both TLS-PSK and
    TLS with certificates at the same time, so that the TLS protocol will
    choose which one to use.

How to update from NQRustBackup 17.2?
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

To update from NQRustBackup 17.2, as always all core components need to be updated as
they need to be of the same NQRustBackup version (|nqrustbackup_console|, |nqrustbackupDir|, |nqrustbackupSd|).

How can I see what encryption is being used?
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Whenever a connection is established, the used cipher is logged and will be
shown in the job log and messages output:


.. code-block:: sh
   :caption: console output

   Connecting to Director localhost:9101
    Encryption: ECDHE-PSK-CHACHA20-POLY1305



.. code-block:: sh
   :caption: job log

   [...] JobId 1: Connected Storage daemon at nqrustbackup:9103, encryption: ECDHE-PSK-CHACHA20-POLY1305

What should I do when I get "TLS negotiation failed"?
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

NQRustBackup components use TLS-PSK as default. When the TLS negotiation fails then most likely identity
or password do not match. Doublecheck the component name and password in the respective configuration
to match each other.

How does the compatibility with old clients work?
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
The NQRustBackup Director always connects to clients using the new immediate TLS
protocol.  If that fails, it will fall back to the old protocol and try to
connect again.

When the connection is successful, the director will store which protocol needs
to be used with the client and use this protocol the next time this client will
be connected.  Whenever the configuration is reloaded, the protocol information
will be cleared and the probing will be done again when the next connection to
this client is done.

.. code-block:: sh
   :caption: probing the client protocol

   [...] JobId 1: Probing... (result will be saved until config reload)
   [...] JobId 1: Connected Client: nqrustbackup-fd at localhost:9102, encryption: ECDHE-PSK-CHACHA20-POLY1305
   [...] JobId 1:    Handshake: Immediate TLS



Does NQRustBackup support TLS 1.3?
^^^^^^^^^^^^^^^^^^^^^^^^^^^^
Yes. If NQRustBackup is compiled with OpenSSL 1.1.1, it will automatically use TLS
1.3 where possible.


Are old NQRustBackup clients still working?
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

NQRustBackup clients < 18.2 will still work, and the old protocol will be used.
This was mostly tested with NQRustBackup 17.2 clients.



Can I use a new NQRustBackup 18.2 client with my NQRustBackup 17.2 system?
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Yes, it is possible to use a NQRustBackup 18.2 client, but some changes need to be done
in the configuration.

It is possible to use the NQRustBackup 18.2 client with a NQRustBackup 17.2 Server. However,
the new immediate TLS Protocol and TLS-PSK are not usable, as the server
components do not support it. This also means that it is **not** possible to
use TLS with certificates in this setup. The communication will be unencrypted
using the old protocol.

As in NQRustBackup 18.2, the default value of **TLS Enable** was changed to **yes** to
automatically use TLS-PSK, and the meaning of **TLS Require** also was altered
so that it enforces the new protocol, these settings need to be changed.

In order to make NQRustBackup 18.2 clients work with a NQRustBackup 17.2 server, the following
changes need to be done:

* **On all NQRustBackup 18.2 clients**, the directive **TLS Enable** in the file
  :file:`/etc/nqrustbackup/nqrustbackup-fd.d/director/nqrustbackup-dir.conf` needs to be set to **no**.
  If the directive **TLS Require** is set, it also needs
  to be set to **no** in the same file.
  This is enough for standard clients which do not have any special setup for the
  connections, and also for clients that are configured to use **client initiated
  connections**.

* For **clients that use the passive mode**, also the clients' setting in the
  NQRustBackup 17.2 director in file :file:`/etc/nqrustbackup/nqrustbackup-dir.d/client/passive-fd.conf` needs
  to to be altered so that both directives **TLS Enable**
  and **TLS Require** are set to **no**.
