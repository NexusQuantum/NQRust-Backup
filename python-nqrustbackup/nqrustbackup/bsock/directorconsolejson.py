#   NQRUSTBACKUP - Backup Archiving REcovery Open Sourced
#
#   Copyright (C) 2016-2021 NQRustBackup GmbH & Co. KG
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

"""
Communicate with the NQRustBackup Director Daemon Console interface in API mode 2 (JSON).
"""

from nqrustbackup.bsock.directorconsole import DirectorConsole
import nqrustbackup.exceptions
from pprint import pformat, pprint
import json


class DirectorConsoleJson(DirectorConsole):
    """Communicate with the NQRustBackup Director Daemon Console interface in API mode 2 (JSON).

    Example:

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

       In case of an error, an exception, derived from ``nqrustbackup.exceptions.Error`` is raised.
    """

    def __init__(self, *args, **kwargs):
        """\
        **Parameters:** The parameter are identical to :py:class:`nqrustbackup.bsock.directorconsole.DirectorConsole`.

        Raises:
            nqrustbackup.exceptions.JsonRpcInvalidJsonReceivedException:
                if the ".api" command is not available.
        """
        super(DirectorConsoleJson, self).__init__(*args, **kwargs)

    def _init_connection(self):
        # older version did not support compact mode,
        # therfore first set api mode to json (which should always work in nqrustbackup >= 15.2.0)
        # and then set api mode json compact (which should work with nqrustbackup >= 15.2.2)
        self.logger.debug(self.call(".api json"))
        self.logger.debug(self.call(".api json compact=yes"))

    def call(self, command):
        """Calls a command on the NQRustBackup Director and returns its result.

        If the JSON-RPC result indicates an error
        (contains the ``error`` element),
        an exception will be raised.

        Args:
           command (str or list): Command to execute. Best provided as a list.

        Returns:
            dict: Result received from the NQRustBackup Director.

        Raises:
            nqrustbackup.exceptions.JsonRpcErrorReceivedException:
                if an JSON-RPC error object is received.
            nqrustbackup.exceptions.JsonRpcInvalidJsonReceivedException:
                if an invalid JSON-RPC result is received.
        """
        json = self.call_fullresult(command)
        if json == None:
            return
        if "result" in json:
            result = json["result"]
        elif "error" in json:
            raise nqrustbackup.exceptions.JsonRpcErrorReceivedException(json)
        else:
            raise nqrustbackup.exceptions.JsonRpcInvalidJsonReceivedException(json)
        return result

    def call_fullresult(self, command):
        """Calls a command on the NQRustBackup Director and returns its result.

        Returns:
            dict: Result received from the NQRustBackup Director.

        Raises:
            nqrustbackup.exceptions.JsonRpcInvalidJsonReceivedException:
                if an invalid JSON-RPC result is received.
        """
        resultstring = super(DirectorConsoleJson, self).call(command)
        data = None
        if resultstring:
            try:
                data = json.loads(resultstring.decode("utf-8"))
            except ValueError as e:
                # in case result is not valid json,
                # create a JSON-RPC wrapper
                data = {"error": {"code": 2, "message": str(e), "data": resultstring}}
                raise nqrustbackup.exceptions.JsonRpcInvalidJsonReceivedException(data)
        return data

    def _show_result(self, msg):
        pprint(msg)
