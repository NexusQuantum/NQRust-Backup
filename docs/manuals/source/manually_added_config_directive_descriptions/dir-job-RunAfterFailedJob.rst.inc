This is a shortcut for the :config:option:`dir/job/RunScript`\  resource, that runs a command after a failed job.

If the exit code of the program run is non-zero, NQRustBackup will print a warning message.



   .. literalinclude:: /include/config/DirJobRunAfterFailedJob1.conf
      :language: nqrustbackupconfig
