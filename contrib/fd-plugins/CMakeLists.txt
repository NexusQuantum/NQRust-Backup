#   NQRUSTBACKUP® - Backup Archiving REcovery Open Sourced
#
#   Copyright (C) 2021-2025 NQRustBackup GmbH & Co. KG
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

# nqrustbackup_elasticsearch is only a PoC
#install(DIRECTORY nqrustbackup_elasticsearch DESTINATION ${PYTHON_MODULE_PATH} COMPONENT filedaemon)
install(DIRECTORY nqrustbackup_mysql_dump DESTINATION ${PYTHON_MODULE_PATH} COMPONENT filedaemon)
install(DIRECTORY mariadb-dump DESTINATION ${PYTHON_MODULE_PATH} COMPONENT filedaemon)
# nqrustbackup_option_example is only an example
#install(DIRECTORY nqrustbackup_option_example DESTINATION ${PYTHON_MODULE_PATH} COMPONENT filedaemon)
install(DIRECTORY nqrustbackup_tasks DESTINATION ${PYTHON_MODULE_PATH} COMPONENT filedaemon)
install(DIRECTORY openvz7 DESTINATION ${PYTHON_MODULE_PATH} COMPONENT filedaemon)
