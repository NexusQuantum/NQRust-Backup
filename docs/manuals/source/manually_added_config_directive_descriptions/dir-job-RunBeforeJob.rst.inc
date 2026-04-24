This is a shortcut for the :config:option:`dir/job/RunScript`\  resource, that runs a command before a job.

If the exit code of the program run is non-zero, the current NQRustBackup job will be canceled.



   .. literalinclude:: /include/config/DirJobRunBeforeJob1.conf
      :language: nqrustbackupconfig



is equivalent to:



   .. literalinclude:: /include/config/DirJobRunBeforeJob2.conf
      :language: nqrustbackupconfig
