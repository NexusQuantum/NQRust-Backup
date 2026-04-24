#   NQRUSTBACKUP - Backup Archiving REcovery Open Sourced
#
#   Copyright (C) 2016-2024 NQRustBackup GmbH & Co. KG
#
#   This program is Free Software; you can redistribute it and/or
#   modify it under the terms of version three of the GNU Affero General Public
#   License as published by the Free Software Foundation and included
#   in the file LICENSE.
#
#   This program is distributed in the hope that it will be useful, but
#   WITHOUT ANY WARRANTY; without even the implied warranty of
#   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
#   Affero General Public License for more details.
#
#   You should have received a copy of the GNU Affero General Public License
#   along with this program; if not, write to the Free Software
#   Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
#   02110-1301, USA.

"""Module to access a https://www.nqrustbackup.com backup system.

.. note::

   By default, the NQRustBackup Director (>= 18.2.4)
   uses TLS-PSK when communicating through the network.

   The Python (https://github.com/python/cpython) core module ``ssl``
   does support TLS-PSK only since Python >= 3.13.
   The section `Transport Encryption (TLS-PSK)`_ describes
   how to use TLS-PSK and about the limitations.
   For testing this module can also be used without TLS.


Preparations
============

Create some named consoles for testing:

.. code-block:: shell-session

   root@host:~# nqrustbackup_console
   *configure add console name=user1 password=secret profile=operator TlsEnable=no
   *configure add console name=user-tls password=secret profile=operator


This creates a console user with name `user1` and the profile `operator`.
The `operator` profile is a default profile that comes with the NQRustBackup Director.
It does allow most commands, but deny some dangerous commands (see ``show profile=operator``),
so it is well suited for this purpose.
Futhermore, TLS enforcement is disabled for this console user.

For testing with TLS-PSK, we also create the user `user-tls`.


Examples
========

Calling nqrustbackup-director console commands
----------------------------------------

.. code:: python

   >>> import nqrustbackup.bsock
   >>> directorconsole=nqrustbackup.bsock.DirectorConsole(address='localhost', port=9101, name='user1', password='secret')
   >>> print(directorconsole.call('help').decode("utf-8"))

This creates a console connection to a NQRustBackup Director.
This connection can be used to `call` commands.
These are the same commands as available via ``nqrustbackup_console``.

To connect to the default console instead, omit the `name` parameter:

.. code:: python

   >>> directorconsole = nqrustbackup.bsock.DirectorConsole(address='localhost', port=9101, password='defaultconsolepassword')

The result of the call method is a ``bytes`` object. In most cases, it has to be decoded to UTF-8.



Simple version of the nqrustbackup_console in Python
----------------------------------------

.. code:: python

   >>> import nqrustbackup.bsock
   >>> directorconsole = nqrustbackup.bsock.DirectorConsole(address='localhost', port=9101, password='secret')
   >>> directorconsole.interactive()

Or use the ``nqrustbackup_console.py`` script:

.. code-block:: shell-session

   nqrustbackup_console.py --debug --name=user1 --password=secret localhost


Use JSON objects of the API mode 2
----------------------------------

Requires: NQRustBackup >= 15.2

The class `DirectorConsoleJson` is inherited from `DirectorConsole`
and uses the Director Console API mode 2 (JSON).

For general information about API mode 2 and what data structures to expect,
see https://docs.nqrustbackup.org/DeveloperGuide/api.html#api-mode-2-json

Example:

.. code:: python

   >>> import nqrustbackup.bsock
   >>> directorconsole = nqrustbackup.bsock.DirectorConsoleJson(address='localhost', port=9101, password='secret')
   >>> pools = directorconsole.call('list pools')
   >>> for pool in pools["pools"]:
   ...   print(pool["name"])
   ...
   Scratch
   Incremental
   Full
   Differential


The results the the `call` method is a ``dict`` object.

In case of an error, an exception, derived from :py:class:`nqrustbackup.exceptions.Error` is raised.

Example:


.. code:: python

   >>> directorconsole.call("test it")
   Traceback (most recent call last):
   ...
   nqrustbackup.exceptions.JsonRpcErrorReceivedException: failed: test it: is an invalid command.



.. _section-python-nqrustbackup-tls-psk:

Transport Encryption (TLS-PSK)
==============================

Since NQRustBackup >= 18.2.4, NQRustBackup supports TLS-PSK (Transport-Layer-Security Pre-Shared-Key) to secure its network connections and uses this by default.

Unfortunately the Python core module ``ssl`` does support TLS-PSK only with Python >= 3.13.
For some older versions of Python,
the extra module ``sslpsk`` (see https://github.com/drbild/sslpsk) offers limited support.

Fallback To Unencrypted Connections
-----------------------------------

Normally `DirectorConsole` tries to connect using the latest known protocol version.
In order to allow connections in more environments,
the `DirectorConsole` can fall back to older protocol versions.
Specify `protocolversion = None` (or 0 as command line argument) to enable automatic fall back.
If connecting via TLS-PSK fails, it falls back to the old, unencrypted protocol version.
Depending on your nqrustbackup-director configuration, unencrypted connections will be accepted:

.. code:: python

   >>> import nqrustbackup.bsock
   /.../nqrustbackup/bsock/lowlevel.py:39: UserWarning: Connection encryption via TLS-PSK is not available (TLS-PSK is not available in the ssl module and the extra module sslpsk is not installed).
   >>> directorconsole=nqrustbackup.bsock.DirectorConsole(address='localhost', port=9101, name='user-tls', password='secret', protocolversion=None)
   socket error: Conversation terminated (-4)
   Failed to connect using protocol version 2. Trying protocol version 1.
   >>> print(directorconsole.call('help').decode("utf-8"))


To enforce a encrypted connection, use the ``tls_psk_require=True`` parameter:

.. code:: python

   >>> import nqrustbackup.bsock
   /.../nqrustbackup/bsock/lowlevel.py:39: UserWarning: Connection encryption via TLS-PSK is not available, as the module sslpsk is not installed.
   >>> directorconsole=nqrustbackup.bsock.DirectorConsole(address='localhost', port=9101, name='user-tls', password='secret', tls_psk_require=True)
   Traceback (most recent call last):
   ...
   nqrustbackup.exceptions.ConnectionError: TLS-PSK is required, but not available.


In this case, an exception is raised, if the connection can not be established via TLS-PSK.

sslpsk
------

The extra module `sslpsk` (see https://github.com/drbild/sslpsk)
extends the core module `ssl` by TLS-PSK.

At the time of writing, the lasted version installable via pip is 1.0.0 (https://pypi.org/project/sslpsk/),
which is not working with Python >= 3.

For using `python-nqrustbackup` with TLS-PSK with
Python >= 3 and Python <= 3.9
the latest version must by installed manually.
At the time of writing, even the latest version
(https://github.com/drbild/sslpsk/commit/d88123a75786953f82f5e25d6c43d9d9259acb62)
does not support Python >= 3.10.
However, Python >= 3.13 has direct support for TLS-PSK in the core `ssl` module.

Installing the `sslpsk` module manually:

.. code:: shell

   git clone https://github.com/drbild/sslpsk.git
   cd sslpsk
   python setup.py build
   python setup.py install

`python-nqrustbackup` will detect, that `sslpsk` is available and will use it automatically.
This can be verified by following command:

.. code:: python

   >>> import nqrustbackup.bsock
   >>> nqrustbackup.bsock.DirectorConsole.is_tls_psk_available()
   True

Another limitation of the current `sslpsk` version is,
that it is not able to autodetect the TLS protocol version to use.

In order to use it, specify ``tls_version`` with an appropriate protocol version.
In most cases this should be ``tls_version=ssl.PROTOCOL_TLSv1_2``,
like in the following example:

.. code:: python

   >>> import ssl
   >>> import nqrustbackup.bsock
   >>> directorconsole = nqrustbackup.bsock.DirectorConsoleJson(address='localhost', user='user-tls', password='secret', tls_version=ssl.PROTOCOL_TLSv1_2)
   >>> print(directorconsole.call('help').decode("utf-8"))
"""

from nqrustbackup.exceptions import *
from nqrustbackup.util.password import Password
from nqrustbackup.bsock.connectiontype import ConnectionType
from nqrustbackup.bsock.constants import Constants
from nqrustbackup.bsock.filedaemon import FileDaemon
from nqrustbackup.bsock.directorconsole import DirectorConsole
from nqrustbackup.bsock.directorconsolejson import DirectorConsoleJson
from nqrustbackup.bsock.protocolversions import ProtocolVersions
from nqrustbackup.bsock.tlsversionparser import TlsVersionParser

# compat
from nqrustbackup.bsock.bsock import BSock
from nqrustbackup.bsock.bsockjson import BSockJson
