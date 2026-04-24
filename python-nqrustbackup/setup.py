#!/usr/bin/python
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


import os
import re
from setuptools import find_packages, setup


def get_version():
    base_dir = os.path.abspath(os.path.dirname(__file__))

    try:
        with open(os.path.join(base_dir, "nqrustbackup", "VERSION.txt")) as version_file:
            # read version
            # and adapt it according to
            # https://www.python.org/dev/peps/pep-0440/.
            # Local version identifiers
            # <public version identifier>[+<local version label>]
            # are not supported by PyPI and therefore not included.
            fullversion = version_file.read().strip()
            __version__ = re.sub(r"~pre([0-9]+)\..*", r".dev\1", fullversion)
    except IOError:
        # Fallback version.
        # First protocol implemented
        # has been introduced with this version.
        __version__ = "18.2.5"

    return __version__


def get_long_description():
    base_dir = os.path.abspath(os.path.dirname(__file__))
    with open("README.rst") as readme_file:
        description = readme_file.read()
    with open(os.path.join(base_dir, "nqrustbackup", "bsock", "__init__.py")) as bsock_file:
        bsock_content = bsock_file.read().strip()
    # bsock_description = re.sub(r'.*"""(.*?)""".*', r'\1', bsock_content, flags=re.DOTALL)
    bsock_description = re.sub(
        ":py:class:",
        "",
        re.sub(r'.*(\.\. note::.*?)""".*', r"\n\n\1", bsock_content, flags=re.DOTALL),
    )
    return description + bsock_description


setup(
    name="python-nqrustbackup",
    version=get_version(),
    license="AGPLv3",
    author="NQRustBackup Team",
    author_email="packager@nqrustbackup.com",
    packages=find_packages(),
    scripts=["bin/nqrustbackup_console.py", "bin/nqrustbackup_console-json.py", "bin/nqrustbackup-fd-connect.py"],
    package_data={"nqrustbackup": ["VERSION.txt"]},
    url="https://github.com/nqrustbackup/nqrustbackup/",
    # What does your project relate to?
    keywords="nqrustbackup",
    description="Client library and tools for NQRustBackup console access.",
    long_description=get_long_description(),
    long_description_content_type="text/x-rst",
    # RHEL7: python-3.6
    python_requires=">=3.6",
    extras_require={"configfile": ["configargparse"]},
    classifiers=[
        "Development Status :: 5 - Production/Stable",
        "License :: OSI Approved :: GNU Affero General Public License v3",
        "Operating System :: OS Independent",
        "Programming Language :: Python",
        "Topic :: System :: Archiving :: Backup",
    ],
)
