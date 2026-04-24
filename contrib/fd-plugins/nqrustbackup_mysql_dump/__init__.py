#!/usr/bin/env python
# -*- coding: utf-8 -*-
# NQRustBackup-fd-local-fileset a simple example for a python NQRustBackup FD Plugin using NQRustBackupFdPluginLocalFileset
# The plugin argument 'filename' is used to read all files listed in that file and add it to the fileset
# License: AGPLv3

# Provided by the NQRustBackup FD Python plugin interface
import nqrustbackupfd

# This module contains the wrapper functions called by the NQRustBackup-FD, the
# functions call the corresponding methods from your plugin class
import NQRustBackupFdWrapper
from NQRustBackupFdWrapper import *

# This module contains the used plugin class
from nqrustbackup_mysql_dump.NQRustBackupFdMySQLclass import NQRustBackupFdMySQLclass

def load_nqrustbackup_plugin(plugindef):
    '''
    This function is called by the NQRustBackup-FD to load the plugin
    We use it to instantiate the plugin class
    '''
    # NQRustBackupFdWrapper.nqrustbackup_fd_plugin_object is the module attribute that holds the plugin class object
    NQRustBackupFdWrapper.nqrustbackup_fd_plugin_object = NQRustBackupFdMySQLclass(plugindef)
    return nqrustbackupfd.bRC_OK

# the rest is done in the Plugin module
