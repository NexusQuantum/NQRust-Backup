NQRustBackup - Graphite - Feeder

Use nqrustbackup-graphite-poller.py to read performance data from NQRustBackup and feed it into a Graphite backend.

# Configuration

The default configuration file is /etc/nqrustbackup/graphite-poller.conf, which can be adjusted using -c when calling the poller.
Here you configure NQRustBackup director access and the Graphite backend.

```
[director]
server=fqhn
name=your-director-name
password=secret

[graphite]
server=localhost
port=2003
```

# Usage

Output from help:

```
usage: nqrustbackup-graphite-poller.py [-h] [-d {error,warning,info,debug}]
                                 [-c CONFIG] [-e] [-j] [-v] [-s START] [-t TO]

Graphite poller for NQRustBackup director.

optional arguments:
  -h, --help            show this help message and exit
  -d {error,warning,info,debug}, --debug {error,warning,info,debug}
                        Set debug level to {error,warning,info,debug}
  -c CONFIG, --config CONFIG
                        Configfile
  -e, --events          Import Events (JobStart / JobEnd)
  -j, --jobstats        Import jobstats
  -v, --devicestats     Import devicestats
  -s START, --start START
                        Import entries after this timestamp (only used for
                        stats and events)
  -t TO, --to TO        Import entries before this timestamp (only used for
                        stats and events)
```

If you use `-s last` a timestamp file will be created, unique for the options used. On the first call, no data will be imported, on all following calls
with this options, all data since the last call will read and transferred. This is suitable to use as cronjob.

Example call:
`nqrustbackup-graphite-poller.py -c /etc/nqrustbackup/graphite-poller-jenkins.conf -s "last" -e -j -v`

Which will call the poller, using the given configuration file, using timestamps and also process events, jobstats and devicestats.

# Details and options

By default some standard metrics will be queried, like number of running jobs, total bytes in backup and statistics per pool, job and clients.

If you are using enhanced device- and job- statistics from NQRustBackup, these can be queried by -v and -j. If you specify -e for events, all timestamps
regarding job-start and job-end are submitted as events to Graphite.

See http://doc.nqrustbackup.org for more information to enable statistic gathering in NQRustBackup.

# Metrics

The metrics in Grahite will have the prefix nqrustbackup.*your director name*.[clients|devices|jobs|pools]. 


 
