#!/usr/bin/env python
# -*- coding: utf-8 -*-
# NQRUSTBACKUP - Backup Archiving REcovery Open Sourced
#
# Copyright (C) 2022-2023 NQRustBackup GmbH & Co. KG
#
# This program is Free Software; you can redistribute it and/or
# modify it under the terms of version three of the GNU Affero General Public
# License as published by the Free Software Foundation, which is
# listed in the file LICENSE.
#
# This program is distributed in the hope that it will be useful, but
# WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
# Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program; if not, write to the Free Software
# Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
# 02110-1301, USA.

# Provided by the NQRustBackup FD Python plugin interface
import nqrustbackupfd

# This module contains the wrapper functions called by the NQRustBackup-FD, the
# functions call the corresponding methods from your plugin class
import NQRustBackupFdWrapper

from NQRustBackupFdWrapper import *  # noqa

# This module contains the used plugin class
import NQRustBackupFdPluginLocalFilesetAclXattr


def load_nqrustbackup_plugin(gcplugindef):
    """
    This function is called by the NQRustBackup-FD to load the plugin
    We use it to instantiate the plugin class
    """
    # NQRustBackupFdWrapper.nqrustbackup_fd_plugin_object is the module attribute that
    # holds the plugin class object
    NQRustBackupFdWrapper.nqrustbackup_fd_plugin_object = (
        NQRustBackupFdPluginLocalFilesetAclXattr.NQRustBackupFdPluginLocalFilesetAclXattr(
            gcplugindef
        )
    )
    return nqrustbackupfd.bRC_OK


# the rest is done in the Plugin module
