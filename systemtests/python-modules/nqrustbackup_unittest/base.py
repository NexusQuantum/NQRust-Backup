#!/usr/bin/env python
#
#   NQRUSTBACKUP - Backup Archiving REcovery Open Sourced
#
#   Copyright (C) 2019-2025 NQRustBackup GmbH & Co. KG
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

import logging
import os
import re
import subprocess
from time import sleep
import unittest

import nqrustbackup.bsock
from nqrustbackup.bsock.constants import Constants
from nqrustbackup.bsock.protocolmessages import ProtocolMessages
from nqrustbackup.bsock.protocolversions import ProtocolVersions
from nqrustbackup.bsock.lowlevel import LowLevel
import nqrustbackup.exceptions


class Base(unittest.TestCase):

    config_directory = "/etc/nqrustbackup"

    director_name = "nqrustbackup-dir"

    director_address = "localhost"
    director_port = 9101
    director_root_password = "secret"
    director_extra_options = {}
    client = "nqrustbackup-fd"

    filedaemon_address = "localhost"
    filedaemon_port = 9102
    filedaemon_director_password = "secret"

    dbcheck_binary = "dbcheck"

    # restorefile = '/usr/sbin/nqrustbackup_console'
    # path to store logging files
    backup_directory = "tmp/data/"
    debug = False
    logpath = "{}/PythonNQRustBackupTest.log".format(os.getcwd())

    @classmethod
    def setUpClass(cls):
        # Configure the logger, for information about the timings set it to INFO
        cls.get_env()

        logging.basicConfig(
            filename=cls.logpath,
            format="%(levelname)s %(module)s.%(funcName)s: %(message)s",
            level=logging.INFO,
        )
        logger = logging.getLogger()
        if cls.debug:
            logger.setLevel(logging.DEBUG)
        logger.debug("setUpClass")

    def setUp(self):
        logger = logging.getLogger()
        logger.debug("setUp")

    def tearDown(self):
        logger = logging.getLogger()
        logger.debug("tearDown\n\n\n")

    def get_name_of_test(self):
        return self.id().split(".", 1)[1]

    def get_operator_username(self, tls=None):
        if tls is None:
            if nqrustbackup.bsock.DirectorConsole.is_tls_psk_available():
                tls = True
            else:
                tls = False
        if tls:
            return "admin-tls"
        else:
            return "admin-notls"

    def get_operator_password(self, username=None):
        return nqrustbackup.bsock.Password("secret")

    @staticmethod
    def append_to_file(filename, data):
        with open(filename, "a") as writer:
            writer.write(data)
            writer.flush()

    def run_dbcheck(self):
        logger = logging.getLogger()
        cmd = [self.dbcheck_binary, "-c", self.config_directory, "-vvv", "-b", "-f"]
        logger.debug("calling: {}".format(" ".join(cmd)))
        output = subprocess.check_output(cmd, stderr=subprocess.PIPE)
        logger.debug("result: {}".format(output))

    @classmethod
    def get_env(cls):
        """
        Get attributes as environment variables,
        if not available or set use defaults.
        """

        config_directory = os.environ.get("NQRUSTBACKUP_CONFIG_DIR")
        if config_directory:
            cls.config_directory = config_directory

        director_root_password = os.environ.get("dir_password")
        if director_root_password:
            cls.director_root_password = director_root_password

        director_port = os.environ.get("NQRUSTBACKUP_DIRECTOR_PORT")
        if director_port:
            cls.director_port = director_port

        filedaemon_director_password = os.environ.get("fd_password")
        if filedaemon_director_password:
            cls.filedaemon_director_password = filedaemon_director_password

        filedaemon_port = os.environ.get("NQRUSTBACKUP_FD_PORT")
        if filedaemon_port:
            cls.filedaemon_port = filedaemon_port

        tls_version_str = os.environ.get("PYTHON_NQRUSTBACKUP_TLS_VERSION")
        if tls_version_str is not None:
            tls_version_parser = nqrustbackup.bsock.TlsVersionParser()
            tls_version = tls_version_parser.get_protocol_version_from_string(
                tls_version_str
            )
            if tls_version is not None:
                cls.director_extra_options["tls_version"] = tls_version
            else:
                print(
                    "Environment variable PYTHON_NQRUSTBACKUP_TLS_VERSION has invalid value ({}). Valid values: {}".format(
                        tls_version_str,
                        ", ".join(tls_version_parser.get_protocol_versions()),
                    )
                )

        backup_directory = os.environ.get("BackupDirectory")
        if backup_directory:
            cls.backup_directory = backup_directory

        dbcheck_binary = os.environ.get("NQRUSTBACKUP_DBCHECK_BINARY")
        if dbcheck_binary:
            cls.dbcheck_binary = dbcheck_binary

        if os.environ.get("REGRESS_DEBUG"):
            cls.debug = True
