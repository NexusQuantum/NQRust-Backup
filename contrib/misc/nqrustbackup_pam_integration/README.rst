NQRustBackup PAM Integration
======================

PAM, the *Pluggable Authentication Modules* used by Linux
provide dynamic authentication support for applications and services in a Linux system.

PAM authentication is included since NQRustBackup >= 18.2, see https://docs.nqrustbackup.org/master/TasksAndConcepts/PAM.html#configuration

However, this supports only the authentication mechanism.
That means, the user must be known in the backend system used by PAM  (:file:`/etc/passwd`, LDAP or ...)
**and** the user has to exist in the NQRustBackup Director.

The PAM implementation of NQRustBackup is only used for authentication of console connections.
Console access is only provided by the NQRustBackup Director.

PAM Configuration
-----------------

By default, PAM configuration files resides in the directory :file:`/etc/pam.d/`.

Authentication using PAM is requested by a service name.
The NQRustBackup Director uses the service name **nqrustbackup**.
The corresponding configuration file is :file:`/etc/pam.d/nqrustbackup`.
If this file does not exist, PAM uses the fallback file :file:`/etc/pam.d/other`.

Often PAM is offered by system services, meaning the calling process has *root* priviliges.
In contrast, the NQRustBackup Director on Linux runs as user *nqrustbackup*,
therefore by default it might not offer all required functionality.

Known Limitations of PAM Modules
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

:pam_unix:
    When authenticating with pam_unix, it tries to read system files,
    including the file :file:`/etc/shadow`.
    By default, the user *nqrustbackup* do not have the permission to read this file.
    If this functionality is required, adapt the priviliges accordingly
    (e.g. add the user nqrustbackup to the group owning the file).

:pam_ldap:
    When using pam_ldap make sure
    your configuration does not require the rootbinddn and :file:`/etc/pam_ldap.secret` settings.
    Instead use the binddn/bindpw settings (if required).

Another limitation is, that some PAM modules do not ask for a login name.
They only ask for the password.
As result, the nqrustbackup_console command (without the -p parameter)
will only ask for a password, but not the login name.
As the user is unknown, the authentication fails.

One method to circumvent this
is to provide the PAM credentials to the nqrustbackup_console by an extra credentials file.
This credentials file is adressed by the nqrustbackup_console -p parameter.

Testing PAM Authentication
~~~~~~~~~~~~~~~~~~~~~~~~~~

If you have configured the PAM settings for NQRustBackup (:file:`/etc/pam.d/nqrustbackup`),
you can test it outside of NQRustBackup.

Make sure, the program **pamtester** (package: pamtester on Debian) is installed.

In this example, we will test, if the user USER_TO_TEST can be successfully authenticated by PAM.

::

   # switch to user nqrustbackup, to run with the same priviliges as nqrustbackup-dir
   su - nqrustbackup -s /bin/bash

   # use pamtester to try authentication by the PAM service nqrustbackup
   pamtester nqrustbackup USER_TO_TEST authenticate


Pamtester will ask for a password.
After providing this,
it will print if the user can be authenticated successfully (output: "pamtester: successfully authenticated") or not.

Also the account management phase can be tested:

::

   # switch to user nqrustbackup, to run with the same priviliges as nqrustbackup-dir
   su - nqrustbackup -s /bin/bash

   # use pamtester to test the PAM account management of the nqrustbackup service
   pamtester nqrustbackup USER_TO_TEST acct_mgmt


Testing PAM Authentication of the NQRustBackup Director
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

After PAM has been successfully tested using pamtester,
it can be tested using the nqrustbackup-dir.

Configure the NQRustBackup Director as described by https://docs.nqrustbackup.org/TasksAndConcepts/PAM.html#configuration.

Create a nqrustbackup_console configuration file, name it :file:`nqrustbackup_console-pam.conf`.

Test to connect via nqrustbackup_console to the nqrustbackup-dir::

   $ nqrustbackup_console -c nqrustbackup_console-pam.conf
   Connecting to Director localhost:9101
    Encryption: ECDHE-PSK-CHACHA20-POLY1305
   Login:USER_TO_TEST
   Passwort: ********
   1000 OK: nqrustbackup-dir Version: 19.1.2 (01 February 2019)
   You are logged in as: USER_TO_TEST

   Enter a period to cancel a command.
   *

After successfully testing with nqrustbackup_console, the NQRustBackup WebUI can be tested.

Reuse your existing PamConsole or create an additional one::

   Console {
     Name = "pam-webui"
     Password = "secret"
     UsePamAuthentication = yes
     TLS Enable = no
   }

As PHP does not yet support TLS-PSK, the setting ``TLS Enable = no`` is required.
Therefore it is advised to run the NQRustBackup Director and NQRustBackup WebUI on the same host.

Configure the ``pam_console_name`` and the ``pam_console_password`` in :file:`/etc/nqrustbackup-webui/directors.ini`
as defined in the Console Resource, see above.

You may want to add an additional NQRustBackup Director section like this or add the
parameters to an already existing one if heading for PAM usage only.

::

   [localhost-dir-pam]
   enabled              = "yes"
   diraddress           = "localhost"
   dirport              = 9101
   tls_verify_peer      = false
   server_can_do_tls    = false
   server_requires_tls  = false
   client_can_do_tls    = false
   client_requires_tls  = false
   pam_console_name     = "pam-webui"
   pam_console_password = "secret"

PAM users require a dedicated User Resource, see https://docs.nqrustbackup.org/Configuration/Director.html#user-resource .

A User Resource for a user named `alice` in the file :file:`/etc/nqrustbackup/nqrustbackup-dir.d/user/alice.conf` could
look like folllowing::

   User {
      Name = "alice"
      Profile = "webui-admin"
   }

Now you should be able to login using PAM user `alice` for example.


Auto Create NQRustBackup Users
~~~~~~~~~~~~~~~~~~~~~~~~

Until now, only PAM users that are already configured in the NQRustBackup Director can login.

The PAM script ``pam_exec_add_nqrustbackup_user.py`` can circumvent this.

It can be integrated into the NQRustBackup PAM configuration by ``pam_exec`` .

This version of the script requires NQRustBackup >= 19.2.12 or >= 20.0.6 or >= 21.1.0.

Installation
^^^^^^^^^^^^

* Verify that ``pam_exec`` is installed. On Debian it is part of the PAM base package **libpam-modules**.
* Install ``python-nqrustbackup``.
* Copy ``pam_exec_add_nqrustbackup_user.py`` to :file:`/usr/local/bin/`.

Create a NQRustBackup console for user pam-adduser (:file:`pam-adduser.conf`):

::

   Console {
     Name       = "pam-adduser"
     Password   = "secret"
     CommandACL = ".api", ".profiles", ".users", "configure", "version"
     TlsEnable  = no
   }


Add a pam_exec line to the PAM configuration file :file:`/etc/pam.d/nqrustbackup`.
This example uses pam_ldap to authenticate.

::

   auth     required            pam_ldap.so
   account  requisite           pam_ldap.so
   account  [default=ignore]    pam_exec.so /usr/bin/python3 /usr/local/bin/pam_exec_add_nqrustbackup_user.py --name pam-adduser --password secret --profile webui-admin


Make sure, an unsuccessful authentication ends before pam_exec.so.
In this example, this is done by the *requisite* keyword (when not successful, stop executing the PAM stack).

Using this, a user who successfully authenticates against LDAP, will be created as NQRustBackup user with ACLs as defined in profile *webui-admin*.
