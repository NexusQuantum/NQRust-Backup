
.. _UpdateChapter:

Updating and upgrading NQRustBackup
=============================

.. index::
   pair: NQRustBackup; Update

NQRustBackup consists of 4 main components:

  * |dir|
  * |sd|
  * |fd|
  * |webui|


.. _section-UpdateUpgradePreamble:

Before updating or upgrading NQRustBackup software
--------------------------------------------

.. index::
   single: Update Preamble

We consider both operations usually safe and they can be applied without restriction to all installations. The NQRustBackup project avoids breaking changes as far as possible, so usually existing configurations will work with newer installations without changes.
Especially when updating the |dir| (together with the corresponding |sd|) it is recommended to do a number of security steps before going ahead:

- Read the :ref:`NQRustBackup current release notes <nqrustbackup-current-releasenotes>` and watch out for changes that might impact your installation. Take special care for the :strong:`Breaking Changes` paragraph, as it contains the information about changes that require your special attention when upgrading your installation as they might require adaptions of the configuration.
- Update your operating system to the latest security and patch level of the publisher.
- Empty the running jobs queue.
- Run :strong:`BackupCatalog` or equivalent as last job (keep the most up to date database dump & configuration state).
- Stop all NQRustBackup daemons.
- If not done before

   - Save a copy of the actual configuration used. (Usually :file:`/etc/nqrustbackup/` and :file:`/etc/nqrustbackup-webui/`.
   - Dump your database content to an easy to restore format.

- Cleanup the working dir :file:`/var/lib/nqrustbackup`

   - Remove any old crash traces :file:`*.bactrace` and :file:`*.traceback` files.
   - Remove any left debugging :file:`*.trace` files.
   - Remove any leftover :file:`*.mail` that have not been sent.
   - Remove any :file:`*core*` that are left behind.

- Rotate log files if you don't use :command:`logrotate`

   - :file:`/var/log/nqrustbackup/nqrustbackup.log`
   - :file:`/var/log/nqrustbackup/nqrustbackup-audit.log`

Now you can safely apply the update or upgrade.

.. warning::

   Please remind, that |dir| and |sd| must always have the same version.
   The version of the |fd| may differ.


.. note::

   When you change the repository and refresh it, it is likely to have a new signing key.
   You will have to accept it.


.. code-block:: shell-session
   :caption: new gpg key detection on EL 8 (dnf/yum)

      Backup Archiving Recovery Open Sourced (EL_8)                    1.6 MB/s | 1.6 kB     00:00
      Importing GPG key 0xC9FED482:
      Userid     : "NQRustBackup 21 Signing Key <signing@nqrustbackup.com>"
      Fingerprint: 91DA 1DC3 564A E20A 76C4 CA88 E019 57D6 C9FE D482
      From       : /etc/pki/rpm-gpg/RPM-GPG-KEY-nqrustbackup-release-21
      Is this ok [y/N]: y


.. code-block:: shell-session
   :caption: new gpg key detection on SUSE (zypper)

      Forcing raw metadata refresh
      New repository or package signing key received:
      Repository:       nqrustbackup
      Key Fingerprint:  91DA 1DC3 564A E20A 76C4 CA88 E019 57D6 C9FE D482
      Key Name:         NQRustBackup 21 Signing Key <signing@nqrustbackup.com>
      Key Algorithm:    RSA 4096
      Key Created:      Mon Dec 20 10:04:50 2021
      Key Expires:      (does not expire)
      Rpm Name:         gpg-pubkey-c9fed482-61c05542

         Note: Signing data enables the recipient to verify that no modifications occurred after the data
         were signed. Accepting data with no, wrong or unknown signature can lead to a corrupted system
         and in extreme cases even to a system compromise.

         Note: A GPG pubkey is clearly identified by its fingerprint. Do not rely on the key's name. If
         you are not sure whether the presented key is authentic, ask the repository provider or check
         their web site. Many providers maintain a web page showing the fingerprints of the GPG keys they
         are using.

      Do you want to reject the key, trust temporarily, or trust always? [r/t/a/?] (r): a



.. _section-UpdateFromCommunityToSubscription:

Updating from community to subscription binaries
------------------------------------------------

.. index::
    single: update from community to subscription
    pair: NQRustBackup; community repository
    pair: NQRustBackup; subscription repository

To update the installed community packages (https://download.nqrustbackup.org) to NQRustBackup Subscription packages,
you will have to point to the subscription repositories located at https://download.nqrustbackup.com/.

Once you received your `download.nqrustbackup.com` portal/repository credentials, you can refer to the following section :ref:`section-AddSoftwareRepository` for complete instructions how-to use the :file:`add_nqrustbackup_repositories.sh` helper.

Choose the same operating system and NQRustBackup major version you are already using.

Read the :ref:`NQRustBackup current release notes <nqrustbackup-current-releasenotes>` to check all fixes that have been made.

Proceed to the next section, to install last minor bugfix release on your systems.


.. _section-UpdateMinorBugfix:

Updating NQRustBackup to the latest minor or bugfix release
------------------------------------------------------

.. index::
   single: Update latest minor bugfix
   pair: update; minor;
   pair: update; bugfix

In most cases, a NQRustBackup update is simply done by a package update of the distribution.

.. note::

   Please before processing, apply steps in :ref:`section-UpdateUpgradePreamble`


In this section, we explain how to update your NQRustBackup major version to the latest minor or bugfix release.

For upgrading to a new major version see :ref:`Update NQRustBackup to a new major release <section-UpgradeMajor>`.

.. note::

   You can install directly the latest Major,Minor,Bugfix release available.
   So updating from 21.0.0 directly to 21.1.5 is not a problem.



Example how to update from 21.0.0 to 21.1.5.

.. index::
   single: update minor bugfix; RHEL
   single: update minor bugfix; CentOS
   single: update minor bugfix; Fedora
   single: update minor bugfix; EL

.. code-block:: shell-session
   :caption: Shell example command to update NQRustBackup on on EL 8

   root@host:~# dnf upgrade --repo=nqrustbackup --refresh
      Backup Archiving Recovery Open Sourced (EL_8)            .5 kB/s | 833  B     00:00
      Dependencies resolved.
      ===================================================================================
      Package                         Architecture     Version         Repository   Size
      ===================================================================================
      Upgrading:
      nqrustbackup                          x86_64           21.1.5-3.el8    nqrustbackup      7.4 k
      nqrustbackup-nqrustbackup_console                 x86_64           21.1.5-3.el8    nqrustbackup       37 k
      nqrustbackup-client                   x86_64           21.1.5-3.el8    nqrustbackup      7.5 k
      nqrustbackup-common                   x86_64           21.1.5-3.el8    nqrustbackup      764 k
      nqrustbackup-database-common          x86_64           21.1.5-3.el8    nqrustbackup       87 k
      nqrustbackup-database-postgresql      x86_64           21.1.5-3.el8    nqrustbackup       42 k
      nqrustbackup-database-tools           x86_64           21.1.5-3.el8    nqrustbackup      107 k
      nqrustbackup-director                 x86_64           21.1.5-3.el8    nqrustbackup      425 k
      nqrustbackup-filedaemon               x86_64           21.1.5-3.el8    nqrustbackup      120 k
      nqrustbackup-storage                  x86_64           21.1.5-3.el8    nqrustbackup       97 k
      nqrustbackup-tools                    x86_64           21.1.5-3.el8    nqrustbackup       52 k

      Transaction Summary
      ===================================================================================
      Upgrade  11 Packages

      Total download size: 1.7 M
      Is this ok [y/N]: y


.. index::
   single: update minor bugfix; SLE
   single: update minor bugfix; openSUSE

.. code-block:: shell-session
   :caption: Shell example command to update NQRustBackup on SLES / openSUSE

   root@host:~# zypper refresh --force nqrustbackup
   root@host:~# zypper -v update --repo=nqrustbackup
      Verbosity: 2
      Initializing Target
      Checking whether to refresh metadata for nqrustbackup
      Retrieving: repomd.xml ..........................................[done (3.0 KiB/s)]
      Retrieving: media ......................................................[not found]
      Retrieving: repomd.xml.asc ..................................................[done]
      Retrieving: repomd.xml.key ..................................................[done]
      Retrieving: repomd.xml ......................................................[done]
      Repository:       nqrustbackup
      Key Fingerprint:  91DA 1DC3 564A E20A 76C4 CA88 E019 57D6 C9FE D482
      Key Name:         NQRustBackup 21 Signing Key <signing@nqrustbackup.com>
      Key Algorithm:    RSA 4096
      Key Created:      Mon Dec 20 10:04:50 2021
      Key Expires:      (does not expire)
      Rpm Name:         gpg-pubkey-c9fed482-61c05542
      Retrieving: 7c2078b9b802f0f5c4edb818e870be0084ae132b4a5f21111617582fd927a65f-primary.xml.gz ...[done]
      Retrieving repository 'nqrustbackup' metadata .....................................[done]
      Building repository 'nqrustbackup' cache ..........................................[done]
      Loading repository data...
      Reading installed packages...
      Force resolution: No

      The following 10 packages are going to be upgraded:
      nqrustbackup                      21.0.0-4 -> 21.1.5-3
      nqrustbackup-nqrustbackup_console             21.0.0-4 -> 21.1.5-3
      nqrustbackup-client               21.0.0-4 -> 21.1.5-3
      nqrustbackup-common               21.0.0-4 -> 21.1.5-3
      nqrustbackup-database-common      21.0.0-4 -> 21.1.5-3
      nqrustbackup-database-postgresql  21.0.0-4 -> 21.1.5-3
      nqrustbackup-database-tools       21.0.0-4 -> 21.1.5-3
      nqrustbackup-director             21.0.0-4 -> 21.1.5-3
      nqrustbackup-filedaemon           21.0.0-4 -> 21.1.5-3
      nqrustbackup-storage              21.0.0-4 -> 21.1.5-3

      10 packages to upgrade.
      Overall download size: 1.5 MiB.
      Already cached: 0 B.
      After the operation, additional 59.6 KiB will be used.
      Continue? [y/n/v/...? shows all options] (y): y

.. index::
   single: update minor bugfix; Debian
   single: update minor bugfix; Ubuntu

.. code-block:: shell-session
   :caption: Shell example command to update NQRustBackup on Debian

   root@host:~# apt update
      Hit:1 http://deb.debian.org/debian bullseye InRelease
      Hit:2 http://deb.debian.org/debian-security bullseye-security InRelease
      Hit:3 http://deb.debian.org/debian bullseye-updates InRelease
      Get:4 https://download.nqrustbackup.com/nqrustbackup/release/21/Debian_11  InRelease [1861 B]
      Get:5 https://download.nqrustbackup.com/nqrustbackup/release/21/Debian_11  Sources [5660 B]
      Get:6 https://download.nqrustbackup.com/nqrustbackup/release/21/Debian_11  Packages [36.0 kB]
      Fetched 43.5 kB in 1s (42.3 kB/s)
      Reading package lists... Done
      Building dependency tree... Done
      Reading state information... Done
      15 packages can be upgraded. Run 'apt list --upgradable' to see them.
   root@host:~# apt upgrade
      Reading package lists... Done
      Building dependency tree... Done
      Reading state information... Done
      Calculating upgrade... Done
      The following packages will be upgraded:
      nqrustbackup nqrustbackup-nqrustbackup_console nqrustbackup-client nqrustbackup-common nqrustbackup-database-common
      nqrustbackup-database-postgresql nqrustbackup-database-tools nqrustbackup-director nqrustbackup-filedaemon
      nqrustbackup-storage nqrustbackup-tools libgssapi-krb5-2 libk5crypto3 libkrb5-3
      libkrb5support0
      15 upgraded, 0 newly installed, 0 to remove and 0 not upgraded.
      Need to get 2557 kB of archives.
      After this operation, 114 kB of additional disk space will be used.
      Do you want to continue? [Y/n] Y

.. index::
   single: update minor bugfix; FreeBSD

.. code-block:: shell-session
   :caption: Shell example command to update NQRustBackup on FreeBSD

   root@host:~# pkg update --repository NQRustBackup
   root@host:~# pkg upgrade --repository NQRustBackup
      Updating NQRustBackup repository catalogue...
      NQRustBackup repository is up to date.
      All repositories are up to date.
      Checking for upgrades (8 candidates): 100%
      Processing candidates (8 candidates): 100%
      The following 8 package(s) will be affected (of 0 checked):
      Installed packages to be UPGRADED:
            nqrustbackup.com-nqrustbackup_console: 21.0.0 -> 21.1.5 [NQRustBackup]
            nqrustbackup.com-common: 21.0.0 -> 21.1.5 [NQRustBackup]
            nqrustbackup.com-database-common: 21.0.0 -> 21.1.5 [NQRustBackup]
            nqrustbackup.com-database-postgresql: 21.0.0 -> 21.1.5 [NQRustBackup]
            nqrustbackup.com-database-tools: 21.0.0 -> 21.1.5 [NQRustBackup]
            nqrustbackup.com-director: 21.0.0 -> 21.1.5 [NQRustBackup]
            nqrustbackup.com-filedaemon: 21.0.0 -> 21.1.5 [NQRustBackup]
            nqrustbackup.com-storage: 21.0.0 -> 21.1.5 [NQRustBackup]
      Number of packages to be upgraded: 8
      1 MiB to be downloaded.
      Proceed with this action? [y/N]: y


.. _section-UpdatePostChecks:

Post update checks
~~~~~~~~~~~~~~~~~~

.. index::
   single: Update post checks

After the update, it is recommended to check if any new warnings are raised when starting the daemon, mostly deprecated configuration directives.
NQRustBackup will mark configuration directives at least for one major release as deprecated, before removing them.

To do so you can use the `-t` flag:

.. code-block:: shell-session
   :caption: Shell example to check the NQRustBackup configuration

   root@host:~# nqrustbackup-fd -t
   root@host:~# su - nqrustbackup -s /bin/sh -c "nqrustbackup-sd -t"
   root@host:~# su - nqrustbackup -s /bin/sh -c "nqrustbackup-dir -t"
   There are configuration warnings:
    * using deprecated keyword CollectStatistics on line 8 of file /etc/nqrustbackup/nqrustbackup-dir.d/storage/File.conf

The same warnings are also shown on a regular start of the daemons.


Depending of the operating system and its configuration, you will have to restart the daemons.
Use your operating system command to do so.


.. code-block:: shell-session
   :caption: Shell command to restart all nqrustbackup daemon with systemd on Linux

   root@host:~# systemctl restart nqrustbackup-director nqrustbackup-storage nqrustbackup-filedaemon
   root@host:~# systemctl status nqrustbackup-director nqrustbackup-storage nqrustbackup-filedaemon


.. code-block:: shell-session
   :caption: Shell command to restart all nqrustbackup daemon with service on FreeBSD

   root@host:~# service nqrustbackup-dir restart
   root@host:~# service nqrustbackup-fd restart
   root@host:~# service nqrustbackup-sd restart



.. _section-UpgradeMajor:

Upgrading NQRustBackup to a new major release
---------------------------------------

.. index::
   single: Upgrade latest major version
   pair: NQRustBackup; Upgrade
   pair: Upgrade; Major

In most cases, a NQRustBackup major upgrade can be achieved by:

- Add new major repository (subscription only)
- Package upgrade of the distribution.
- Database schema upgrade with helper scripts (if schema was changed).
- Configuration review to cleanup deprecated or removed parameters.
- Review of home made scripts and manage their adaptation in case of changes.

It is generally sufficient to upgrade directly to the latest release, without having to install any intermediate releases.
However, it is required to read the release notes of all intermediate releases.

One exception is when using a |mysql| NQRustBackup catalog,
which have been removed with NQRustBackup :sinceVersion:`21.0.0: MySQL backend removed`.
Therefore you first have to upgrade to NQRustBackup 20 and migrate the |mysql| into a |postgresql| NQRustBackup Catalog, see :ref:`section-MigrationMysqlToPostgresql`.


Prepare the upgrade
~~~~~~~~~~~~~~~~~~~

If you not have already done those steps, please refer to instructions in :ref:`section-UpdateUpgradePreamble`.

.. warning::

   If you use any third party plugins, you should check and test their functionalities with the new major version beforehand.

.. _sectionUgradeMajorRepository:

Upgrade the NQRustBackup download repositories
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

This does only apply for subscription repositories (https://download.nqrustbackup.com/nqrustbackup/release/).
The community repository (https://download.nqrustbackup.org/current/) will always contain the latest build of the most recent stable branch.

- First remove the existing NQRustBackup repository definitions, by either removing the definition file(s) or by using your package manager.
- Point your browser to the new NQRustBackup major version for your operating system on the download server.
- Open or save the helper script :file:`add_nqrustbackup_repositories.sh`.

  - You can refer to the following section :ref:`section-AddSoftwareRepository` for complete instructions how to use the :file:`add_nqrustbackup_repositories.sh` helper.

- Transfer the file to your NQRustBackup server, and execute it as **root**.
  This will create (or depending on your OS update) the NQRustBackup repository information.

.. code-block:: shell-session
   :caption: Shell command to upgrade the NQRustBackup repository

   root@host:~# sh add_nqrustbackup_repositories.sh

You should be able now to proceed the appropriate commands to refresh the packages list and upgrade the package to the newer version.

.. note::

   You can refer to section :ref:`section-UpdateMinorBugfix` for commands example.


.. _sectionUpdateConfigurationFiles:

Updating the configuration files
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

When updating NQRustBackup through the distribution packaging mechanism,
the existing configuration files are kept as they are.

If your configuration didn't show any deprecation warnings prior the upgrade, no configuration change will be required on upgrades.
Check the :ref:`Release Notes <releasenotes>` for breaking changes or other special cases.


.. _sectionUpdateDatabaseScheme:

Updating the database scheme
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Sometimes improvements in NQRustBackup make it necessary to update the database scheme.


.. warning::

   If the NQRustBackup catalog database does not have the current schema, the NQRustBackup Director refuses to start.


.. code-block:: shell-session
   :caption: Shell example of nqrustbackup-dir failing to start due to lack of database schema update

   root@host:~# su - nqrustbackup -s /bin/sh -c "nqrustbackup-dir -t"
   nqrustbackup-dir: dird/check_catalog.cc:64-0 Could not open Catalog "MyCatalog", database "nqrustbackup".
   nqrustbackup-dir: dird/check_catalog.cc:71-0 Version error for database "nqrustbackup". Wanted 2210, got 2192
   nqrustbackup-dir ERROR TERMINATION
   Please correct the configuration in /etc/nqrustbackup/nqrustbackup-dir.d/*/*.conf


Detailed information can then be found in the log file :file:`/var/log/nqrustbackup/nqrustbackup.log`.

Take a look into the :ref:`Release Notes <releasenotes>` to see which NQRustBackup updates do require a database scheme update.


.. warning::

   Especially the upgrade to NQRustBackup >= 17.2.0 restructures the **File** database table.
   In larger installations this is very time consuming (up to several hours or days)
   and temporarily doubles the amount of required database disk space.


.. _section-UpdateDatabaseDebianDistributions:

Debian based Linux Distributions
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Since NQRustBackup :sinceVersion:`14.2.0: dbconfig-common (Debian)` the Debian (and Ubuntu) based packages support the **dbconfig-common** mechanism to create and update the NQRustBackup database. If this is properly configured, the database schema will be automatically adapted by the NQRustBackup packages.

For details see :ref:`section-dbconfig`.

If you disabled the usage of **dbconfig-common**, follow the instructions for :ref:`section-UpdateDatabaseOtherPlatforms`.


.. _section-UpdateDatabaseOtherPlatforms:

Other Platforms
^^^^^^^^^^^^^^^

This has to be done as database administrator.
On most platforms NQRustBackup knows only about the credentials to access the NQRustBackup database, but not about the database administrator credentials to modify the database schema.

The task of updating the database schema is done by the scripts :command:`/usr/lib/nqrustbackup/scripts/update_nqrustbackup_tables` and :command:`/usr/lib/nqrustbackup/scripts/grant_nqrustbackup_privileges`.

However, this script requires administration access to the database. Depending on your distribution, this requires different preparations.

More details can be found in chapter :ref:`Catalog Maintenance <CatMaintenanceChapter>`.

.. code-block:: shell-session
   :caption: Update PostgreSQL database schema on most Linux distribution

   su postgres -c /usr/lib/nqrustbackup/scripts/update_nqrustbackup_tables
   su postgres -c /usr/lib/nqrustbackup/scripts/grant_nqrustbackup_privileges

The :command:`grant_nqrustbackup_privileges` command is required, if new databases tables are introduced. It does not hurt to run it multiple times.

After this, restart the NQRustBackup Director and verify it starts without problems.
