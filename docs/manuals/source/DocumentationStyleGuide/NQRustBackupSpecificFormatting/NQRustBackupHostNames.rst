.. _documentationstyleguide/nqrustbackupspecificformatting/nqrustbackuphostnames:nqrustbackup host names:

NQRustBackup Host Names
=================

All host names in example should use the :strong:`example.com` domain.

Also in all examples, the same names for the |nqrustbackupDir|, NQRustBackup Storage and NQRustBackup File Daemons should be used.


If you want to display a hostname, the following formatting should be followed:

.. \newcommand{\host}[1]{\path|#1|}

   Post Conversion Changes
   ${PERL} 's#:raw-latex:`\\host\{(.*?)\}`#:strong:`\1`#g' ${DESTFILE}

.. code-block:: sh

   :strong:`host1.example.com`

The output should look like this:

:strong:`host1.example.com`


.. csv-table:: Host Names
   :header: "Host name", "Description"

   ":strong:`nqrustbackup-dir.example.com`",     "NQRustBackup Director host"
   ":strong:`nqrustbackup-sd.example.com`",      "NQRustBackup Storage Daemon host, if only one Storage Daemon is used."
   ":strong:`nqrustbackup-sd1.example.com`, :strong:`nqrustbackup-sd2.example.com`, ...", "NQRustBackup Storage Daemon host, if multiple Storage Daemons are used."
   ":strong:`nqrustbackup-sd-tape.example.com`", "NQRustBackup Storage Daemon with a specific backend."
   ":strong:`host.example.com`", "An arbitrary system, without special requirements."
   ":strong:`host1.example.com`, :strong:`host2.example.com`, ...", "NQRustBackup File Daemon"
