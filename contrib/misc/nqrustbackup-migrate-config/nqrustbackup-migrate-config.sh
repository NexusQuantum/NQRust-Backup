#!/bin/bash

#
# Split existing configuration into separate resource files, as used by the
# http://doc.nqrustbackup.org/master/html/nqrustbackup-manual-main-reference.html#SubdirectoryConfigurationScheme
#
# Author: philipp.storz@nqrustbackup.com
#

NQRUSTBACKUP_CONSOLE=${NQRUSTBACKUP_CONSOLE:-/usr/bin/nqrustbackup_console}

nqrustbackup_console()
{
  local out="$1"
  local cmd="$2"
  local temp=`mktemp`
  printf "%s\n%s\n%s\n" "gui on" "@out $temp" "$cmd" | $NQRUSTBACKUP_CONSOLE > /dev/null
  # The send command is also written to the output. Remove it.
  # Error messages normally also contain the command, so they are also removed.
  grep -v -e "$cmd" "$temp" > "$out"
}

for restype in catalog client console counter director fileset job jobdefs messages pool profile schedule storage; do
    printf "\n%s:\n" "$restype"
    printf "==========\n"
    mkdir $restype 2>/dev/null
    if [ $restype = director ]; then
        nqrustbackup_console "$restype/nqrustbackup-dir.conf" "show director"
    else
        dotcommand=".${restype}"
        if [ $restype = "messages" ]; then
            dotcommand=".msgs"
        elif [ $restype = "job" ]; then
            dotcommand=".jobs"
        fi
        TEMP=`mktemp`
        nqrustbackup_console "$TEMP" "$dotcommand"
        cat $TEMP
        while read res; do
            #CONFFILENAME=`sed 's/ /_/g' <<< ${res}`
            CONFFILENAME="${res}"
            nqrustbackup_console "$restype/${CONFFILENAME}.conf" "show ${restype}=\"${res}\""
        done < $TEMP
    fi
done

