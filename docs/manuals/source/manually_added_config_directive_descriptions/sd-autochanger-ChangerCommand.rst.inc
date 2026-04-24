This command specifies an external program and parameter to be called that will automatically change volumes as required by NQRustBackup. This command is invoked each time that NQRustBackup wishes to manipulate the autochanger.

Most frequently, you will specify the NQRustBackup supplied :command:`mtx-changer` script.

The following substitutions are made in the command before it is sent to the operating system for execution:

%%
   %

%a
   archive device name

%c
   changer device name

%d
   changer drive index base 0

%f
   Client’s name

%j
   Job name

%o
   command (loaded, load, or unload)

%s
   Slot base 0

%S
   Slot base 1

%v
   Volume name

A typical setting for this is :config:option:`sd/autochanger/ChangerCommand = "/usr/lib/nqrustbackup/scripts/mtx-changer \%c \%o \%S \%a \%d"`\ .

Details of the three commands currently used by NQRustBackup (loaded, load, unload) as well as the output expected by NQRustBackup are given in the :ref:`NQRustBackup Autochanger Interface <autochanger-interface>` section.

If it is specified here, it needs not to be specified in the Device resource. If it is also specified in the Device resource, it will take precedence over the one specified in the Autochanger resource.
