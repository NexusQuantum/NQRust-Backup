NQRustBackup-17.2
-----------


.. index::
   :pair: nqrustbackup-17.2.7; Release Notes

.. _nqrustbackup-17.2.7:

nqrustbackup-17.2.7
~~~~~~~~~~~~~

================ ===============================================
Code Release     2018-07-13
Database Version 2171 (unchanged)
Release Ticket   :issue:`966`
Url              http://download.nqrustbackup.com/nqrustbackup/release/17.2/
================ ===============================================

This release contains several bugfixes and enhancements. Excerpt:

-  :issue:`892` **nqrustbackup-storage-droplet**: improve error handling on unavailable backend.
-  :issue:`902` **nqrustbackup-storage-droplet**: improve job status handling (terminate job after all data is written).
-  :issue:`967` :strong:`Windows`: overwrite symbolic links on restore.
-  :issue:`983` |nqrustbackupSd|: prevent sporadic crash when :config:option:`sd/storage/CollectJobStatistics = yes`\ .
-  :strong:`SLES 12sp2` and :strong:`SLES 12sp3`: provide **nqrustbackup-storage-ceph** and **nqrustbackup-filedaemon-ceph-plugin** packages.
