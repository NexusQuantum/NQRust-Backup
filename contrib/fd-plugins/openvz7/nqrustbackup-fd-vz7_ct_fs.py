# Provided by the NQRustBackup FD Python plugin interface
import nqrustbackupfd

# This module contains the wrapper functions called by the NQRustBackup-FD, the
# functions call the corresponding methods from your plugin class
import NQRustBackupFdWrapper
# from NQRustBackupFdWrapper import parse_plugin_definition, handle_plugin_event, start_backup_file, end_backup_file, start_restore_file, end_restore_file, restore_object_data, plugin_io, create_file, check_file, handle_backup_file  # noqa
from NQRustBackupFdWrapper import *  # noqa

# This module contains the used plugin class
import NQRustBackupFdPluginVz7CtFs


def load_nqrustbackup_plugin(plugindef):
    '''
    This function is called by the NQRustBackup-FD to load the plugin
    We use it to instantiate the plugin class
    '''
    # NQRustBackupFdWrapper.nqrustbackup_fd_plugin_object is the module attribute that
    # holds the plugin class object
    NQRustBackupFdWrapper.nqrustbackup_fd_plugin_object = \
        NQRustBackupFdPluginVz7CtFs.NQRustBackupFdPluginVz7CtFs(
            plugindef)
    return nqrustbackupfd.bRC_OK
