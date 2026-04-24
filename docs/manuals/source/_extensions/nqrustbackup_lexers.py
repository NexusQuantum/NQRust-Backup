#   NQRUSTBACKUP - Backup Archiving REcovery Open Sourced
#
#   Copyright (C) 2019-2022 NQRustBackup GmbH & Co. KG
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

from pygments.lexer import RegexLexer, inherit, bygroups
from pygments.lexers.shell import BashLexer, BashSessionLexer
from pygments.token import *
from nqrustbackup_urls import NQRustBackupUrls

#
# http://pygments.org/docs/lexerdevelopment/
# http://pygments.org/docs/tokens/
#

# Test:
#  cat consolidate.cfg | pygmentize -l _extensions/nqrustbackup_lexers.py:NQRustBackupConfigLexer -x
#


class NQRustBackupBaseLexer(BashLexer):
    name = "NQRustBackupBase"

    def downloadurl_callback(lexer, match):
        yield match.start(), Generic.Headline, NQRustBackupUrls().get_download_nqrustbackup_org_url()

    tokens = {
        "root": [
            # (r'(<input>)(.*)(</input>)', bygroups(None, Generic.Emph, None)),
            # (r'(<input>)(.*)(</input>)', bygroups(None, Generic.Strong, None)),
            (r"(<input>)(.*)(</input>)", bygroups(None, Generic.Heading, None)),
            (r"(<strong>)(.*)(</strong>)", bygroups(None, Generic.Strong, None)),
            (r"(<downloadurl>)", downloadurl_callback),
            inherit,
        ]
    }


class NQRustBackupShellLexer(NQRustBackupBaseLexer):
    name = "ShellNQRustBackup"
    aliases = ["sh"]

    tokens = {"root": [inherit]}


class NQRustBackupConfigLexer(NQRustBackupBaseLexer):
    name = "NQRustBackupConfig"
    aliases = ["nqrustbackupconfig", "bconfig"]
    filenames = ["*.cfg"]

    tokens = {"root": [inherit]}


class NQRustBackupConsoleLexer(NQRustBackupBaseLexer):
    name = "NQRustBackupConsole"
    aliases = ["nqrustbackupconsole", "nqrustbackup_console"]
    # filenames = ['*.cfg']

    tokens = {"root": [inherit]}


class NQRustBackupLogLexer(NQRustBackupBaseLexer):
    name = "NQRustBackupLog"
    aliases = ["nqrustbackuplog"]
    filenames = ["*.log"]

    tokens = {"root": [inherit]}


class NQRustBackupMessageLexer(NQRustBackupBaseLexer):
    name = "NQRustBackupMessage"
    aliases = ["nqrustbackupmessage", "bmessage"]
    # filenames = ['*.log']

    tokens = {"root": [inherit]}


class NQRustBackupShellSessionLexer(BashSessionLexer):
    name = "NQRustBackupShellSession"
    aliases = ["shell-session"]

    def get_tokens_unprocessed(self, text):
        url = NQRustBackupUrls().get_download_nqrustbackup_org_url()
        text = text.replace("<downloadurl>", url)
        return super(NQRustBackupShellSessionLexer, self).get_tokens_unprocessed(text)
