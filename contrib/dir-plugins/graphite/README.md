This plugin implements a statistics sender to Graphite.

Usage

In nqrustbackup-dir.conf enable director plugins and load the Python plugin:

    Director {
      Plugin Directory = /usr/lib64/nqrustbackup/plugins
      Plugin Names = "python"
      # ...
    }

In your JobDefs or Job Definition configure the plugin itself:

    Job {
      Name = "BackupClient1"
      DIR Plugin Options ="python:module_path=/usr/lib64/nqrustbackup/plugins:module_name=nqrustbackup-dir-graphite-sender:collectorHost=graphite:collectorPort=2003:metricPrefix=app"
      JobDefs = "DefaultJob"
    }

* collectorHost (default graphite): IP our resolvable address of your graphite host
* collectorPort (default 2003): graphite server port
* metricPrefix (default apps) : prefix, added to all metric names

Metrics

* <metricPrefix>.nqrustbackup.jobs.<jobName>.status.(error|warning|success)
* <metricPrefix>.nqrustbackup.jobs.<jobName>.jobbytes
* <metricPrefix>.nqrustbackup.jobs.<jobName>.jobfiles
* <metricPrefix>.nqrustbackup.jobs.<jobName>.runningtime
* <metricPrefix>.nqrustbackup.jobs.<jobName>.throughput
