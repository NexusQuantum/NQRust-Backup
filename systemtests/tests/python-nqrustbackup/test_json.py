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

# -*- coding: utf-8 -*-

from __future__ import print_function
import json
import logging
import os
import re
import subprocess
from time import sleep
import unittest
import warnings

import nqrustbackup.bsock
from nqrustbackup.bsock.constants import Constants
from nqrustbackup.bsock.protocolmessages import ProtocolMessages
from nqrustbackup.bsock.protocolversions import ProtocolVersions
from nqrustbackup.bsock.lowlevel import LowLevel
import nqrustbackup.exceptions

import nqrustbackup_unittest

#
# Test with JSON backend
#


class PythonNQRustBackupJsonBackendTest(nqrustbackup_unittest.Json):
    def test_json_backend(self):
        logger = logging.getLogger()

        username = self.get_operator_username()
        password = self.get_operator_password(username)
        client = self.client

        director = nqrustbackup.bsock.DirectorConsoleJson(
            address=self.director_address,
            port=self.director_port,
            name=username,
            password=password,
            **self.director_extra_options
        )
        result = director.call("list clients")
        logger.debug(str(result))
        # test if self.client is in the result of "list clients"
        for i in result["clients"]:
            logger.debug(str(i))
            if i["name"] == client:
                return
        self.fail('Failed to retrieve client {} from "list clients"'.format(client))

    def test_json_with_invalid_command(self):
        logger = logging.getLogger()

        username = self.get_operator_username()
        password = self.get_operator_password(username)

        director = nqrustbackup.bsock.DirectorConsoleJson(
            address=self.director_address,
            port=self.director_port,
            name=username,
            password=password,
            **self.director_extra_options
        )

        with self.assertRaises(nqrustbackup.exceptions.JsonRpcErrorReceivedException):
            result = director.call("invalidcommand")

    def test_json_whoami(self):
        logger = logging.getLogger()

        username = self.get_operator_username()
        password = self.get_operator_password(username)

        director = nqrustbackup.bsock.DirectorConsoleJson(
            address=self.director_address,
            port=self.director_port,
            name=username,
            password=password,
            **self.director_extra_options
        )
        result = director.call("whoami")
        logger.debug(str(result))
        self.assertEqual(username, result["whoami"])

    @unittest.skip("Most commands do return valid JSON")
    def test_json_backend_with_invalid_json_output(self):
        logger = logging.getLogger()

        # This command sends additional plain (none JSON) output.
        # Therefore the result is not valid JSON.
        # Used "show clients" earlier,
        # however, this now produces valid output.
        # Commands like 'status storage' (and 'status client') only produces empty output.
        # The "messages" command shows plain output in JSON mode,
        # but only if there are pending messages.
        bcmd = "show clients"

        username = self.get_operator_username()
        password = self.get_operator_password(username)

        director_plain = nqrustbackup.bsock.DirectorConsole(
            address=self.director_address,
            port=self.director_port,
            name=username,
            password=password,
            **self.director_extra_options
        )

        result = director_plain.call(bcmd)
        logger.debug(str(result))

        director_json = nqrustbackup.bsock.DirectorConsoleJson(
            address=self.director_address,
            port=self.director_port,
            name=username,
            password=password,
            **self.director_extra_options
        )

        # The JsonRpcInvalidJsonReceivedException
        # is inherited from JsonRpcErrorReceivedException,
        # so both exceptions could by tried.
        with self.assertRaises(nqrustbackup.exceptions.JsonRpcInvalidJsonReceivedException):
            result = director_json.call(bcmd)

        with self.assertRaises(nqrustbackup.exceptions.JsonRpcErrorReceivedException):
            result = director_json.call(bcmd)

    def test_json_no_api_command(self):
        """
        The nqrustbackup.bsock.DirectorConsoleJson calls .api on initialization.
        This test verifies, that a exception is raised,
        when it is not available.
        """
        logger = logging.getLogger()

        username = "noapicommand"
        password = "secret"

        nqrustbackup_password = nqrustbackup.bsock.Password(password)
        with self.assertRaises(nqrustbackup.exceptions.JsonRpcInvalidJsonReceivedException):
            # We expect, that an exception is raised,
            # as con class initialization,
            # the ".api json" command is called
            # and the "noapicommand" don't have access to this command.
            director = nqrustbackup.bsock.DirectorConsoleJson(
                address=self.director_address,
                port=self.director_port,
                name=username,
                password=nqrustbackup_password,
                **self.director_extra_options
            )
