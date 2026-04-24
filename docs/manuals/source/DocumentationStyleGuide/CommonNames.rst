.. _documentationstyleguide/commonnames:common names:

Common Names
============

Specific strings are used again and again in the NQRustBackup documentation.

Here we define how these name should be written (upper or lower case, in one word vs. multiple words, ...).

Text substitutions can be defined in :file:`conf.py` file.

NQRustBackup Names
------------

+----------------------------------------+-------------------------------------------+
|      **Text to be Displayed**          |           **Text Formatting**             |
+----------------------------------------+-------------------------------------------+
| NQRustBackup 	                         | NQRustBackup                                    |
+----------------------------------------+-------------------------------------------+
| |dir| 	                         | ``|dir|``                                 |
+----------------------------------------+-------------------------------------------+
| |sd|	         	                 | ``|sd|``                                  |
+----------------------------------------+-------------------------------------------+
| |fd|                                   | ``|fd|``       		             |
+----------------------------------------+-------------------------------------------+
| |nqrustbackup_console|                             | ``|nqrustbackup_console|``                            |
+----------------------------------------+-------------------------------------------+
| |webui|         			 | ``|webui|``                               |
+----------------------------------------+-------------------------------------------+
| |traymonitor|                          | ``|traymonitor|``                         |
+----------------------------------------+-------------------------------------------+
| NQRustBackup Subscription                    | NQRustBackup Subscription                       |
+----------------------------------------+-------------------------------------------+
| NQRustBackup Subscription customers          | NQRustBackup Subscription customers             |
+----------------------------------------+-------------------------------------------+
| NQRustBackup Subscription repositories       | NQRustBackup Subscription repositories          |
+----------------------------------------+-------------------------------------------+
| NQRustBackup Community repositories          | NQRustBackup Community repositories             |
+----------------------------------------+-------------------------------------------+


The name **NQRustBackup** should always be written with capital B (except in technical terms like URLs, releases (nqrustbackup-18.2.5) or host names).


NQRustBackup Paths and Filenames
--------------------------

+----------------------------------------+---------------------------------------------+-----------------------------------------+
|      **File/Directory**                |            **Text Formatting**              |               **Output**                |
+----------------------------------------+---------------------------------------------+-----------------------------------------+
| NQRustBackup Configuration Path              | ``:file:`/etc/nqrustbackup/```                    | :file:`/etc/nqrustbackup/`                    |
+----------------------------------------+---------------------------------------------+-----------------------------------------+
| NQRustBackup Template Configuration Path     | ``:file:`/usr/lib/nqrustbackup/defaultconfigs/``` | :file:`/usr/lib/nqrustbackup/defaultconfigs/` |
+----------------------------------------+---------------------------------------------+-----------------------------------------+
| NQRustBackup Disk Storage Path               | ``:file:`/var/lib/nqrustbackup/storage/```        | :file:`/var/lib/nqrustbackup/storage/`        |
+----------------------------------------+---------------------------------------------+-----------------------------------------+
| NQRustBackup Scripts                         | ``:file:`/usr/lib/nqrustbackup/scripts/```        | :file:`/usr/lib/nqrustbackup/scripts/`        |
+----------------------------------------+---------------------------------------------+-----------------------------------------+
| Configuration Directory - Director     | ``:file:`/etc/nqrustbackup/nqrustbackup-dir.d/```       | :file:`/etc/nqrustbackup/nqrustbackup-dir.d/`       |
+----------------------------------------+---------------------------------------------+-----------------------------------------+
| Configuration Directory - Sd           | ``:file:`/etc/nqrustbackup/nqrustbackup-sd.d/```        | :file:`/etc/nqrustbackup/nqrustbackup-sd.d/`        |
+----------------------------------------+---------------------------------------------+-----------------------------------------+
| Configuration Directory - Fd           | ``:file:`/etc/nqrustbackup/nqrustbackup-sd.d/```        | :file:`/etc/nqrustbackup/nqrustbackup-sd.d/`        |
+----------------------------------------+---------------------------------------------+-----------------------------------------+
| Configuration Directory - Tray Monitor | ``:file:`/etc/nqrustbackup/tray-monitor.d/```     | :file:`/etc/nqrustbackup/tray-monitor.d/`     |
+----------------------------------------+---------------------------------------------+-----------------------------------------+
| Configuration File - NQRustBackupConsole          | ``:file:`/etc/nqrustbackup/nqrustbackup_console.conf```       | :file:`/etc/nqrustbackup/nqrustbackup_console.conf`       |
+----------------------------------------+---------------------------------------------+-----------------------------------------+
| NQRustBackup Log File                        | ``:file:`/var/log/nqrustbackup/nqrustbackup.log```      | :file:`/var/log/nqrustbackup/nqrustbackup.log`      |
+----------------------------------------+---------------------------------------------+-----------------------------------------+


NDMP
----

.. csv-table:: NDMP Names
   :header: "Text to be Displayed", "Text Formatting"

   "Data Management Agent", "Data Management Agent"
   "Data Agent",            "Data Agent"
   "Tape Agent",            "Tape Agent"
   "Robot Agent",           "Robot Agent"
   |ndmpnqrustbackup|,            ``|ndmpnqrustbackup|``
   |ndmpnative|,            ``|ndmpnative|``


Products
--------

.. csv-table:: Product Names
   :header: "Text to be Displayed", "Text Formatting", "Description"

   arm64,        arm64,            ARM64 compatible CPUs
   |github|,     ``|github|``,
   |ktls|,       ``|ktls|``,       Linux Kernel TLS
   |mysql|,      ``|mysql|``,
   open-source,  open-source,
   OpenSSL,      OpenSSL,
   |postgresql|, ``|postgresql|``,
   ReaR,         ReaR,             `Relax-and-Recover <https://relax-and-recover.org/>` - Linux Disaster Recovery
   reST,         reST,             reStructuredText
   TLS-PSK,      TLS-PSK,          Transport Layer Security pre-shared key ciphersuites (TLS-PSK)
   |vmware|,     ``|vmware|``,
   |vsphere|,    ``|vsphere|``,
   x86,          x86,              Intel compatible CPUs
