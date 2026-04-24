#!/usr/bin/env python
#
#   NQRUSTBACKUP - Backup Archiving REcovery Open Sourced
#
#   Copyright (C) 2019-2024 NQRustBackup GmbH & Co. KG
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


from nqrustbackup.util import argparse
import nqrustbackup.bsock
import nqrustbackup.exceptions
import inspect
import logging

# pip install python-jsonrpc
import pyjsonrpc
import sys
from types import MethodType


def add(a, b):
    """Test function"""
    return a + b


class RequestHandler(pyjsonrpc.HttpRequestHandler):
    # Register public JSON-RPC methods
    methods = {"add": add}


class NQRustBackupConsoleMethods:
    def __init__(self, nqrustbackup_console):
        self.logger = logging.getLogger()
        self.logger.debug("init")
        self.conn = nqrustbackup_console

    def execute(self, command):
        """
        Generic function to call any nqrustbackup console command.
        """
        self.logger.debug(command)
        return self.conn.call(command)

    def execute_fullresult(self, command):
        """
        Generic function to call any nqrustbackup console commands,
        and return the full result (also the pseudo jsonrpc header, not required here).
        """
        self.logger.debug(command)
        return self.conn.call_fullresult(command)

    def list(self, command):
        """
        Interface to the NQRustBackup console list command.
        """
        return self.execute("list " + command)

    def call(self, command):
        """
        legacy function, as call is a suboptimal name.
        It is used internally by python-jsonrpc.
        Use execute() instead.
        """
        return self.execute(command)


def nqrustbackup_console_methods_to_jsonrpc(nqrustbackup_console_methods):
    tuples = inspect.getmembers(nqrustbackup_console_methods, predicate=inspect.ismethod)
    methods = RequestHandler.methods
    for i in tuples:
        methods[i[0]] = getattr(nqrustbackup_console_methods, i[0])
        print(i[0])
    print(methods)
    RequestHandler.methods = methods


def getArguments():
    argparser = argparse.ArgumentParser(
        description="Run NQRustBackup Director JSON-RPC proxy."
    )
    argparser.add_argument(
        "-d", "--debug", action="store_true", help="enable debugging output"
    )
    nqrustbackup.bsock.DirectorConsoleJson.argparser_add_default_command_line_arguments(
        argparser
    )
    args = argparser.parse_args()
    return args


if __name__ == "__main__":
    logging.basicConfig(
        format="%(levelname)s %(module)s.%(funcName)s: %(message)s", level=logging.INFO
    )
    logger = logging.getLogger()

    args = getArguments()
    if args.debug:
        logger.setLevel(logging.DEBUG)

    nqrustbackup_args = nqrustbackup.bsock.DirectorConsoleJson.argparser_get_nqrustbackup_parameter(args)
    logger.debug("options: %s" % (nqrustbackup_args))
    try:
        director = nqrustbackup.bsock.DirectorConsoleJson(**nqrustbackup_args)
    except nqrustbackup.exceptions.ConnectionError as e:
        print(str(e))
        sys.exit(1)
    logger.debug("authentication successful")

    nqrustbackup_console_methods = NQRustBackupConsoleMethods(director)

    nqrustbackup_console_methods_to_jsonrpc(nqrustbackup_console_methods)

    print(nqrustbackup_console_methods.call("list jobs last"))

    # Threading HTTP-Server
    http_server = pyjsonrpc.ThreadingHttpServer(
        server_address=("localhost", 8080), RequestHandlerClass=RequestHandler
    )
    print("Starting HTTP server ...")
    print("URL: http://localhost:8080")
    http_server.serve_forever()
