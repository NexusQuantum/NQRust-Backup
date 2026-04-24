NQRustBackupConsole
========

NQRustBackupConsole Command
----------------

A single nqrustbackup_console command can be written as:

.. \newcommand{\bcommand}[2]{\path|#1| \path|#2|}

::

   :bcommand:`help configure`

The output should look like this:

:bcommand:`help configure`


.. Output of a NQRustBackupConsole Command
.. ----------------------------
..
.. If you write only the output of a nqrustbackup_console command and not a complete nqrustbackup_console session, then format it like this:
..
.. .. \newcommand{\nqrustbackup_consoleOutput}[1]{\path|#1|}
..
..    Pre Conversion Changes:
..    perl -pe 's#\\bquote\{\\nqrustbackup_consoleOutput\{Building directory tree ...\}\}#\\bquote{Building directory tree ...}#g' \
..
..    Post Conversion Changes:
..    ${PERL} 's#:raw-latex:`\\bquote\{(.*?)\}`#:emphasis:`\1`#g' ${DESTFILE}
..
.. .. code-block:: sh
..
..    ``Building directory tree ...``
..
.. The output should look like this:
..
..  ``Building directory tree ...``


NQRustBackupConsole session
----------------

If you want to show complete NQRustBackup console session, then it needs to be put in a code block.

.. literalinclude:: /DocumentationStyleGuide/example/code-block-nqrustbackup_console-configure-add.rst.inc
   :language: none

The output will look like this:

.. include:: /DocumentationStyleGuide/example/code-block-nqrustbackup_console-configure-add.rst.inc

A prompt is indicated by ''*''.

To indicate user input, use the <input> decoration::

   <input>some user input here</input>

It will be highlighted in the output.

It is only possible, to use the <strong> decoration, to highlight something,
that is not a user input. In our example, the password is highlighted.
