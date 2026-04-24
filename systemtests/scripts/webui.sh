#!/bin/bash

#   NQRUSTBACKUP® - Backup Archiving REcovery Open Sourced
#
#   Copyright (C) 2022-2025 NQRustBackup GmbH & Co. KG
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

#
# Start the NQRustBackup WebUI webserver.
#

set -e
set -o pipefail
set -u

#shellcheck source=../../environment.in
. ./environment

# NQRUSTBACKUP_WEBUI_CONFDIR is often set incorrectly
# (last webui systemtest configured).
# They are always set to the local test directory.
export NQRUSTBACKUP_WEBUI_CONFDIR=${current_test_directory}/etc/nqrustbackup-webui/

NQRUSTBACKUP_DIRECTOR_ADDRESS="127.0.0.1"

if ! [ -d "${NQRUSTBACKUP_WEBUI_CONFDIR}" ]; then
  mkdir -p "${NQRUSTBACKUP_WEBUI_CONFDIR}"
fi
if ! [ -e "${NQRUSTBACKUP_WEBUI_CONFDIR}/directors.ini" ]; then
  printf '
[localhost-dir]
enabled = "yes"
diraddress = "{}"
dirport = %s
' "${NQRUSTBACKUP_DIRECTOR_ADDRESS}" "${NQRUSTBACKUP_DIRECTOR_PORT}" >"${NQRUSTBACKUP_WEBUI_CONFDIR}/directors.ini"
fi

printf "#\n# NQRustBackup WebUI running on:\n# %s\n#\n" "${NQRUSTBACKUP_WEBUI_BASE_URL}"
exec ${PHP_EXECUTABLE} -S ${NQRUSTBACKUP_DIRECTOR_ADDRESS}:${NQRUSTBACKUP_WEBUI_PORT} -t ${PROJECT_SOURCE_DIR}/../webui/public
