.. _documentationstyleguide/nqrustbackupspecificformatting/nqrustbackuplogging:nqrustbackup logging:

NQRustBackup Logging
==============

If you want to display NQRustBackup specific logs, use following formatting:

.. ${PERL} 's#\{logging\}\{(.*)\}#\n.. code-block:: sh\n    :caption: \1\n#g'   ${DESTFILE}

.. literalinclude:: /DocumentationStyleGuide/example/code-block-nqrustbackuplog.rst.inc

The output will look like this:

.. include:: /DocumentationStyleGuide/example/code-block-nqrustbackuplog.rst.inc
