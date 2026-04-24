#!/usr/bin/env python
# -*- Mode: Python; tab-width: 4 -*-
#
# NQRustBackup FileDaemon Task plugin
# Copyright (C) 2018 Marco Lertora <marco.lertora@gmail.com>
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

import NQRustBackupFdWrapper
from nqrustbackupfd import bRC_OK
from NQRustBackupFdWrapper import *
from nqrustbackup_tasks.pgsql.NQRustBackupFdPgSQLClass import NQRustBackupFdPgSQLClass

def load_nqrustbackup_plugin(plugin_def):
    NQRustBackupFdWrapper.nqrustbackup_fd_plugin_object = NQRustBackupFdPgSQLClass(plugin_def)
    return bRC_OK
