#!/usr/bin/env python
# -*- coding: utf-8 -*-
# NQRUSTBACKUP - Backup Archiving REcovery Open Sourced
#
# Copyright (C) 2014-2014 NQRustBackup GmbH & Co. KG
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
#
# Author: Kozlov Alexander
#

# Provided by the NQRustBackup Dir Python plugin interface
import nqrustbackup_dir_consts

# This module contains the wrapper functions called by the NQRustBackup-Dir, the
# functions call the corresponding methods from your plugin class
import NQRustBackupDirWrapper
from NQRustBackupDirWrapper import *

# This module contains the used plugin class
import NQRustBackupDirPluginGraphiteSender


def load_nqrustbackup_plugin(context, plugindef):
    '''
    This function is called by the NQRustBackup-Dir to load the plugin
    We use it to instantiate the plugin class
    '''
    # NQRustBackupDirWrapper.nqrustbackup_dir_plugin_object is the module attribute that
    # holds the plugin class object
    NQRustBackupDirWrapper.nqrustbackup_dir_plugin_object = \
        NQRustBackupDirPluginGraphiteSender.NQRustBackupDirPluginGraphiteSender(
            context, plugindef)
    return nqrustbackup_dir_consts.bRCs['bRC_OK']

# the rest is done in the Plugin module
