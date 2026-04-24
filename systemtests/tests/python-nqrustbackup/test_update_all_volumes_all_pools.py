#
#   NQRUSTBACKUP - Backup Archiving REcovery Open Sourced
#
#   Copyright (C) 2021-2021 NQRustBackup GmbH & Co. KG
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


class PythonNQRustBackupUpdateAllVolumesAllPools(nqrustbackup_unittest.Json):
    def test_updateAllVolumesAllPools(self):
        """
        This test checks if updating all volumes of a pool has the right message even if there are no volumes in the pool.
        """
        logger = logging.getLogger()

        username = self.get_operator_username()
        password = self.get_operator_password(username)

        directorJson = nqrustbackup.bsock.DirectorConsoleJson(
            address=self.director_address,
            port=self.director_port,
            name=username,
            password=password,
            **self.director_extra_options
        )

        directorRegular = nqrustbackup.bsock.DirectorConsole(
            address=self.director_address,
            port=self.director_port,
            name=username,
            password=password,
            **self.director_extra_options
        )

        newrandompoolname = u"arandompool"

        # check that we do not have any volumes
        self.configure_add(
            directorJson,
            "pools",
            newrandompoolname,
            "pool={}".format(newrandompoolname),
        )
        directorRegular.call("reload")
        directorRegular.call("update volume")
        directorRegular.call("13")  # choosing the `All Volumes from pool` option
        result = directorRegular.call("1")  # choosing the `arandompool` option

        self.assertEqual(
            result.decode(),
            'All Volume defaults updated from "{}" Pool record.\n'.format(
                newrandompoolname
            ),
        )

        directorRegular.call("delete pool={} yes".format(newrandompoolname))
        os.remove("etc/nqrustbackup/nqrustbackup-dir.d/pool/{}.conf".format(newrandompoolname))
