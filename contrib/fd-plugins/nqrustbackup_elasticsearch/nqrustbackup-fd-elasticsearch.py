#!/usr/bin/env python
# -*- coding: utf-8 -*-
# nqrustbackup-fd-elasticsearch.py parses any file in the backup job and sends metadata and (optionally) contents
# to an Elasticsearch server.

# Provided by the NQRustBackup FD Python plugin interface
from nqrustbackupfd import *
from nqrustbackup_fd_consts import *

# This module contains the wrapper functions called by the NQRustBackup-FD, the functions call the corresponding
# methods from your plugin class
from NQRustBackupFdWrapper import *

# This module contains the used plugin class
from NQRustBackupFdPluginElasticsearch import *

def load_nqrustbackup_plugin(context, plugindef):
    '''
    This function is called by the NQRustBackup-FD to load the plugin
    We use it to intantiate the plugin class
    '''
    # NQRustBackupFdWrapper.nqrustbackup_fd_plugin_object is the module attribute that holds the plugin class object
    NQRustBackupFdWrapper.nqrustbackup_fd_plugin_object = NQRustBackupFdPluginFileElasticsearch (context, plugindef);
    return bRCs['bRC_OK'];

# the rest is done in the Plugin module
