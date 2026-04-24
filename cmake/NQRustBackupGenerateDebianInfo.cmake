#   NQRUSTBACKUP® - Backup Archiving REcovery Open Sourced
#
#   Copyright (C) 2018-2025 NQRustBackup GmbH & Co. KG
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

if(MSVC)
  return()
endif()

# always add "src" package snippet
set(DEBIAN_CONTROL_SNIPPETS "src")

if(BUILD_UNIVERSAL_CLIENT)
  list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-universal-client")
  list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-universal-client-dbg")
else()
  if(NOT client-only)
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup")
  endif()
  if(ENABLE_NQRUSTBACKUP_CONSOLE)
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-nqrustbackup_console")
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-client")
  endif()
  list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-common")
  if(NOT client-only)
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-database")
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-dbg")
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-director")
  endif()
  list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-filedaemon")
  if(NOT client-only)
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-storage")
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-tools")
  endif()

  if(NOT client-only
     AND ENABLE_PYTHON
     AND (Python3_FOUND)
  )
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-director-python3-plugin")
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-director-python-plugins-common")
  endif()

  if(HAVE_GFAPI)
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-filedaemon-glusterfs-plugin")
  endif()
  if(ENABLE_PYTHON AND Python3_FOUND)
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-filedaemon-python3-plugin")
    list(APPEND DEBIAN_CONTROL_SNIPPETS
         "nqrustbackup-filedaemon-python-plugins-common"
    )
  endif()

  if(ENABLE_GRPC)
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-filedaemon-grpc-python3-plugin")
  endif()
  if(NOT client-only AND TARGET droplet)
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-storage-droplet")
  endif()
  if(NOT client-only AND HAVE_GFAPI)
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-storage-glusterfs")
  endif()
  if(NOT client-only
     AND ENABLE_PYTHON
     AND Python3_FOUND
  )
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-storage-python3-plugin")
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-storage-python-plugins-common")
  endif()

  if(traymonitor)
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-traymonitor")
  endif()

  if(${NQRUSTBACKUP_PLATFORM} MATCHES "univention")
    list(APPEND DEBIAN_CONTROL_SNIPPETS "univention-nqrustbackup")
  endif()

  if(NOT client-only)
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-webui")
  endif()

  if(NOT client-only AND VIXDISKLIB_FOUND)
    list(APPEND DEBIAN_CONTROL_SNIPPETS "vmware")
  endif()

  if(NOT client-only)
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-contrib-tools")
  endif()

  if(ENABLE_BARRI)
    list(APPEND DEBIAN_CONTROL_SNIPPETS
         "nqrustbackup-filedaemon-barri-python3-plugin"
    )
    list(APPEND DEBIAN_CONTROL_SNIPPETS "nqrustbackup-barri-cli")
  endif()

  if(ENABLE_PROXMOX_PLUGIN)
    list(APPEND DEBIAN_CONTROL_SNIPPETS "proxmox")
  endif()

endif()

include(NQRustBackupConfigureFile)
nqrustbackup_configure_file(GLOB "${CMAKE_SOURCE_DIR}/debian/*.in")

file(GLOB templated_file_path_list "${CMAKE_BINARY_DIR}/debian/*")
set(DEBIAN_TEMPLATED_FILE_LIST "")
foreach(templated_file_path ${templated_file_path_list})
  get_filename_component(templated_file ${templated_file_path} NAME)
  list(APPEND DEBIAN_TEMPLATED_FILE_LIST "${templated_file}")
endforeach()

configure_file(
  ${CMAKE_SOURCE_DIR}/core/cmake/generate-debian-control.cmake.in
  ${CMAKE_BINARY_DIR}/generate-debian-control.cmake @ONLY
)

add_custom_target(
  generate-debian-control
  COMMAND ${CMAKE_COMMAND} -P ${CMAKE_BINARY_DIR}/generate-debian-control.cmake
  WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}/debian"
)
