#!/usr/bin/env python
#
#   NQRUSTBACKUP - Backup Archiving REcovery Open Sourced
#
#   Copyright (C) 2019-2024 NQRustBackup GmbH & Co. KG
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


from nqrustbackup.util import argparse
import nqrustbackup.bsock
import nqrustbackup.exceptions
import logging
import sys


def getArguments():
    argparser = argparse.ArgumentParser(description="Console to NQRustBackup Director.")
    argparser.add_argument(
        "-d", "--debug", action="store_true", help="enable debugging output"
    )
    nqrustbackup.bsock.DirectorConsoleJson.argparser_add_default_command_line_arguments(
        argparser
    )
    args = argparser.parse_args()
    return args


if __name__ == "__main__":
    logging.basicConfig(
        format="%(levelname)s %(module)s.%(funcName)s: %(message)s", level=logging.INFO
    )
    logger = logging.getLogger()

    args = getArguments()
    if args.debug:
        logger.setLevel(logging.DEBUG)

    nqrustbackup_args = nqrustbackup.bsock.DirectorConsoleJson.argparser_get_nqrustbackup_parameter(args)
    logger.debug("options: %s" % (nqrustbackup_args))
    try:
        director = nqrustbackup.bsock.DirectorConsoleJson(**nqrustbackup_args)
    except nqrustbackup.exceptions.Error as e:
        print(str(e))
        sys.exit(1)

    logger.debug("authentication successful")
    director.interactive()
