.. code-block:: shell-session
   :caption: Access the local PostgreSQL database

   root@host:~# cd /etc/nqrustbackup
   root@host:/etc/nqrustbackup# ls -1
   nqrustbackup-fd.d
   nqrustbackup_console.conf
   tray-monitor.d
   root@host:/etc/nqrustbackup# cd
   root@host:~# ssh root@nqrustbackup-dir.example.com
   root@nqrustbackup-dir:~# su - postgres
   postgres@nqrustbackup-dir:~$ psql nqrustbackup
   Welcome to psql 8.3.23, the PostgreSQL interactive terminal.
   ...
