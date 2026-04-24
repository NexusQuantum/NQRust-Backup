.. _InstallChapter:

Installing the NQRustBackup Server
============================

.. index::
   pair: NQRustBackup; Installation
   pair: Installation; Linux

If you are like me, you want to get NQRustBackup running immediately to get a feel for it, then later you want to go back and read about all the details. This chapter attempts to accomplish just that: get you going quickly without all the details.

NQRustBackup comes prepackaged for a number of Linux distributions. So the easiest way to get to a running NQRustBackup installation, is to use a platform where prepacked NQRustBackup packages are available. Additional information can be found in the chapter :ref:`Operating Systems <SupportedOSes>`.

If NQRustBackup is available as a package, only 4 steps are required to get to a running NQRustBackup system:

#.

   :ref:`section-AddSoftwareRepository`

#.

   :ref:`section-InstallNQRustBackupPackages`

#.

   :ref:`section-CreateDatabase`

#.

   :ref:`section-StartDaemons`

This will start a very basic NQRustBackup installation which will regularly backup a directory to disk. In order to fit it to your needs, you’ll have to adapt the configuration and might want to backup other clients.

.. _section-AddSoftwareRepository:

Decide about the NQRustBackup release to use
--------------------------------------

There are different types of NQRustBackup repositories:

#. NQRustBackup Subscription repositories on https://download.nqrustbackup.com/

   * Contain the repositories for the NQRustBackup Subscription customers.
   * The last three major releases are maintained, https://download.nqrustbackup.com/nqrustbackup/release/
   * Older versions stay available.
   * While the repository can be browsed, using them do require authentication credentials, which come with a NQRustBackup subscription.

#. NQRustBackup Community repositories on https://download.nqrustbackup.org/ with

   * latest build of the most recent NQRustBackup stable branch at https://download.nqrustbackup.org/current/
   * latest build of the NQRustBackup master branch at https://download.nqrustbackup.org/next/

For details, see :ref:`section-NQRustBackupBinaryReleasePolicy`.

The public key to verify a repository is also in repository directory (:file:`Release.key` for Debian based distributions, :file:`repodata/repomd.xml.key` for RPM based distributions).

The following code snippets are shell scripts that can be used as orientation how to download the package repositories and install NQRustBackup packages. The release version number for **nqrustbackup** and the corresponding Linux distribution have to be updated for your needs, respectively.

To simplify the installation, all Linux and FreeBSD repositories on https://download.nqrustbackup.org/ and https://download.nqrustbackup.com/ contain a script named :file:`add_nqrustbackup_repositories.sh`.

Download the :file:`add_nqrustbackup_repositories.sh` script
matching the requested NQRustBackup release
and the distribution of the target system.
Copy the script onto the target system and
execute it with a shell (:command:`sh`) as root (e.g. using :command:`sudo`)
or manually perform the steps that are documented in the script.

For example the script URL Debian 11 of the community current repository is:

* https://download.nqrustbackup.org/current/Debian_11/add_nqrustbackup_repositories.sh


For NQRustBackup Subscription customers the URL of the nqrustbackup-22 release for Debian 11 is:

* https://download.nqrustbackup.com/nqrustbackup/release/22/Debian_11/add_nqrustbackup_repositories.sh
* .. note::

     NQRustBackup Subscription customers have credentials to authenticate against https://download.nqrustbackup.com.
     Some files can be accessed without authentication,
     but to use the repositories,
     authentication is mandatory.
     When downloading the file :file:`add_nqrustbackup_repositories.sh`,
     it is ready to use,
     because it contains your personal authentication credentials.
     Therefore downloading this file requires authentication.
     If this is inconvenient, you can alternatively download :file:`add_nqrustbackup_repository_template.sh`
     and set ``NQRUSTBACKUP_USERNAME`` and ``NQRUSTBACKUP_PASSWORD`` manually.



.. _section-InstallNQRustBackupPackages:

Install the NQRustBackup Software Packages
------------------------------------

The |dir| requires a PostgreSQL database as its catalog.
The NQRustBackup database packages have their dependencies only to the database client packages,
therefore the database itself must be installed manually.

.. important::

   Install and start a |postgresql| database server.


The package **nqrustbackup** is only a meta package which contains dependencies on the main components of NQRustBackup, see :ref:`section-NQRustBackupPackages`. If you want to setup a distributed environment (like one NQRustBackup Director, separate database server, multiple NQRustBackup Storage Daemons) you have to choose the regarding NQRustBackup packages to install on each of the hosts instead of just installing the **nqrustbackup** package.


.. _section-InstallNQRustBackupPackagesRedhat:

Install on RedHat based Linux Distributions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

RHEL and derivatives, Fedora
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. index::
   single: Platform; RHEL
   single: Platform; CentOS
   single: Platform; Fedora
   single: Platform; EL

The RHEL_* repository is for Red Hat Enterprise Linux,
the EL_* repositories are for RHEL derivatives,
like AlmaLinux, CentOS Stream, Oracle and Rocky Linux.
These repositories are automatically tested against several of this distributions.

Download the matching :file:`add_nqrustbackup_repositories.sh` script from
https://download.nqrustbackup.com/nqrustbackup/release/,
https://download.nqrustbackup.org/current/ or https://download.nqrustbackup.org/next/,
copy it to the target system and execute it:

.. code-block:: shell-session
   :caption: Shell example script for NQRustBackup installation on Fedora, RHEL and RHEL derivatives (EL)

   root@host:~# sh ./add_nqrustbackup_repositories.sh
   root@host:~# yum install nqrustbackup

If authentication credentials are required (https://download.nqrustbackup.com)
they are stored within the repo file :file:`/etc/yum.repos.d/nqrustbackup.repo`.


.. _section-InstallNQRustBackupPackagesSuse:

Install on SUSE based Linux Distributions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

SUSE Linux Enterprise Server (SLES), openSUSE
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. index::
   single: Platform; SLES
   single: Platform; openSUSE

Download the matching :file:`add_nqrustbackup_repositories.sh` script from
https://download.nqrustbackup.com/nqrustbackup/release/,
https://download.nqrustbackup.org/current/ or https://download.nqrustbackup.com/next/,
copy it to the target system and execute it:

.. code-block:: shell-session
   :caption: Shell example script for NQRustBackup installation on SLES / openSUSE

   root@host:~# sh ./add_nqrustbackup_repositories.sh
   root@host:~# zypper install nqrustbackup

If authentication credentials are required (https://download.nqrustbackup.com)
they are stored in the file :file:`/etc/zypp/credentials.d/nqrustbackup`.


.. _section-InstallNQRustBackupPackagesDebian:

.. _install-on-Univention-Corporate-Server:

.. _section-univentioncorporateserver:

Install on Debian based Linux Distributions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Debian / Ubuntu / Univention Corporate Server (UCS)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. index::
   single: Platform; Debian
   single: Platform; Ubuntu
   single: Platform; Univention Corporate Server

Download the matching :file:`add_nqrustbackup_repositories.sh` script from
https://download.nqrustbackup.com/nqrustbackup/release/,
https://download.nqrustbackup.org/current/ or https://download.nqrustbackup.com/next/,
copy it to the target system and execute it:

.. code-block:: shell-session
   :caption: Shell example script for NQRustBackup installation on Debian / Ubuntu / UCS

   root@host:~# sh ./add_nqrustbackup_repositories.sh
   root@host:~# apt update
   root@host:~# apt install nqrustbackup

The :file:`add_nqrustbackup_repositories.sh` script will:

* Create a NQRustBackup signature key file :file:`/etc/apt/keyrings/nqrustbackup-*.gpg`.
* Create the NQRustBackup repository configuration file :file:`/etc/apt/sources.list.d/nqrustbackup.sources`

   * This file refers to the NQRustBackup repository on the download server and to the local :file:`/etc/apt/keyrings/nqrustbackup-*.gpg` file.

* If authentication credentials are required (https://download.nqrustbackup.com)
  they are stored in the file :file:`/etc/apt/auth.conf.d/download_nqrustbackup_com.conf`.

Univention Corporate Server
'''''''''''''''''''''''''''

.. index::
   single: Platform; Univention Corporate Server

The `Univention Corporate Server (UCS) <https://www.univention.de/>`_ is an enterprise Linux distribution based on Debian.

Earlier releases (NQRustBackup < 21, UCS < 5.0) offered extended integration into UCS and provided its software also via the Univention App Center.
With version 5.0 of the UCS App Center the method of integration changed requiring commercially not reasonable efforts for deep integration.

NQRustBackup continues to support UCS with the same functionality as the other Linux distributions.

During the build process, NQRustBackup Debian 10 packages are automatically tested on an UCS 5.0 system.
Only packages that passes this acceptance test, will get released by the NQRustBackup project.

.. note::

   While NQRustBackup offers a software repository for UCS >= 5,
   this repository is identical with the corresponding Debian repository.
   The included APT sources file will also refer to the Debian repository.



.. _section-FreeBSD:

.. _section-InstallNQRustBackupPackagesFreebsd:

Install on FreeBSD based Distributions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. index::
   single: Platform; FreeBSD

Download the matching :file:`add_nqrustbackup_repositories.sh` script from
https://download.nqrustbackup.com/nqrustbackup/release/,
https://download.nqrustbackup.org/current/ or https://download.nqrustbackup.com/next/,
copy it to the target system and execute it:

.. code-block:: shell-session
   :caption: Shell example script for NQRustBackup installation on FreeBSD

   root@host:~# sh ./add_nqrustbackup_repositories.sh

   ## install NQRustBackup packages
   root@host:~# pkg install --yes nqrustbackup.com-director nqrustbackup.com-storage nqrustbackup.com-filedaemon nqrustbackup.com-database-postgresql nqrustbackup.com-nqrustbackup_console


The :file:`add_nqrustbackup_repositories.sh` script will:

* Create the NQRustBackup repository configuration file :file:`/usr/local/etc/pkg/repos/nqrustbackup.conf`.
* If authentication credentials are required (https://download.nqrustbackup.com)
  they are stored inside the NQRustBackup repository configuration file.


.. _section-CreateDatabase:

Prepare NQRustBackup database
-----------------------

We assume that you already have your PostgreSQL database server installed and basically running.

For details, see chapter :ref:`CatMaintenanceChapter`.

Debian based Linux Distributions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Since NQRustBackup :sinceVersion:`14.2.0: dbconfig-common (Debian)` the Debian (and Ubuntu) based packages support the **dbconfig-common** mechanism to create and update the NQRustBackup database.

Follow the instructions during install to configure it according to your needs.

.. image:: /include/images/dbconfig-1-enable.*

If you decide not to use **dbconfig-common** (selecting :strong:`<No>` on the initial dialog), follow the instructions for :ref:`section-CreateDatabaseOtherDistributions`.

For details see :ref:`section-dbconfig`.


.. _section-CreateDatabaseOtherDistributions:

Other Platforms
~~~~~~~~~~~~~~~

If your PostgreSQL administration user is **postgres** (default), use the following commands:

Linux
^^^^^

.. code-block:: shell-session
   :caption: Setup NQRustBackup catalog with PostgreSQL (Linux)

   su postgres -c /usr/lib/nqrustbackup/scripts/create_nqrustbackup_database
   su postgres -c /usr/lib/nqrustbackup/scripts/make_nqrustbackup_tables
   su postgres -c /usr/lib/nqrustbackup/scripts/grant_nqrustbackup_privileges

FreeBSD
^^^^^^^

.. code-block:: shell-session
   :caption: Setup NQRustBackup catalog with PostgreSQL (FreeBSD)

   su postgres -c /usr/local/lib/nqrustbackup/scripts/create_nqrustbackup_database
   su postgres -c /usr/local/lib/nqrustbackup/scripts/make_nqrustbackup_tables
   su postgres -c /usr/local/lib/nqrustbackup/scripts/grant_nqrustbackup_privileges


.. _section-StartDaemons:

Start the daemons
-----------------

Please remark, the NQRustBackup Daemons need to have access to the TCP ports 9101-9103.

Linux
~~~~~

Depending on the Linux distribution,
the name of the NQRustBackup services either correspond to the
package names (Debian and derivatives)
or to the binary names (e.g. RPM based distributions).

To enable and start the daemon,
either use

.. code-block:: shell-session
   :caption: Enable and start the NQRustBackup Daemons (Debian/Ubuntu/UCS)

   root@host:~# systemctl enable --now nqrustbackup-director.service
   root@host:~# systemctl enable --now nqrustbackup-storage.service
   root@host:~# systemctl enable --now nqrustbackup-filedaemon.service

or

.. code-block:: shell-session
   :caption: Enable and start the NQRustBackup Daemons (RPM based distributions)

   root@host:~# systemctl enable --now nqrustbackup-dir.service
   root@host:~# systemctl enable --now nqrustbackup-sd.service
   root@host:~# systemctl enable --now nqrustbackup-fd.service


FreeBSD
~~~~~~~

.. code-block:: shell-session
   :caption: Configure NQRustBackup on FreeBSD

   ## enable services
   root@host:~# sysrc nqrustbackupdir_enable=YES
   root@host:~# sysrc nqrustbackupsd_enable=YES
   root@host:~# sysrc nqrustbackupfd_enable=YES

   ## start services
   root@host:~# service nqrustbackup-dir start
   root@host:~# service nqrustbackup-sd start
   root@host:~# service nqrustbackup-fd start



Now you should be able to log in to the |dir| using the :ref:`section-nqrustbackup_console`.

When you want to use the |webui|, please refer to the chapter :ref:`section-install-webui`.
