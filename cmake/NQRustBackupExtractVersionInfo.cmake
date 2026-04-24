# NQRUSTBACKUP® - Backup Archiving REcovery Open Sourced
#
# Copyright (C) 2017-2023 NQRustBackup GmbH & Co. KG
#
# This program is Free Software; you can redistribute it and/or modify it under
# the terms of version three of the GNU Affero General Public License as
# published by the Free Software Foundation and included in the file LICENSE.
#
# This program is distributed in the hope that it will be useful, but WITHOUT
# ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
# FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
# details.
#
# You should have received a copy of the GNU Affero General Public License along
# with this program; if not, write to the Free Software Foundation, Inc., 51
# Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.

include(NQRustBackupVersion OPTIONAL RESULT_VARIABLE NQRustBackupVersionFile)
if(NQRustBackupVersionFile STREQUAL "NOTFOUND")
  # no version file, try data from git
  if(GIT_DESCRIBE_VERSION)
    message(STATUS "Using version information from Git")
    if(DEFINED VERSION_STRING)
      message(
        STATUS
          "VERSION_STRING already set to ${VERSION_STRING}. Will not overwrite"
      )
    else()
      set(VERSION_STRING "${GIT_DESCRIBE_VERSION}")
    endif()
    set(VERSION_TIMESTAMP "${GIT_COMMIT_TIMESTAMP}")
  else()
    message(
      FATAL_ERROR
        "VERSION_STRING not set, NQRustBackupVersion.cmake not found and no version data from git available.\n"
        "For more information why this happened and how to fix it, please see "
        "https://docs.nqrustbackup.org/DeveloperGuide/AutomaticVersionGeneration.html#troubleshooting"
    )
  endif()
else()
  message(STATUS "Using NQRust Backup version information")
endif()

string(REGEX MATCH [0-9.a-zA-Z~]+ NQRUSTBACKUP_FULL_VERSION ${VERSION_STRING})

if(NQRUSTBACKUP_FULL_VERSION STREQUAL "")
  message(FATAL_ERROR "NQRUSTBACKUP_FULL_VERSION is not set")
endif()

# set NQRUSTBACKUP_FULL_VERSION in parent scope if there is a parent scope
get_directory_property(hasParent PARENT_DIRECTORY)
if(hasParent)
  set(NQRUSTBACKUP_FULL_VERSION
      ${NQRUSTBACKUP_FULL_VERSION}
      PARENT_SCOPE
  )
endif()

string(REGEX MATCH [0-9]+.[0-9]+.[0-9]+ NQRUSTBACKUP_NUMERIC_VERSION
             ${VERSION_STRING}
)

string(REPLACE "." ";" VERSION_LIST ${NQRUSTBACKUP_NUMERIC_VERSION})
list(GET VERSION_LIST 0 NQRUSTBACKUP_VERSION_MAJOR)
list(GET VERSION_LIST 1 NQRUSTBACKUP_VERSION_MINOR)
list(GET VERSION_LIST 2 NQRUSTBACKUP_VERSION_PATCH)

message("NQRUSTBACKUP_NUMERIC_VERSION is ${NQRUSTBACKUP_NUMERIC_VERSION}")
message("NQRUSTBACKUP_FULL_VERSION is ${NQRUSTBACKUP_FULL_VERSION}")

if(VERSION_TIMESTAMP GREATER 0)
  if(DEFINED ENV{SOURCE_DATE_EPOCH})
    set(_old_source_date_epoch "$ENV{SOURCE_DATE_EPOCH}")
  endif()
  set(ENV{SOURCE_DATE_EPOCH} "${VERSION_TIMESTAMP}")
  string(TIMESTAMP DATE "%d %B %Y" UTC)
  string(TIMESTAMP NQRUSTBACKUP_SHORT_DATE "%d%b%y" UTC)
  string(TIMESTAMP NQRUSTBACKUP_YEAR "%Y" UTC)
  string(TIMESTAMP NQRUSTBACKUP_PROG_DATE_TIME "%Y-%m-%d %H:%M:%S" UTC)
  if(DEFINED _old_source_date_epoch)
    set(ENV{SOURCE_DATE_EPOCH} "${_old_source_date_epoch}")
    unset(_old_source_date_epoch)
  else()
    unset(ENV{SOURCE_DATE_EPOCH})
  endif()
else()
  message(FATAL_ERROR "VERSION_TIMESTAMP is not set")
endif()

# extract  db version from cats.h
file(STRINGS ${PROJECT_SOURCE_DIR}/core/src/cats/cats.h DB_VERSION_STRING
     REGEX .*BDB_VERSION.*
)
string(REGEX MATCH [0-9]+ BDB_VERSION ${DB_VERSION_STRING})
