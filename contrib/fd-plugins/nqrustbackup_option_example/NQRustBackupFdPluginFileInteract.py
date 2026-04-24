#!/usr/bin/env python

# -*- coding: utf-8 -*-

# NQRustBackup python class as stub for option plugin
# handle_backup_file gets called for each file in the backup
# 
# (c) NQRustBackup GmbH & Co. KG
# AGPL v.3
# Author: Maik Aussendorf

import nqrustbackupfd
from NQRustBackupFdPluginBaseclass import NQRustBackupFdPluginBaseclass

class NQRustBackupFdPluginFileInteract(NQRustBackupFdPluginBaseclass):

    def __init__(self, plugindef):
        super(NQRustBackupFdPluginFileInteract, self).__init__(plugindef)
        nqrustbackupfd.RegisterEvents([nqrustbackupfd.bEventHandleBackupFile])

    def handle_backup_file(self, savepkt):
        nqrustbackupfd.DebugMessage(100, "handle_backup_file called with " + str(savepkt) + "\n");
        nqrustbackupfd.DebugMessage(100, "fname: " + savepkt.fname + " Type: " + str(savepkt.type) + "\n");
        if (savepkt.type == nqrustbackupfd.FT_REG):
            nqrustbackupfd.DebugMessage(100, "regular file, do something now...\n");
            # Add your stuff here.

        return nqrustbackupfd.bRC_OK
