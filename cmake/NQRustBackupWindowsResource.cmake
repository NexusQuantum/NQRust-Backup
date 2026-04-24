#   NQRUSTBACKUP® - Backup Archiving REcovery Open Sourced
#
#   Copyright (C) 2025-2025 NQRustBackup GmbH & Co. KG
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

function(nqrustbackup_windows_resource target)
  if(HAVE_WIN32)
    nqrustbackup_configure_file(RESULT_VAR file_list FILES ${ARGN})
    target_sources(${target} PRIVATE ${file_list})
  endif()
endfunction()
