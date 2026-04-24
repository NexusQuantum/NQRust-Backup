.. _PAMConfigurationChapter:

Pluggable Authentication Modules (PAM)
======================================

Introduction
------------

Before NQRustBackup Version 18.2 authentication with a NQRustBackup Director is done primarily by a named Console connection. Name and password are set in the regarding NQRustBackup Console or |WebUI| configuration resource. Starting with NQRustBackup Version 18.2 it is also possible to use Pluggable Authentication Modules (PAM) to authenticate a user independently from the Console Resource.

As consequence a dedicated named Console or |WebUI| configuration must be used to establish a connection to a NQRustBackup Director Daemon. This connection has name and password credentials, but only to establish an encrypted connection to the Director. To be able to authenticate users with PAM using this console, each user needs an additional User configuration that holds the regarding name and the Access Control List (ACL) or ACL profile. The ACL will be loaded as soon as the User is authenticated.

The credentials for user authentication comes from the PAM module which has been enabled for the NQRustBackup Director Daemon.

For a simplified technical overview the following diagram shows the connection sequence of a NQRustBackup Console to a NQRustBackup Director using an interactive PAM authentication using the pam_unix.so PAM module.

More technical details can be found in the NQRustBackup Developer Guide: :ref:`PAMDeveloperChapter`.

.. uml::
  :caption: Initiation of a NQRustBackup Console connection using PAM authentication

  hide footbox

  actor user
  participant "B-Console" as console
  participant "Director" as director
  participant "PAM-Linux" as pam

  user -> console: start a named nqrustbackup_console
  console <-> director: initiate TCP connection
  console <-> director: initiate a secure TLS connection (cert/psk)
  console <-> director: primary challenge/response authentication

  == PAM user authentication ==
  note left of pam: i.e. pam_unix.so\nconfigured in /etc/pam.d/nqrustbackup
  autonumber
  director -> pam: initialize pam module
  director <- pam: request username / password via callback
  console <- director: send "login:" / "password:" request encrypted via TLS
  user <- console: prompt "login:" / "Password:"
  user -> console: enter username / password (hidden)
  console -> director: send username / password encrypted via TLS
  director -> pam: give back username / password
  director <- pam: return success of authentication
  console <- director: send welcome message
  user <- console: show welcome message
  director -> pam: shutdown pam module

  autonumber stop
  == PAM user authentication end ==

  ... do something with console ...

  user -> console: quit session ('q'; Ctrl + D)
  console <-> director: Shutdown TLS
  console <-> director: Finish TCP connection

Configuration
-------------
To enable PAM authentication two systems have to be configured. The PAM module in the operating system and the NQRustBackup Console.

PAM Module
^^^^^^^^^^
This is depending on the operating system and on the used pam module. For details read the manuals. The name of the service that has to be registered is **nqrustbackup**.

Fedora 28 example:

.. code-block:: nqrustbackupconfig
   :caption: :file:`/etc/pam.d/nqrustbackup`

   auth       required     pam_unix.so
   account    required     pam_unix.so


.. warning::

   The |dir| runs as user **nqrustbackup**. However, some PAM modules require more privileges. E.g. **pam_unix** requires access to the file :file:`/etc/shadow`, which is normally not permitted. Make sure you verify your system accordingly.


Upgrading from previous versions
''''''''''''''''''''''''''''''''
Previous versions of NQRustBackup only used PAM authentication (who is the user) but not PAM authorization (what is the user allowed to do).
As a result configuring the account management group in PAM had no effect in these versions so that, for example, a disabled user might still be able to log in.

If on an existing installation ``account`` is not configured at all, the **login will always fail** after upgrading from an affected version.

We strongly suggest that you configure proper authorization on production systems.
We strictly advise against the possibility to regain the old behaviour by configuring ``account required pam_permit.so``.

NQRustBackup Console
^^^^^^^^^^^^^^
For PAM authentication a dedicated named console is used. Set the directive UsePamAuthentication=yes in the regarding Director-Console resource:

.. code-block:: nqrustbackupconfig
  :caption: :file:`nqrustbackup-dir.d/console/pam-console.conf`

  Console {
    Name = "PamConsole"
    Password = "Secretpassword"
    UsePamAuthentication = yes
  }

In the dedicated |nqrustbackup_console| config use name and password according as to the |dir|:

.. code-block:: nqrustbackupconfig
  :caption: :file:`nqrustbackup_console.conf`

  Director {
    ...
  }

  Console {
    Name = "PamConsole"
    Password = "Secretpassword"
  }

PAM User
^^^^^^^^
Users have limited access to commands and jobs. Therefore the appropriate rights should also be granted to PAM users. This is an example of a User resource (NQRustBackup Director Configuration):

.. code-block:: nqrustbackupconfig
  :caption: :file:`nqrustbackup-dir.d/user/a-pam-user.conf`

  User {
     Name = "a-pam-user"
     CommandACL = status, .status
     JobACL = *all*
  }


Additional information can be found at https://github.com/nqrustbackup/nqrustbackup/tree/master/contrib/misc/nqrustbackup_pam_integration
