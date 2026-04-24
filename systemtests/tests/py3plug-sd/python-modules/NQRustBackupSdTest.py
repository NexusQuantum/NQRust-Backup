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
import nqrustbackupsd
import NQRustBackupSdPluginBaseclass

from sys import version_info


class NQRustBackupSdTest(NQRustBackupSdPluginBaseclass.NQRustBackupSdPluginBaseclass):
    def __init__(self, plugindef):
        nqrustbackupsd.DebugMessage(
            100,
            "Constructor called in module %s with plugindef=%s\n"
            % (__name__, plugindef),
        )
        nqrustbackupsd.DebugMessage(
            100,
            "Python Version: %s.%s.%s\n"
            % (version_info.major, version_info.minor, version_info.micro),
        )
        super(NQRustBackupSdTest, self).__init__(plugindef)

        self.outputfile = None

        events = []
        events.append(nqrustbackupsd.bsdEventJobStart)
        events.append(nqrustbackupsd.bsdEventDeviceReserve)
        events.append(nqrustbackupsd.bsdEventVolumeUnload)
        events.append(nqrustbackupsd.bsdEventVolumeLoad)
        events.append(nqrustbackupsd.bsdEventDeviceOpen)
        events.append(nqrustbackupsd.bsdEventDeviceMount)
        events.append(nqrustbackupsd.bsdEventLabelRead)
        events.append(nqrustbackupsd.bsdEventLabelVerified)
        events.append(nqrustbackupsd.bsdEventLabelWrite)
        events.append(nqrustbackupsd.bsdEventSetupRecordTranslation)
        events.append(nqrustbackupsd.bsdEventWriteRecordTranslation)
        events.append(nqrustbackupsd.bsdEventDeviceUnmount)
        events.append(nqrustbackupsd.bsdEventDeviceClose)
        events.append(nqrustbackupsd.bsdEventJobEnd)
        nqrustbackupsd.RegisterEvents(events)

    def parse_plugin_definition(self, plugindef):
        super(NQRustBackupSdTest, self).parse_plugin_definition(plugindef)
        if "output" in self.options:
            self.outputfile = self.options["output"]
        else:
            self.outputfile = "/tmp/nqrustbackup-dir-test-plugin.log"

        return nqrustbackupsd.bRC_OK

    def handle_plugin_event(self, event):
        super(NQRustBackupSdTest, self).handle_plugin_event(event)
        nqrustbackupsd.DebugMessage(
            100,
            "%s: bsdEventJobStart event %d triggered\n" % (__name__, event),
        )
        if event == nqrustbackupsd.bsdEventJobStart:
            self.toFile("nqrustbackupsd.bsdEventJobStart\n")
        elif event == nqrustbackupsd.bsdEventDeviceReserve:
            self.toFile("nqrustbackupsd.bsdEventDeviceReserve\n")
        elif event == nqrustbackupsd.bsdEventVolumeUnload:
            self.toFile("nqrustbackupsd.bsdEventVolumeUnload\n")
        elif event == nqrustbackupsd.bsdEventVolumeLoad:
            self.toFile("nqrustbackupsd.bsdEventVolumeLoad\n")
        elif event == nqrustbackupsd.bsdEventDeviceOpen:
            self.toFile("nqrustbackupsd.bsdEventDeviceOpen\n")
        elif event == nqrustbackupsd.bsdEventDeviceMount:
            self.toFile("nqrustbackupsd.bsdEventDeviceMount\n")
        elif event == nqrustbackupsd.bsdEventLabelRead:
            self.toFile("nqrustbackupsd.bsdEventLabelRead\n")
        elif event == nqrustbackupsd.bsdEventLabelVerified:
            self.toFile("nqrustbackupsd.bsdEventLabelVerified\n")
        elif event == nqrustbackupsd.bsdEventLabelWrite:
            self.toFile("nqrustbackupsd.bsdEventLabelWrite\n")
        elif event == nqrustbackupsd.bsdEventSetupRecordTranslation:
            self.toFile("nqrustbackupsd.bsdEventSetupRecordTranslation\n")
        elif event == nqrustbackupsd.bsdEventWriteRecordTranslation:
            self.toFile("nqrustbackupsd.bsdEventWriteRecordTranslation\n")
        elif event == nqrustbackupsd.bsdEventDeviceUnmount:
            self.toFile("nqrustbackupsd.bsdEventDeviceUnmount\n")
        elif event == nqrustbackupsd.bsdEventDeviceClose:
            self.toFile("nqrustbackupsd.bsdEventDeviceClose\n")
        elif event == nqrustbackupsd.bsdEventJobEnd:
            self.toFile("nqrustbackupsd.bsdEventJobEnd\n")

        return nqrustbackupsd.bRC_OK

    def toFile(self, text):
        doc = open(self.outputfile, "a")
        doc.write(text)
        doc.close()


# vim: ts=4 tabstop=4 expandtab shiftwidth=4 softtabstop=4
