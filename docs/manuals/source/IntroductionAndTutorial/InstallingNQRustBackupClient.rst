.. _section-InstallNQRustBackupClient:

Installing a NQRustBackup Client
==========================

When installing a NQRustBackup client,
you should choose the same release as on the NQRustBackup server.

The package **nqrustbackup-client** is a meta-package.
Installing it will also install
the **nqrustbackup-filedaemon**, **nqrustbackup-nqrustbackup_console** and
suggests the installation of the **nqrustbackup-traymonitor**.

If you prefer to install just the backup client,
it is sufficient to only install the package **nqrustbackup-filedaemon**.

After installing the client,
please read the chapter :ref:`section-AddAClient`
about how to configure the client.

Installing a NQRustBackup Client on Linux Distributions
-------------------------------------------------

The installation of a NQRustBackup client on a Linux (and FreeBSD) system
is identical as described for NQRustBackup server installations.

Just install the package **nqrustbackup-filedaemon** or
**nqrustbackup-client** (**nqrustbackup-filedaemon**, **nqrustbackup-nqrustbackup_console** and **nqrustbackup-traymonitor**)
instead of the meta-package **nqrustbackup**.

If there is no specific NQRustBackup repository for your Linux distribution,
consider using the :ref:`section-UniversalLinuxClient` instead.

* :ref:`section-InstallNQRustBackupPackagesRedhat`
* :ref:`section-InstallNQRustBackupPackagesSuse`
* :ref:`section-InstallNQRustBackupPackagesDebian`
* :ref:`section-UniversalLinuxClient`


.. _section-UniversalLinuxClient:

Installing the NQRustBackup Universal Linux Client
--------------------------------------------

The NQRustBackup project provides packages
for the current releases of all major Linux distributions.
In order to support even more platforms
NQRustBackup :sinceVersion:`21.0.0: Universal Linux Client (ULC)`
provides the so called **Universal Linux Client (ULC)**.

The Universal Linux Client is a |fd|,
built in a way to have minimal dependencies to other libraries.

.. note::

   The **Universal Linux Client** depends on the **OpenSSL** library
   of the host in order to utilize security updates for this library.

It incorporates all functionality for normal backup and restore operations,
however it has only limited plugin support.

Currently it is provided as a Debian (x86, arm) and RPM (x86, arm) package.

The ULC have extra repositories, their names starting with **ULC_**
(e.g. **ULC_deb_OpenSSL_3.0**)
at https://download.nqrustbackup.com/nqrustbackup/release/ and https://download.nqrustbackup.org/current/.
There will be different repositories depending on packaging standard
and remaining dependencies.
These repositories contain the **nqrustbackup-universal-client** package
and sometimes their corresponding debug package.
You can either add the repository to your system
(e.g. by the :file:`add_nqrustbackup_repositories*.sh` scripts provided in the repository,
see the description of the other platforms)
or only download and install the package file.

One of ULC's goals is to support new platforms
for which native packages are not yet available.
As soon as native packages are available,
their repository can be added
and on an update the ULC package
will be seamlessly replaced by the normal |fd| package.
No change to the NQRustBackup configuration is required.

.. warning::

   While ULC packages are designed to run on as many Linux platforms as possible,
   they should only be used
   if this platform is not directly supported by the NQRustBackup project.
   When available, native packages should be preferred.

Feature overview:

  * Single package installation
  * Repository based installation
  * Minimal dependencies to system libraries (except OpenSSL)
  * Uses host OpenSSL library
  * Replaceable by the normal |fd|. No configuration change required.


Installing a NQRustBackup Client on FreeBSD
-------------------------------------

Installing the NQRustBackup client is very similar to :ref:`section-InstallNQRustBackupPackagesFreebsd`.

Get the :file:`add_nqrustbackup_repositories.sh`
matching the requested NQRustBackup release
and the distribution of the target system
from https://download.nqrustbackup.org/ or https://download.nqrustbackup.com/
and execute it on the target system:

.. code-block:: shell-session
   :caption: Shell example script for NQRustBackup installation on FreeBSD

   root@host:~# sh ./add_nqrustbackup_repositories.sh
   root@host:~# pkg install --yes nqrustbackup.com-filedaemon

   ## enable services
   root@host:~# sysrc nqrustbackupfd_enable=YES

   ## start services
   root@host:~# service nqrustbackup-fd start


.. _section-Solaris:

Installing a NQRustBackup Client on Oracle Solaris
--------------------------------------------

.. index::
   single: Platform; Solaris

The |fd| is available as **IPS** (*Image Packaging System*) packages for **Oracle Solaris 11.4**.

First, download the Solaris package to the local disk and add the package as publisher
**nqrustbackup**:

.. code-block:: shell-session
   :caption: Add nqrustbackup publisher

   root@solaris114:~# pkg set-publisher -p nqrustbackup-fd-<version>.p5p  nqrustbackup
   pkg set-publisher:
     Added publisher(s): nqrustbackup


Then, install the filedaemon with **pkg install**:


.. code-block:: shell-session
   :caption: Install solaris package

   root@solaris114:~# pkg install nqrustbackup-fd
             Packages to install:  1
              Services to change:  1
         Create boot environment: No
   Create backup boot environment: No

   DOWNLOAD                                PKGS         FILES    XFER (MB)   SPEED
   Completed                                1/1         44/44      1.0/1.0  4.8M/s

   PHASE                                          ITEMS
   Installing new actions                         94/94
   Updating package state database                 Done
   Updating package cache                           0/0
   Updating image state                            Done
   Creating fast lookup database                working |


After installation, check the nqrustbackup-fd service status with **svcs nqrustbackup-fd**:

.. code-block:: shell-session
   :caption: Check solaris service

   root@solaris114:~# svcs nqrustbackup-fd
   STATE          STIME      FMRI
   online         16:16:14   svc:/nqrustbackup-fd:default


Finish the installation by adapting the configuration in :file:`/usr/local/etc/nqrustbackup` and restart the
service with **svcadm restart nqrustbackup-fd**:

.. code-block:: shell-session
   :caption: Restart solaris service

   root@solaris114:~# svcadm restart nqrustbackup-fd

The |fd| service on solaris is now ready for use.


.. _section-macosx:

Installing a NQRustBackup Client on macOS
------------------------------------

.. index::
   single: Platform; macOS

NQRustBackup for macOS is available either

-  as pkg file from https://download.nqrustbackup.org/ or https://download.nqrustbackup.com/.

-  via the `Homebrew project <https://brew.sh/>`_ (https://formulae.brew.sh/formula/nqrustbackup-client).

However, you have to choose upfront, which client you want to use. Otherwise conflicts do occur.

Both packages contain the |fd| and :command:`nqrustbackup_console`.

Installing the NQRustBackup Client as PKG
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. index::
   single: Installation; macOS

The NQRustBackup installer package for macOS contains the |fd| for macOS 10.5 or later.

On your local Mac, you must be an admin user. The main user is an admin user.

Download the :file:`nqrustbackup-*.pkg` installer package from https://download.nqrustbackup.org/ or https://download.nqrustbackup.com/.

Find the .pkg you just downloaded. Install the .pkg by holding the CTRL key, left-clicking the installer and choosing "open".

Follow the directions given to you and finish the installation.

Alternatively you can install the package via command line:

.. code-block:: shell-session

   sudo installer -pkg nqrustbackup-*.pkg -target /

Configuration
~~~~~~~~~~~~~

To make use of your |fd| on your system, it is required to configure the |dir| and the local |fd|.

Configure the server-side by follow the instructions at :ref:`section-AddAClient`.

After configuring the server-side you can either transfer the necessary configuration file using following command or configure the client locally.

The configuration path differs from a Linux installation.
On Linux the configuration files are located under :file:`/etc/nqrustbackup/`.
On macOS pkg installations, the configuration path is  :file:`/usr/local/nqrustbackup/etc/nqrustbackup/`.
On macOS Homebrew installations, the configuration path is :file:`/usr/local/etc/nqrustbackup/`.


Option 1: Copy the director resource from the NQRustBackup Director to the Client
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Assuming your client has the DNS entry :strong:`client2.example.com` and has been added to |dir| as :config:option:`dir/client = client2-fd`\ :

.. code-block:: shell-session
   :caption: copy director resource to a macOS pkg installation client

   scp /etc/nqrustbackup/nqrustbackup-dir-export/client/client2-fd/nqrustbackup-fd.d/director/nqrustbackup-dir.conf root@client2.example.com:/usr/local/nqrustbackup/etc/nqrustbackup/nqrustbackup-fd.d/director/

Option 2: Edit the director resource on the Client
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Alternatively, you can edit the file :file:`/usr/local/nqrustbackup/etc/nqrustbackup/nqrustbackup-fd.d/director/nqrustbackup-dir.conf`.

This can be done by right-clicking the finder icon in your task bar, select "Go to folder ..." and paste :file:`/usr/local/nqrustbackup/etc/nqrustbackup/nqrustbackup-fd.d/director/`.

Select the :file:`nqrustbackup-dir.conf` file and open it.

Alternatively you can also call following command on the command console:

.. code-block:: shell-session

   open -t /usr/local/nqrustbackup/etc/nqrustbackup/nqrustbackup-fd.d/director/nqrustbackup-dir.conf

The file should look similar to this:

.. code-block:: nqrustbackupconfig
   :caption: nqrustbackup-fd.d/director/nqrustbackup-dir.conf

   Director {
     Name = nqrustbackup-dir
     Password = "SOME_RANDOM_PASSWORD"
     Description = "Allow the configured Director to access this file daemon."
   }

Set this client-side password to the same value as given on the server-side.



.. warning::

   The configuration file contains passwords and therefore must not be accessible for any users except admin users.

Restart nqrustbackup-fd after changing the configuration
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The nqrustbackup-fd must be restarted to reread its configuration:

.. code-block:: shell-session
   :caption: Restart the |fd|

   sudo launchctl stop  com.nqrustbackup.nqrustbackup-fd
   sudo launchctl start com.nqrustbackup.nqrustbackup-fd

Verify that the NQRustBackup File Daemon is working
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Open the :command:`nqrustbackup_console` on your |dir| and check the status of the client with

.. code-block:: nqrustbackupconfig

   *<input>status client=client2-fd</input>

In case, the client does not react, following command are useful the check the status:

.. code-block:: shell-session
   :caption: Verify the status of |fd|

   # check if nqrustbackup-fd is started by system:
   sudo launchctl list com.nqrustbackup.nqrustbackup-fd

   # get process id (PID) of nqrustbackup-fd
   pgrep nqrustbackup-fd

   # show files opened by nqrustbackup-fd
   sudo lsof -p `pgrep nqrustbackup-fd`

   # check what process is listening on the |fd| port
   sudo lsof -n -iTCP:9102 | grep LISTEN

You can also manually start nqrustbackup-fd in debug mode by:

.. code-block:: shell-session
   :caption: Start |fd| in debug mode

   cd /usr/local/nqrustbackup
   sudo /usr/local/nqrustbackup/sbin/nqrustbackup-fd -f -d 100


Installing a NQRustBackup Client on Windows
-------------------------------------

See :ref:`Windows:Installation`.
