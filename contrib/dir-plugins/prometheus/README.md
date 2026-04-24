This plugin implements a exporter for NQRustBackup jobs and pushs it to a Prometheus Pushgateway

## Design decision and Concept
The idea behind this plugin is to submit metrics about every finished job to a prometheus server to have the data in a timeseries database.
For big setups it is good practice not to crunch big SQL statements on the production catalog but get these information from an independent  
time series database.

Since the backup metrics are available for the plugin after after a job is finished it will then be sent to a Prometheus Pushgateway instantly
from which Prometheus can scrap it anytime. Therefore no additional interaction with the NQRustBackup director or catalog database is nessesary.

An alternative design would be to write these values to a textfile and use the textfile collector of the [node_exporter](https://github.com/prometheus/node_exporter).

## Dependencies

* You need a prometheus push gateway
  https://github.com/prometheus/pushgateway
* and the Python library to talk to it
  https://github.com/prometheus/client_python
* Of course the NQRustBackup Python library is needed
  https://github.com/nqrustbackup/nqrustbackup/tree/master/python-nqrustbackup/

See requirements.txt for python requirements.

## Usage

In nqrustbackup-dir.conf enable director plugins and load the Python plugin:

    Director {
      Plugin Directory = /usr/lib/nqrustbackup/plugins
      Plugin Names = "python3"
    }

In your JobDefs or Job Definition configure the plugin itself:

    Job {
      Name = "BackupClient1"
      DIR Plugin Options ="python3:module_path=/usr/lib/nqrustbackup/plugins:module_name=nqrustbackup-dir-prometheus-exporter:gateway_host=pushgateway.domain.tld:gateway_port=443:use_tls=yes:username=monitoring:password=foobar"
      JobDefs = "DefaultJob"
    }

## Available parameters
* `gateway_host` (default=localhost): Where the Prometheus pushgateway is reachable
* `gateway_port` (default=9091): TCP port on which the Prometheus pushgateway is reachable
* `username` & `password`: For HTTP Basic Authentication. Both or none must be set to enable/disable the usage of BasicAuth
* `use_tls` (default=false): Whether TLS encryption should be used to communication with the pushgateway
* `report_failed` (default=false): Whether failed jobs should be reported or not

## exported Prometheus metrics

* nqrustbackup_job_status
* nqrustbackup_job_running_time
* nqrustbackup_job_files
* nqrustbackup_job_bytes
* nqrustbackup_job_throughput
* nqrustbackup_job_priority
* nqrustbackup_job_level
* nqrustbackup_job_type
* nqrustbackup_job_client
