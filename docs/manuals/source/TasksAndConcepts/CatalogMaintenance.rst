.. _CatMaintenanceChapter:

Catalog Maintenance
===================

.. index::
   single: Maintenance; Catalog
   single: Catalog Maintenance

Catalog Database
----------------

NQRustBackup stores its catalog in a PostgreSQL database.
The database often runs on the same server as the |dir|. However, it is also possible to run it on a different system. This might require some more manual configuration, an example can be found in :ref:`catalog-maintenance-remote-psql`.


.. _section-dbconfig:

dbconfig-common (Debian)
~~~~~~~~~~~~~~~~~~~~~~~~

.. index::
   single: Platform; Debian; dbconfig-common
   single: Platform; Ubuntu; dbconfig-common

Since NQRustBackup :sinceVersion:`14.2.0: dbconfig-common (Debian)` the Debian and Ubuntu packages support the **dbconfig-common** mechanism to create and update the NQRustBackup database, according to the user choices.

The first choice is, if **dbconfig-common** should be used at all. If you decide against it, the database must be configured manually, see :ref:`CatMaintenanceManualConfiguration`.

If you decided to use **dbconfig-common**, the next question will be asked.

.. image:: /include/images/dbconfig-1-enable.*

The **dbconfig-common** configuration (and credentials) is done by the **nqrustbackup-database-common** package. Settings are stored in the file :file:`/etc/dbconfig-common/nqrustbackup-database-common.conf`.

The NQRustBackup database backend will get automatically configured in :file:`/etc/nqrustbackup/nqrustbackup-dir.d/catalog/MyCatalog.conf`. If the Server is not running locally you need to specify :config:option:`dir/catalog/DbAddress`\  in the catalog resource. A later reconfiguration might require manual changes.

.. note::

   If you need to debug installation or configuration steps,
   you can export the variable **dbc_debug**
   with :command:`export dbc_debug=1` command
   before using :command:`apt` or :command:`dpkg-reconfigure nqrustbackup-database-common`.


.. note::

   In case you want to answer/see all low level questions of dbconfig-common, prepare your
   environment with the following command:

   .. code-block:: shell-session
      :caption: Allow all dbconfig-common question to be shown

      export DEBIAN_PRIORITY=low
      apt install nqrustbackup-database-common


.. note::

   To show all dbconfig settings for nqrustbackup-database-common use:

   .. code-block:: shell-session
      :caption: show dbconfig-common settings for nqrustbackup-database-common

      root@host:~# debconf-show nqrustbackup-database-common | sort
        nqrustbackup-database-common/app-password-confirm: (password omitted)
      * nqrustbackup-database-common/database-type: pgsql
        nqrustbackup-database-common/db/app-user: nqrustbackup@localhost
      * nqrustbackup-database-common/dbconfig-install: true
        nqrustbackup-database-common/dbconfig-reinstall: false
        nqrustbackup-database-common/dbconfig-remove: true
        nqrustbackup-database-common/dbconfig-upgrade: true
        nqrustbackup-database-common/db/dbname: nqrustbackup
        nqrustbackup-database-common/install-error: abort
        nqrustbackup-database-common/internal/reconfiguring: false
        nqrustbackup-database-common/internal/skip-preseed: false
        nqrustbackup-database-common/missing-db-package-error: abort
        nqrustbackup-database-common/password-confirm: (password omitted)
        nqrustbackup-database-common/passwords-do-not-match:
        nqrustbackup-database-common/pgsql/admin-pass: (password omitted)
        nqrustbackup-database-common/pgsql/admin-user: postgres
        nqrustbackup-database-common/pgsql/app-pass: (password omitted)
        nqrustbackup-database-common/pgsql/authmethod-admin: ident
        nqrustbackup-database-common/pgsql/authmethod-user: ident
        nqrustbackup-database-common/pgsql/changeconf: false
        nqrustbackup-database-common/pgsql/manualconf:
        nqrustbackup-database-common/pgsql/method: Unix socket
        nqrustbackup-database-common/pgsql/no-empty-passwords:
        nqrustbackup-database-common/purge: false
      * nqrustbackup-database-common/remote/host: localhost
        nqrustbackup-database-common/remote/newhost: localhost
        nqrustbackup-database-common/remote/port:
        nqrustbackup-database-common/remove-error: abort
        nqrustbackup-database-common/upgrade-backup: false
        nqrustbackup-database-common/upgrade-error: abort


.. _CatMaintenanceManualConfiguration:

Manual Configuration
~~~~~~~~~~~~~~~~~~~~

NQRustBackup comes with a number of scripts to prepare and update the databases. All these scripts are located in the NQRustBackup script directory, normally at :file:`/usr/lib/nqrustbackup/scripts/`.

================================= ============== ===================================================
**Script**                        **Stage**      **Description**
================================= ============== ===================================================
:file:`create_nqrustbackup_database`    installation   create NQRustBackup database
:file:`make_nqrustbackup_tables`        installation   create NQRustBackup tables
:file:`grant_nqrustbackup_privileges`   installation   grant database access privileges
:file:`update_nqrustbackup_tables [-f]` update         update the database schema
:file:`drop_nqrustbackup_tables`        deinstallation remove NQRustBackup database tables
:file:`drop_nqrustbackup_database`      deinstallation remove NQRustBackup database
:file:`make_catalog_backup`       backup         backup the NQRustBackup database
:file:`delete_catalog_backup`     backup helper  remove the temporary NQRustBackup database backup file
================================= ============== ===================================================

The database preparation scripts have following configuration options:

db_name
   -  environment variable ``db_name``\

   -  :config:option:`dir/catalog/DbName`\  from the configuration

   -  default: nqrustbackup

db_user
   -  environment variable ``db_user``\

   -  :config:option:`dir/catalog/DbUser`\  from the configuration

   -  default: nqrustbackup

db_password
   -  environment variable ``db_password``\

   -  :config:option:`dir/catalog/DbPassword`\  from the configuration

   -  default: *none*

Reading the settings from the configuration require read permission for the current user. The normal PostgreSQL administrator user (**postgres**) doesn’t have these permissions. So if you plan to use non-default database settings, you might add the user **postgres** to the group :strong:`nqrustbackup`.

The database preparation scripts need to have password-less administrator access to the database. Depending on the distribution you’re using, this requires additional configuration. See the following section about howto achieve this for the different database systems.

To view and test the currently configured settings, use following commands:

.. code-block:: shell-session
   :caption: Show current database configuration

   /usr/sbin/nqrustbackup-dir --xc Catalog MyCatalog
   Catalog {
      Name = "MyCatalog"
      DbPassword = YourPassword
      DbUser = "nqrustbackup"
      DbName = "nqrustbackup"
   }

.. code-block:: shell-session
   :caption: Test the database connection. Example: wrong password

   /usr/sbin/nqrustbackup-dir -t -f -d 500
   [...]
   nqrustbackup-dir (100): cats/postgresql.cc:971-0 db_init_database first time
   nqrustbackup-dir (50): cats/postgresql.cc:226-0 pg_real_connect failed
   nqrustbackup-dir (50): cats/postgresql.cc:228-0 db_user=nqrustbackup db_name=nqrustbackup db_password=YourPasswordWrong
   nqrustbackup-dir: dird/check_catalog.cc:64-0 Could not open Catalog "mycatalog", database "nqrustbackup".
   nqrustbackup-dir: dird/check_catalog.cc:71-0 cats/postgresql.cc:232 Unable to connect to PostgreSQL server. Database=nqrustbackup User=nqrustbackup
   Possible causes: SQL server not running; password incorrect; max_connections exceeded.
      (connection to server on socket "/run/postgresql/.s.PGSQL.5432" failed: FATAL:  password authentication failed for user "nqrustbackup")
   nqrustbackup-dir ERROR TERMINATION
   Please correct the configuration in /etc/nqrustbackup/nqrustbackup-dir.d/*/*.conf

PostgreSQL configuration
^^^^^^^^^^^^^^^^^^^^^^^^

On most distributions, PostgreSQL uses `ident` to allow access to the local database system. The database administrator account is the Unix user **postgres**. Normally, this user can access the database without password, as the ident mechanism is used to identify the user.

If this works on your system can be verified by

.. code-block:: shell-session
   :caption: Access the local PostgreSQL database

   su - postgres
   psql

If your database is configured to require a password, this must be defined in the file `~/.pgpass <https://www.postgresql.org/docs/current/libpq-pgpass.html>`_ in the following syntax: :strong:`HOST:PORT:DATABASE:USER:PASSWORD`, e.g.

.. code-block:: cfg
   :caption: PostgreSQL access credentials

   localhost:*:nqrustbackup:nqrustbackup:secret

The permission of this file must be 0600 (:command:`chmod 0600 ~/.pgpass`).

Again, verify that you have specified the correct settings by calling the :command:`psql` command. If this connects you to the database, your credentials are good. Exit the PostgreSQL client and run the NQRustBackup database preparation scripts:

.. code-block:: shell-session
   :caption: Setup NQRustBackup catalog database

   su - postgres
   /usr/lib/nqrustbackup/scripts/create_nqrustbackup_database
   /usr/lib/nqrustbackup/scripts/make_nqrustbackup_tables
   /usr/lib/nqrustbackup/scripts/grant_nqrustbackup_privileges

The encoding of the nqrustbackup database must be :strong:`SQL_ASCII`. The command :command:`create_nqrustbackup_database` automatically creates the database with this encoding. This can be verified by the command :command:`psql -l`, which shows information about existing databases:

.. code-block:: shell-session
   :caption: List existing databases

   psql -l
           List of databases
      Name    |  Owner   | Encoding
   -----------+----------+-----------
    nqrustbackup    | postgres | SQL_ASCII
    postgres  | postgres | UTF8
    template0 | postgres | UTF8
    template1 | postgres | UTF8
   (4 rows)

The owner of the database may vary. The NQRustBackup database maintenance scripts don’t change the default owner of the NQRustBackup database, so it stays at the PostgreSQL administration user. The :command:`grant_nqrustbackup_privileges` script grant the required permissions to the NQRustBackup database user. In contrast, when installing (not updating) using :ref:`dbconfig <section-dbconfig>`, the database owner will be identical with the NQRustBackup database user.

By default, using PostgreSQL ident, a Unix user can access a database of the same name. Therefore the user **nqrustbackup** can access the database :file:`nqrustbackup`.

.. code-block:: shell-session
   :caption: Verify NQRustBackup database on PostgreSQL as Unix user nqrustbackup (nqrustbackup-13.2.3)

   root@linux:~# su - nqrustbackup -s /bin/sh
   nqrustbackup@linux:~# psql
   Welcome to psql 8.3.23, the PostgreSQL interactive terminal.

   Type:  \copyright for distribution terms
          \h for help with SQL commands
          \? for help with psql commands
          \g or terminate with semicolon to execute query
          \q to quit

   nqrustbackup=> \dt
                    List of relations
    Schema |          Name          | Type  |  Owner
   --------+------------------------+-------+----------
    public | basefiles              | table | postgres
    public | cdimages               | table | postgres
    public | client                 | table | postgres
    public | counters               | table | postgres
    public | device                 | table | postgres
    public | devicestats            | table | postgres
    public | file                   | table | postgres
    public | filename               | table | postgres
    public | fileset                | table | postgres
    public | job                    | table | postgres
    public | jobhisto               | table | postgres
    public | jobmedia               | table | postgres
    public | jobstats               | table | postgres
    public | location               | table | postgres
    public | locationlog            | table | postgres
    public | log                    | table | postgres
    public | media                  | table | postgres
    public | mediatype              | table | postgres
    public | ndmpjobenvironment     | table | postgres
    public | ndmplevelmap           | table | postgres
    public | path                   | table | postgres
    public | pathhierarchy          | table | postgres
    public | pathvisibility         | table | postgres
    public | pool                   | table | postgres
    public | quota                  | table | postgres
    public | restoreobject          | table | postgres
    public | status                 | table | postgres
    public | storage                | table | postgres
    public | unsavedfiles           | table | postgres
    public | version                | table | postgres
   (30 rows)

   nqrustbackup=> select * from Version;
    versionid
   -----------
         2002
   (1 row)

   nqrustbackup=> \du
                                    List of roles
      Role name   | Superuser | Create role | Create DB | Connections | Member of
   ---------------+-----------+-------------+-----------+-------------+-----------
    nqrustbackup        | no        | no          | no        | no limit    | {}
    postgres      | yes       | yes         | yes       | no limit    | {}
   (2 rows)

   nqrustbackup=> \dp
                    Access privileges for database "nqrustbackup"
    Schema |               Name                |   Type   |  Access privileges
   --------+-----------------------------------+----------+--------------------------------------
    public | basefiles                         | table    | {root=arwdxt/root,nqrustbackup=arwdxt/root}
    public | basefiles_baseid_seq              | sequence | {root=rwU/root,nqrustbackup=rw/root}
   ...

   nqrustbackup=>

.. _catalog-maintenance-remote-psql:

Remote PostgreSQL Database
^^^^^^^^^^^^^^^^^^^^^^^^^^

When configuring nqrustbackup with a remote database, you need a PostgreSQL superuser login account able to connect to the remote database host.

.. note::

   The PostgreSQL connection must not be a ssl-connection.
   If the PostgreSQL server only allows ssl-connections, the database can not be opened.

Your first step is to check the connection from the |dir| host into the database.
You can then export the needed environment PG variable, and execute the scripts in the same order than local installation.
A functional connection can be verified by

.. code-block:: shell-session
   :caption: Access the remote PostgreSQL database

   export PGUSER=remotedba
   export PGHOST=nqrustbackup-database.example.com
   export PGPASSWORD=dbasecret
   psql -d postgres

With a correct configuration you can access the database. If it fails, you need to correct the PostgreSQL servers' configuration files, or the exported PGVARS.

One way to manually create the database is to execute the NQRustBackup database preparation scripts with the :strong:`export PGVARS` as explained later.
However, it is advised to use the **dbconfig-common**. Both methods require you to add the database hostname/address as :config:option:`dir/catalog/DbAddress`\ .

If you’re using **dbconfig-common** you should choose :strong:`New Host`, enter the hostname or the remote address followed by the password.
As **dbconfig-common** uses the :strong:`ident` authentication by default the first try to connect will fail. Don’t be bothered by that.
Choose :strong:`Retry` when prompted. From there, read carefully and configure the database to your needs. The authentication should be set
to password, as the ident method will not work with a remote server. Set the user and administrator according to your PostgreSQL servers settings.

Set the PostgreSQL server IP as :config:option:`dir/catalog/DbAddress`\  in :ref:`DirectorResourceCatalog`. You can also customize other parameters or use the defaults. A quick check should display your recent changes:

.. code-block:: shell-session
   :caption: Show current database configuration

   /usr/sbin/nqrustbackup-dir --xc Catalog MyCatalog
   Catalog {
      Name = "MyCatalog"
      DbAddress = nqrustbackup-database.example.com
      DbPassword = "secret"
      DbUser = "nqrustbackup"
      DbName = "nqrustbackup"
   }

If **dbconfig-common** did not succeed or you choose not to use it, run the NQRustBackup database preparation scripts with:

.. code-block:: shell-session
   :caption: Setup NQRustBackup catalog database

   export PGUSER=remotedba
   export PGHOST=nqrustbackup-database.example.com
   export PGPASSWORD=dbasecret
   /usr/lib/nqrustbackup/scripts/create_nqrustbackup_database
   /usr/lib/nqrustbackup/scripts/make_nqrustbackup_tables
   /usr/lib/nqrustbackup/scripts/grant_nqrustbackup_privileges



PostgreSQL Database
-------------------

.. index::
   single: Database; PostgreSQL
   single: PostgreSQL


Database Size Planning
~~~~~~~~~~~~~~~~~~~~~~

.. index::
   single: Size; Database
   single: Database Size

Your Catalog will grow each time you run a Job, and the space used in tables will be relaxed when a volume get recycled and the previous job removed.
You can make a calculation assuming approximately 154 bytes for each File saved and knowing the number of Files that are saved during each backup and the number of Clients you backup.

For example, suppose you do a backup of two systems, each with 100,000 files.
Suppose further that you do a Full backup weekly and an Incremental every day, and that the Incremental backup typically saves 4,000 files.
The size of your database after a month can roughly be calculated as:


::

   Size = 154 * No. Systems * (100,000 * 4 + 10,000 * 26)


Where we have assumed four weeks in a month and 26 incremental backups per month. This would give the following:


::

   Size = 154 * 2 * (100,000 * 4 + 10,000 * 26) = 203,280,000 bytes
   Indexes Size = (154 * 2 * (100,000 * 4 + 10,000 * 26))/3 = 67,760,000



So for the above two systems, we should expect to have a database size of approximately 270 Megabytes including the indexes.
Of course, this will vary according to how many files are actually backed up.

You will note that the File table (containing the file attributes) make up the large bulk of the number of records as well as the space used.

Without proper setup and maintenance, your Catalog may continue to grow indefinitely read carefully the following sections for planning free space and autovacuuming.


.. _FreeSpacePostgres:

Free space needed with PostgreSQL Database
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

To ensure that all PostgreSQL maintenance operations like vacuuming and reindexing roll out smoothly we highly recommend not to fill the disk containing the postgresql data directory to more than 50% during normal operation.
If this is not possible, you might need to make more space available at least temporary when the database is being upgraded to a new schema version.

Normal NQRustBackup operation can create huge temp file requiring free space.
Upgrading to a new PostgreSQL major version, will from time to time, impose a reindex operation which will use temp space too, especially if option :command:`concurrently` is used.

You can create and use a dedicated `tablespace` for temporary files, check `PostgreSQL documentation <https://www.postgresql.org/docs/current/manage-ag-tablespaces.html>`_\.

To check how much temp files and bytes have been used you can run the following query.

.. code-block:: nqrustbackup_console
   :caption: SQL query to show temporary number of files and bytes used

   *sql
   Entering SQL query mode.
   Terminate each query with a semicolon.
   Terminate query mode with a blank line.
   Enter SQL query: select datname,temp_files,temp_bytes
   from pg_stat_database where datname='nqrustbackup';
   +---------+------------+---------------+
   | datname | temp_files | temp_bytes    |
   +---------+------------+---------------+
   | nqrustbackup  |         35 | 7,646,920,704 |
   +---------+------------+---------------+
   Enter SQL query:
   End query mode.
   *


.. _CompactingPostgres:

Compacting Your PostgreSQL Database
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. index::
   single: Database; PostgreSQL; Compacting

Over time, as noted above, your database will tend to grow until NQRustBackup starts deleting old expired records based on retention periods. After that starts, it is expected that the database size remains constant, provided that the amount of clients and files being backed up is constant.

Note that PostgreSQL uses multiversion concurrency control (MVCC), so that an UPDATE or DELETE of a row does not immediately remove the old version of the row. Space occupied by outdated or deleted row versions is only reclaimed for reuse by new rows when running **VACUUM**. Such outdated or deleted row versions are also referred to as *dead tuples*.

Since PostgreSQL Version 8.3, autovacuum is enabled by default, so that setting up a cron job to run VACUUM is not necessary in most of the cases. Note that there are two variants of VACUUM: standard VACUUM and VACUUM FULL. Standard VACUUM only marks old row versions for reuse, it does not free any allocated disk space to the operating system. Only VACUUM FULL can free up disk space, but it requires exclusive table locks so that it can not be used in parallel with production database operations
and temporarily requires up to as much additional disk space that the table being processed occupies.

All database programs have some means of writing the database out in ASCII format and then reloading it. Doing so will re-create the database from scratch producing a compacted result, so below, we show you how you can do this for PostgreSQL.

For a PostgreSQL database, you could write the NQRustBackup database as an ASCII file (:file:`nqrustbackup.sql`) then reload it by doing the following:

.. code-block:: shell-session
   :caption: Instruction to dump and reload NQRustBackup catalog database

   pg_dump -c nqrustbackup > nqrustbackup.sql
   cat nqrustbackup.sql | psql nqrustbackup
   rm -f nqrustbackup.sql

Depending on the size of your database, this will take more or less time and a fair amount of disk space. For example, you can :command:`cd` to the location of the NQRustBackup database (typically :file:`/var/lib/pgsql/data` or possible :file:`/usr/local/pgsql/data`) and check the size.

Except from special cases PostgreSQL does not need to be dumped/restored to keep the database efficient. A normal process of vacuuming will prevent the database from getting too large. If you want to fine-tweak the database storage, commands such as VACUUM, VACUUM FULL, REINDEX, and CLUSTER exist specifically to keep you from having to do a dump/restore.

More details on this subject can be found in the PostgreSQL documentation. The page https://www.postgresql.org/docs/ contains links to the documentation for all PostgreSQL versions. The section *Routine Vacuuming* explains how VACUUM works and why it is required, see https://www.postgresql.org/docs/current/routine-vacuuming.html for the current PostgreSQL version.

.. _PostgresSize:

What To Do When The Database Keeps Growing
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Especially when a high number of files are being backed up or when working with high retention periods, it is probable that default autovacuuming will not be triggered.
When starting to use NQRustBackup with an empty Database, it is normal that the file table and other tables grow, but the growth rate should drop as soon as jobs are deleted by retention or pruning.
The file table is usually the largest table in NQRustBackup.

The reason for autovacuuming not being triggered is then probably the default setting of ``autovacuum_vacuum_scale_factor = 0.2``, the current value can be shown with the following query or looked up in ``postgresql.conf``:

.. code-block:: shell-session
   :caption: SQL statement to show the autovacuum\_vacuum\_scale\_factor parameter

   nqrustbackup=# show autovacuum_vacuum_scale_factor;
    autovacuum_vacuum_scale_factor
    --------------------------------
     0.2
     (1 row)

In essence, this means that a VACUUM is only triggered when 20% of table size are obsolete. Consequently, the larger the table is, the less frequently VACUUM will be triggered by autovacuum.
This make sense because vacuuming has a performance impact. While it is possible to override the autovacuum parameters on a table-by-table basis, it can then still be triggered at any time.

.. code-block:: shell-session
   :caption: SQL statement to set the autovacuum\_vacuum\_scale\_factor parameter for table file

   root@localhost# su postgres -c 'psql -d nqrustbackup -c "ALTER TABLE public.file SET (autovacuum_vacuum_scale_factor = 0.02);"'

To learn more details about autovacuum see https://www.postgresql.org/docs/current/routine-vacuuming.html#AUTOVACUUM

The following example shows how to configure running VACUUM on the file table by using an admin-job in NQRustBackup. The job will be scheduled to run at a time that should not run in parallel with normal backup jobs, here by scheduling it to run after the BackupCatalog job.

First step is to check the amount of dead tuples and if autovacuum triggers VACUUM:

.. code-block:: shell-session
   :caption: Check dead tuples and vacuuming on PostgreSQL

   nqrustbackup=# SELECT relname, n_dead_tup, last_vacuum, last_autovacuum, last_analyze, last_autoanalyze
   FROM pg_stat_user_tables WHERE n_dead_tup > 0 ORDER BY n_dead_tup DESC;
   -[ RECORD 1 ]----+------------------------------
   relname          | file
   n_dead_tup       | 2955116
   last_vacuum      |
   last_autovacuum  |
   last_analyze     |
   last_autoanalyze |
   -[ RECORD 2 ]----+------------------------------
   relname          | log
   n_dead_tup       | 111298
   last_vacuum      |
   last_autovacuum  |
   last_analyze     |
   last_autoanalyze |
   -[ RECORD 3 ]----+------------------------------
   relname          | job
   n_dead_tup       | 1785
   last_vacuum      |
   last_autovacuum  | 2015-01-08 01:13:20.70894+01
   last_analyze     |
   last_autoanalyze | 2014-12-27 18:00:58.639319+01
   ...

In the above example, the file table has a high number of dead tuples and it has not been vacuumed. Same for the log table, but the dead tuple count is not very high. On the job table autovacuum has been triggered.

Note that the statistics views in PostgreSQL are not persistent, their values are reset on restart of the PostgreSQL service.

To setup a scheduled admin job for vacuuming the file table, the following must be done:

#. Create a file with the SQL statements for example
   ``/usr/local/lib/nqrustbackup/scripts/postgresql_file_table_maintenance.sql``
   with the following content:

   .. code-block:: shell-session
      :caption: SQL Script for vacuuming the file table on PostgreSQL

      \t \x
      SELECT relname, n_dead_tup, last_vacuum, last_autovacuum, last_analyze, last_autoanalyze
      FROM pg_stat_user_tables WHERE relname='file';
      VACUUM VERBOSE ANALYZE file;
      SELECT relname, n_dead_tup, last_vacuum, last_autovacuum, last_analyze, last_autoanalyze
      FROM pg_stat_user_tables WHERE relname='file';
      \t \x
      SELECT table_name,
        pg_size_pretty(pg_total_relation_size(table_name)) AS total_sz,
        pg_size_pretty(pg_total_relation_size(table_name) - pg_relation_size(table_name)) AS idx_sz
        FROM ( SELECT ('"' || relname || '"' ) AS table_name
          FROM pg_stat_user_tables WHERE relname != 'batch' ) AS all_tables
        ORDER BY pg_total_relation_size(table_name) DESC LIMIT 5;

   The SELECT statements are for informational purposes only, the final statement shows the total and index disk usage of the 5 largest tables.

#. Create a shell script that runs the SQL statements, for example
   ``/usr/local/lib/nqrustbackup/scripts/postgresql_file_table_maintenance.sh``
   with the following content:

   .. code-block:: shell-session
      :caption: SQL Script for vacuuming the file table on PostgreSQL

      #!/bin/sh
      psql nqrustbackup < /usr/local/lib/nqrustbackup/scripts/postgresql_file_table_maintenance.sql

#. As in PostgreSQL only the database owner or a database superuser is allowed to run VACUUM, the script will be run as the ``postgres`` user. To permit the ``nqrustbackup`` user to run the script via ``sudo``, write the following sudo rule to a file by executing ``visudo -f /etc/sudoers.d/nqrustbackup_postgres_vacuum``:

   .. code-block:: shell-session
      :caption: sudo rule for allowing nqrustbackup to run a script as postgres

      nqrustbackup ALL = (postgres) NOPASSWD: /usr/local/lib/nqrustbackup/scripts/postgresql_file_table_maintenance.sh

   and make sure that ``/etc/sudoers`` includes it, usually by the line

   ::

      #includedir /etc/sudoers.d


#. Create the following admin job in the director configuration

   .. code-block:: shell-session
      :caption: SQL Script for vacuuming the file table on PostgreSQL

      # PostgreSQL file table maintenance job
      Job {
        Name = FileTableMaintJob
        JobDefs = DefaultJob
        Schedule = "WeeklyCycleAfterBackup"
        Type = Admin
        Priority = 20

        RunScript {
          RunsWhen = Before
          RunsOnClient = no
          Fail Job On Error = yes
          Command = "sudo -u postgres /usr/local/lib/nqrustbackup/scripts/postgresql_file_table_maintenance.sh"
        }
      }

   In this example the job will be run by the schedule WeeklyCycleAfterBackup, the ``Priority`` should be set to a higher value than ``Priority`` in the BackupCatalog job.

.. _RepairingPSQL:

Repairing Your PostgreSQL Database
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

For NQRustBackup specific problems, consider using :ref:`nqrustbackup-dbcheck` program.
In other cases, consult the PostgreSQL documents for how to repair the database.

.. _BackingUpNQRustBackup:

Backing Up Your NQRustBackup Database
-------------------------------

.. index::
   single: Backup; NQRustBackup database
   single: Backup; Catalog
   single: Database; Backup NQRustBackup database

If ever the machine on which your NQRustBackup database crashes, and you need to restore from backup tapes, one of your first priorities will probably be to recover the database. Although NQRustBackup will happily backup your catalog database if it is specified in the FileSet, this is not a very good way to do it, because the database will be saved while NQRustBackup is modifying it. Thus the database may be in an instable state. Worse yet, you will backup the database before all the NQRustBackup updates have been
applied.

To resolve these problems, you need to backup the database after all the backup jobs have been run. In addition, you will want to make a copy while NQRustBackup is not modifying it. To do so, you can use two scripts provided in the release make_catalog_backup and delete_catalog_backup. These files will be automatically generated along with all the other NQRustBackup scripts. The first script will make an ASCII copy of your NQRustBackup database into nqrustbackup.sql in the working directory you specified in your
configuration, and the second will delete the nqrustbackup.sql file.

The basic sequence of events to make this work correctly is as follows:

-  Run all your nightly backups

-  After running your nightly backups, run a Catalog backup Job

-  The Catalog backup job must be scheduled after your last nightly backup

-  You use :config:option:`dir/job/RunBeforeJob`\  to create the ASCII backup file and :config:option:`dir/job/RunAfterJob`\  to clean up

Assuming that you start all your nightly backup jobs at 1:05 am (and that they run one after another), you can do the catalog backup with the following additional Director configuration statements:

.. code-block:: nqrustbackupconfig
   :caption: nqrustbackup-dir.d/job/BackupCatalog.conf

   Job {
      Name = "BackupCatalog"
      Description = "Backup the catalog database (after the nightly save)"
      JobDefs = "DefaultJob"
      Level = Full
      FileSet="Catalog"
      Schedule = "WeeklyCycleAfterBackup"

      # This creates an ASCII copy of the catalog
      # Arguments to make_catalog_backup are:
      #  make_catalog_backup <catalog-name>
      RunBeforeJob = "/usr/lib/nqrustbackup/scripts/make_catalog_backup MyCatalog"

      # This deletes the copy of the catalog
      RunAfterJob  = "/usr/lib/nqrustbackup/scripts/delete_catalog_backup MyCatalog"

      # This sends the bootstrap via mail for disaster recovery.
      # Should be sent to another system, please change recipient accordingly
      Write Bootstrap = "|/usr/sbin/nqrustbackup_smtp -h localhost -f \"\(NQRustBackup\) \" -s \"Bootstrap for Job %j\" root@localhost"
      Priority = 11                   # run after main backup
   }

.. code-block:: nqrustbackupconfig
   :caption: nqrustbackup-dir.d/schedule/WeeklyCycleAfterBackup.conf

   # This schedule does the catalog. It starts after the WeeklyCycle
   Schedule {
     Name = "WeeklyCycleAfterBackup"
     Run = Level=Full sun-sat at 1:10
   }

.. code-block:: nqrustbackupconfig
   :caption: nqrustbackup-dir.d/fileset/Catalog.conf

   # This is the backup of the catalog
   FileSet {
     Name = "Catalog"
     Include {
       Options {
         signature=MD5
       }
       File = "/var/lib/nqrustbackup/nqrustbackup.sql" # database dump
       File = "/etc/nqrustbackup"                # configuration
     }
   }

It is preferable to write/send the :ref:`bootstrap <BootstrapChapter>` file to another computer. It will allow you to quickly recover the database backup should that be necessary. If you do not have a bootstrap file, it is still possible to recover your database backup, but it will be more work and take longer.


.. _PGDG:

Provide postgresql.service with PGDG packages
---------------------------------------------

.. index::
   single: PGDG; NQRustBackup database
   single: systemd; nqrustbackup-dir.service
   single: systemd; nqrustbackup-director.service
   single: systemd; postgresql-XX.service


If you are using packages from :strong:`PostgreSQL Global Development Group` aka :strong:`PGDG` the delivered `systemd postgresql-XX.service` needs to be edited to add the standard `postgresql.service` alias which is required to start |dir| after `postgresql`\.

You can either override `nqrustbackup-dir.service` to add the corresponding After requirement.

.. code-block:: shell-session
   :caption: Add After requirement line in nqrustbackup-dir.service with PGDG postgresql-15 server

   systemctl edit nqrustbackup-dir.service

   ### Editing /etc/systemd/system/nqrustbackup-dir.service.d/override.conf
   ### Anything between here and the comment below will become the new contents of the file

   [Unit]
   After=postgresql-15.service

   systemctl daemon-reload
   systemctl reload nqrustbackup-dir

Or surcharge the PGDG `postgresql-XX.service` to add the missing postgresql.service alias.

.. code-block:: shell-session
   :caption: Add alias postgresql.service in install section

   systemctl edit postgresql-15.service

   ### Editing /etc/systemd/system/postgresql-15.service.d/override.conf
   ### Anything between here and the comment below will become the new contents of the file

   [Install]
   Alias=postgresql.service

   systemctl daemon-reload
   systemctl enable --now postgresql-15


.. _section-JobStatistics:

Job Statistics
--------------

.. index::
   single: Statistics
   single: Job; Statistics

NQRustBackup catalog contains lot of information about your IT infrastructure, how many files, their size, the number of video or music files etc. Using NQRustBackup catalog during the day to get them permit to save resources on your servers.

In this chapter, you will find tips and information to measure NQRustBackup efficiency and report statistics.

If you want to have statistics on your backups to provide some Service Level Agreement indicators, you could use a few SQL queries on the Job table to report how many:

-  jobs have run

-  jobs have been successful

-  files have been backed up

-  ...

However, these statistics are accurate only if your job retention is greater than your statistics period. Ie, if jobs are purged from the catalog, you won’t be able to use them.

Now, you can use the :bcommand:`update stats [days=num]` console command to fill the JobHistory table with new Job records. If you want to be sure to take in account only good jobs, ie if one of your important job has failed but you have fixed the problem and restarted it on time, you probably want to delete the first bad job record and keep only the successful one. For that simply let your staff do the job, and update JobHistory table after two or three days depending on your
organization using the :strong:`[days=num]` option.

These statistics records aren’t used for restoring, but mainly for capacity planning, billings, etc.

The :config:option:`dir/director/StatisticsRetention`\  defines the length of time that NQRustBackup will keep statistics job records in the Catalog database after the Job End time. This information is stored in the ``JobHistory`` table. When this time period expires, and if user runs :bcommand:`prune stats` command, NQRustBackup will prune (remove) Job records that are older than the specified period.

You can use the following Job resource in your nightly :config:option:`dir/job = BackupCatalog`\  job to maintain statistics.

.. code-block:: nqrustbackupconfig
   :caption: nqrustbackup-dir.d/job/BackupCatalog.conf

   Job {
     Name = BackupCatalog
     ...
     RunScript {
       Console = "update stats days=3"
       Console = "prune stats yes"
       RunsWhen = After
       RunsOnClient = no
     }
   }
