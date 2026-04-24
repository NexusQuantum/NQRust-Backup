# NQRUSTBACKUP - Backup Archiving REcovery Open Sourced
#
# Copyright (C) 2019-2023 NQRustBackup GmbH & Co. KG
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
# Author: Tobias Plum
#
import nqrustbackupdir
import NQRustBackupDirPluginBaseclass

from time import time
from sys import version_info


class NQRustBackupDirTest(NQRustBackupDirPluginBaseclass.NQRustBackupDirPluginBaseclass):
    def __init__(self, plugindef):
        nqrustbackupdir.DebugMessage(
            100,
            "Constructor called in module %s with plugindef=%s\n"
            % (__name__, plugindef),
        )
        nqrustbackupdir.DebugMessage(
            100,
            "Python Version: %s.%s.%s\n"
            % (version_info.major, version_info.minor, version_info.micro),
        )
        super(NQRustBackupDirTest, self).__init__(plugindef)

        self.outputfile = None

    def parse_plugin_definition(self, plugindef):
        super(NQRustBackupDirTest, self).parse_plugin_definition(plugindef)
        if "output" in self.options:
            self.outputfile = self.options["output"]
        else:
            self.outputfile = "/tmp/nqrustbackup-dir-test-plugin.log"

        return nqrustbackupdir.bRC_OK

    def handle_plugin_event(self, event):
        super(NQRustBackupDirTest, self).handle_plugin_event(event)
        job_name = repr(nqrustbackupdir.GetValue(nqrustbackupdir.bDirVarJobName))
        job_id = repr(nqrustbackupdir.GetValue(nqrustbackupdir.bDirVarJobId))
        microtime = round(time() * 1000)
        msg_f = (
            "%s Job:"
            + job_name
            + " JobId: "
            + job_id
            + " Time: "
            + repr(microtime)
            + "\n"
        )

        if event == nqrustbackupdir.bDirEventJobStart:
            self.toFile(msg_f % "bDirEventJobStart")

        elif event == nqrustbackupdir.bDirEventJobEnd:
            self.toFile(msg_f % "bDirEventJobEnd")

        elif event == nqrustbackupdir.bDirEventJobInit:
            self.toFile(msg_f % "bDirEventJobInit")

        elif event == nqrustbackupdir.bDirEventJobRun:
            self.toFile(msg_f % "bDirEventJobRun")

        return nqrustbackupdir.bRC_OK

    def toFile(self, text):
        nqrustbackupdir.DebugMessage(
            100,
            "Writing string '%s' to '%s'\n" % (text, self.outputfile),
        )
        doc = open(self.outputfile, "a")
        doc.write(text)
        doc.close()


# vim: ts=4 tabstop=4 expandtab shiftwidth=4 softtabstop=4
