Send the message to the system log (syslog).

Since :sinceVersion:`14.4.0: Syslog Level` the facility can be specified in the address field and the loglevel correspond to the NQRustBackup :ref:`MessageTypes`. The defaults are :strong:`DAEMON` and :strong:`LOG_ERR`.

Although the syslog destination is not used in the default NQRustBackup config files, in certain cases where NQRustBackup encounters errors in trying to deliver a message, as a last resort, it will send it to the system syslog to prevent loss of the message, so you might occasionally check the syslog for NQRustBackup output.
