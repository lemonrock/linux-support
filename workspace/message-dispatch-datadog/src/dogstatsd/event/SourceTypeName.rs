// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// From <https://docs.datadoghq.com/integrations/faq/list-of-api-source-attribute-value/>.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SourceTypeName
{
	/// Activemq.
	ACTIVEMQ,
	
	/// Airbrake.
	AIRBRAKE,
	
	/// Akamai Datastream.
	AKAMAIDATASTREAM,
	
	/// Akamai Mpulse.
	AKAMAIMPULSE,
	
	/// Alibaba Cloud.
	ALIBABACLOUD,
	
	/// Amazon Api Gateway.
	APIGATEWAY,
	
	/// Amazon App Mesh.
	AMAZONAPPMESH,
	
	/// Amazon Appstream.
	APPSTREAM,
	
	/// Amazon Appsync.
	APPSYNC,
	
	/// Amazon Athena.
	ATHENA,
	
	/// Amazon Auto Scaling.
	AUTOSCALING,
	
	/// Amazon Billing.
	BILLING,
	
	/// Amazon Cloudfront.
	CLOUDFRONT,
	
	/// Amazon Cloudsearch.
	CLOUDSEARCH,
	
	/// Amazon Cloudtrail.
	CLOUDTRAIL,
	
	/// Amazon Codebuild.
	CODEBUILD,
	
	/// Amazon Codedeploy.
	CODEDEPLOY,
	
	/// Amazon Cognito.
	COGNITO,
	
	/// Amazon Connect.
	AWSCONNECT,
	
	/// Amazon Directconnect.
	DIRECTCONNECT,
	
	/// Amazon Dms.
	AWSDMS,
	
	/// Amazon Documentdb.
	DOCUMENTDB,
	
	/// Amazon Dynamodb.
	DYNAMODB,
	
	/// Amazon Ebs.
	EBS,
	
	/// Amazon Ec2.
	EC2,
	
	/// Amazon Ec2 Spot.
	EC2SPOT,
	
	/// Amazon Ecs.
	ECS,
	
	/// Amazon Efs.
	EFS,
	
	/// Amazon Elastic Transcoder.
	ELASTICTRANSCODER,
	
	/// Amazon Elasticache.
	ELASTICACHE,
	
	/// Amazon Elasticbeanstalk.
	ELASTICBEANSTALK,
	
	/// Amazon Elb.
	ELB,
	
	/// Amazon Emr.
	EMR,
	
	/// Amazon Es.
	ES,
	
	/// Amazon Eventbridge.
	AWSEVENTHUB,
	
	/// Amazon Firehose.
	FIREHOSE,
	
	/// Amazon Gamelift.
	GAMELIFT,
	
	/// Amazon Glue.
	GLUE,
	
	/// Amazon Health.
	HEALTH,
	
	/// Amazon Inspector.
	INSPECTOR,
	
	/// Amazon Iot.
	IOT,
	
	/// Amazon Kinesis.
	KINESIS,
	
	/// Amazon Kms.
	KMS,
	
	/// Amazon Lambda.
	LAMBDA,
	
	/// Amazon Lex.
	AWSLEX,
	
	/// Amazon Machine Learning.
	MACHINELEARNING,
	
	/// Amazon Mediaconnect.
	MEDIACONNECT,
	
	/// Amazon Mediaconvert.
	MEDIACONVERT,
	
	/// Amazon Mediapackage.
	MEDIAPACKAGE,
	
	/// Amazon Mediatailor.
	MEDIATAILOR,
	
	/// Amazon Mq.
	MQ,
	
	/// Amazon Msk.
	MSK,
	
	/// Amazon Nat Gateway.
	NATGATEWAY,
	
	/// Amazon Neptune.
	NEPTUNE,
	
	/// Amazon Ops Works.
	OPSWORKS,
	
	/// Amazon Polly.
	POLLY,
	
	/// Amazon Rds.
	RDS,
	
	/// Amazon Redshift.
	REDSHIFT,
	
	/// Amazon Rekognition.
	REKOGNITION,
	
	/// Amazon Route53.
	ROUTE53,
	
	/// Amazon S3.
	S3,
	
	/// Amazon Sagemaker.
	SAGEMAKER,
	
	/// Amazon Ses.
	SES,
	
	/// Amazon Shield.
	SHIELD,
	
	/// Amazon Sns.
	SNS,
	
	/// Amazon Sqs.
	SQS,
	
	/// Amazon Step Functions.
	STEPFUNCTIONS,
	
	/// Amazon Storage Gateway.
	STORAGEGATEWAY,
	
	/// Amazon Swf.
	SWF,
	
	/// Amazon Translate.
	TRANSLATE,
	
	/// Amazon Trusted Advisor.
	TRUSTEDADVISOR,
	
	/// Amazon Vpn.
	AWSVPN,
	
	/// Amazon Waf.
	WAF,
	
	/// Amazon Web Services.
	CLOUDWATCH,
	
	/// Amazon Workspaces.
	WORKSPACES,
	
	/// Amazon Xray.
	AWSXRAY,
	
	/// Ansible.
	ANSIBLE,
	
	/// Apache.
	APACHE,
	
	/// Api or My Apps.
	API,
	
	/// Azure.
	AZURE,
	
	/// Azure Analysis Services.
	AZUREANALYSISSERVICES,
	
	/// Azure Api Management.
	AZUREAPIMANAGEMENT,
	
	/// Azure App Service Environment.
	AZUREAPPSERVICEENVIRONMENT,
	
	/// Azure App Service Plan.
	AZUREAPPSERVICEPLAN,
	
	/// Azure App Services.
	AZUREAPPSERVICES,
	
	/// Azure Application Gateway.
	AZUREAPPLICATIONGATEWAY,
	
	/// Azure Automation.
	AZUREAUTOMATION,
	
	/// Azure Batch.
	AZUREBATCH,
	
	/// Azure Blob Storage.
	AZUREBLOBSTORAGE,
	
	/// Azure Cognitive Services.
	AZURECOGNITIVESERVICES,
	
	/// Azure Container Instances.
	AZURECONTAINERINSTANCES,
	
	/// Azure Container Service.
	AZURECONTAINERSERVICE,
	
	/// Azure Cosmosdb.
	AZURECOSMOSDB,
	
	/// Azure Customer Insights.
	AZURECUSTOMERINSIGHTS,
	
	/// Azure Data Factory.
	AZUREDATAFACTORY,
	
	/// Azure Data Lake Analytics.
	AZUREDATALAKEANALYTICS,
	
	/// Azure Data Lake Store.
	AZUREDATALAKESTORE,
	
	/// Azure Db For Mariadb.
	AZUREDBFORMARIADB,
	
	/// Azure Db For Mysql.
	AZUREDBFORMYSQL,
	
	/// Azure Db For Postgresql.
	AZUREDBFORPOSTGRESQL,
	
	/// Azure Event Grid.
	AZUREEVENTGRID,
	
	/// Azure Event Hub.
	AZUREEVENTHUB,
	
	/// Azure Express Route.
	AZUREEXPRESSROUTE,
	
	/// Azure File Storage.
	AZUREFILESTORAGE,
	
	/// Azure Hd Insight.
	AZUREHDINSIGHT,
	
	/// Azure Iot Hub.
	AZUREIOTHUB,
	
	/// Azure Key Vault.
	AZUREKEYVAULT,
	
	/// Azure Load Balancer.
	AZURELOADBALANCER,
	
	/// Azure Logic App.
	AZURELOGICAPP,
	
	/// Azure Network Interface.
	AZURENETWORKINTERFACE,
	
	/// Azure Notification Hubs.
	AZURENOTIFICATIONHUBS,
	
	/// Azure Public Ip Address.
	AZUREPUBLICIPADDRESS,
	
	/// Azure Queue Storage.
	AZUREQUEUESTORAGE,
	
	/// Azure Redis Cache.
	AZUREREDISCACHE,
	
	/// Azure Relay.
	AZURERELAY,
	
	/// Azure Search.
	AZURESEARCH,
	
	/// Azure Service Bus.
	AZURESERVICEBUS,
	
	/// Azure Sql Database.
	AZURESQLDATABASE,
	
	/// Azure Sql Elastic Pool.
	AZURESQLELASTICPOOL,
	
	/// Azure Storage.
	AZURESTORAGE,
	
	/// Azure Stream Analytics.
	AZURESTREAMANALYTICS,
	
	/// Azure Table Storage.
	AZURETABLESTORAGE,
	
	/// Azure Vm.
	AZUREVM,
	
	/// Azure Vm Scale Set.
	AZUREVMSCALESET,
	
	/// Bitbucket.
	BITBUCKET,
	
	/// Btrfs.
	BTRFS,
	
	/// Bugsnag.
	BUGSNAG,
	
	/// Cacti.
	CACTI,
	
	/// Campfire.
	CAMPFIRE,
	
	/// Capistrano.
	CAPISTRANO,
	
	/// Cassandra.
	CASSANDRA,
	
	/// Catchpoint.
	CATCHPOINT,
	
	/// Ceph.
	CEPH,
	
	/// Chatwork.
	CHATWORK,
	
	/// Chef.
	CHEF,
	
	/// Circleci.
	CIRCLECI,
	
	/// Cisco Aci.
	CISCOACI,
	
	/// Cloud Foundry.
	CLOUDFOUNDRY,
	
	/// Cloudflare.
	CLOUDFLARE,
	
	/// Cloudhealth.
	CLOUDHEALTH,
	
	/// Cloudnetworkhealth.
	CLOUDNETWORKHEALTH,
	
	/// Consul.
	CONSUL,
	
	/// Couchbase.
	COUCHBASE,
	
	/// Couchdb.
	COUCHDB,
	
	/// Datadog or System.
	SYSTEM,
	
	/// Desk.
	DESK,
	
	/// Dingtalk.
	DINGTALK,
	
	/// Docker.
	DOCKER,
	
	/// Dyn.
	DYN,
	
	/// Elasticsearch.
	ELASTICSEARCH,
	
	/// Etcd.
	ETCD,
	
	/// Event Viewer.
	EVENTVIEWER,
	
	/// Exceptional.
	EXCEPTIONAL,
	
	/// Express.
	EXPRESS,
	
	/// Fabric.
	FABRIC,
	
	/// Fastly.
	FASTLY,
	
	/// Feed.
	FEED,
	
	/// Flowdock.
	FLOWDOCK,
	
	/// Fluentd.
	FLUENTD,
	
	/// Ganglia.
	GANGLIA,
	
	/// Gearman.
	GEARMAN,
	
	/// Git.
	GIT,
	
	/// Github.
	GITHUB,
	
	/// Go Expvar.
	GOEXPVAR,
	
	/// Google App Engine.
	GAE,
	
	/// Google Cloud Apis.
	GCPAPIS,
	
	/// Google Cloud Big Query.
	GCPBIGQUERY,
	
	/// Google Cloud Bigtable.
	GCPBIGTABLE,
	
	/// Google Cloud Composer.
	GCPCOMPOSER,
	
	/// Google Cloud Dataflow.
	GCPDATAFLOW,
	
	/// Google Cloud Dataproc.
	GCPDATAPROC,
	
	/// Google Cloud Datastore.
	GCPDATASTORE,
	
	/// Google Cloud Filestore.
	GCPFILESTORE,
	
	/// Google Cloud Firebase.
	GCPFIREBASE,
	
	/// Google Cloud Firestore.
	GCPFIRESTORE,
	
	/// Google Cloud Functions.
	GCPCLOUDFUNCTIONS,
	
	/// Google Cloud Interconnect.
	GCPINTERCONNECT,
	
	/// Google Cloud Iot.
	GCPIOT,
	
	/// Google Cloud Loadbalancing.
	GCPLOADBALANCING,
	
	/// Google Cloud Ml.
	GCPML,
	
	/// Google Cloud Platform.
	GCP,
	
	/// Google Cloud Pubsub.
	GCPPUBSUB,
	
	/// Google Cloud Redis.
	GCPREDIS,
	
	/// Google Cloud Router.
	GCPROUTER,
	
	/// Google Cloud Run.
	GCPCLOUDRUN,
	
	/// Google Cloud Spanner.
	GCPSPANNER,
	
	/// Google Cloud Storage.
	GCPSTORAGE,
	
	/// Google Cloud Tasks.
	GCPTASKS,
	
	/// Google Cloud Tpu.
	GCPTPU,
	
	/// Google Cloud Vpn.
	GCPVPN,
	
	/// Google Cloudsql.
	GCPCLOUDSQL,
	
	/// Google Compute Engine.
	GCPCOMPUTE,
	
	/// Google Container Engine.
	GCPCONTAINER,
	
	/// Google Hangouts Chat.
	HANGOUTS,
	
	/// Google Stackdriver Logging.
	GCPLOGGING,
	
	/// Gunicorn.
	GUNICORN,
	
	/// Haproxy.
	HAPROXY,
	
	/// Hdfs.
	HDFS,
	
	/// Hipchat.
	HIPCHAT,
	
	/// Honeybadger.
	HONEYBADGER,
	
	/// Hp Cloud.
	HPCLOUD,
	
	/// Iis.
	IIS,
	
	/// Immunio.
	IMMUNIO,
	
	/// Java.
	JAVA,
	
	/// Hudson or Jenkins.
	JENKINS,
	
	/// Jira.
	JIRA,
	
	/// Kafka.
	KAFKA,
	
	/// Kong.
	KONG,
	
	/// Kubernetes.
	KUBERNETES,
	
	/// Kyoto Tycoon.
	KYOTOTYCOON,
	
	/// Lightbendrp.
	LIGHTBENDRP,
	
	/// Lighttpd.
	LIGHTTPD,
	
	/// Mapreduce.
	MAPREDUCE,
	
	/// Marathon.
	MARATHON,
	
	/// Marlo.
	MARLO,
	
	/// Memcached.
	MEMCACHED,
	
	/// Mesos.
	MESOS,
	
	/// Metric or Monitor Alert.
	ALERT,
	
	/// Microsoft Teams.
	MICROSOFTTEAMS,
	
	/// Mongodb.
	MONGODB,
	
	/// Mongodb Atlas.
	MONGODBATLAS,
	
	/// Moxtra.
	MOXTRA,
	
	/// Mparticle.
	MPARTICLE,
	
	/// Mysql.
	MYSQL,
	
	/// Nagios.
	NAGIOS,
	
	/// New Relic.
	NEWRELIC,
	
	/// Nginx.
	NGINX,
	
	/// Node.
	NODE,
	
	/// Office 365 Groups.
	OFFICE365GROUPS,
	
	/// Okta.
	OKTA,
	
	/// Openstack.
	OPENSTACK,
	
	/// Opsgenie.
	OPSGENIE,
	
	/// Opsmatic.
	OPSMATIC,
	
	/// Pagerduty.
	PAGERDUTY,
	
	/// Papertrail.
	PAPERTRAIL,
	
	/// Pgbouncer.
	PGBOUNCER,
	
	/// Php.
	PHP,
	
	/// Phpfpm.
	PHPFPM,
	
	/// Pingdom.
	PINGDOM,
	
	/// Pivotal.
	PIVOTAL,
	
	/// Postfix.
	POSTFIX,
	
	/// Postgres.
	POSTGRES,
	
	/// Powerdns Recursor.
	POWERDNSRECURSOR,
	
	/// Puppet.
	PUPPET,
	
	/// Pusher.
	PUSHER,
	
	/// Python.
	PYTHON,
	
	/// Rabbitmq.
	RABBITMQ,
	
	/// Redis.
	REDIS,
	
	/// Redmine.
	REDMINE,
	
	/// Riak.
	RIAK,
	
	/// Riakcs.
	RIAKCS,
	
	/// Rollbar.
	ROLLBAR,
	
	/// Ruby.
	RUBY,
	
	/// Segment.
	SEGMENT,
	
	/// Sentry.
	SENTRY,
	
	/// Servicenow.
	SERVICENOW,
	
	/// Sketch.
	SKETCH,
	
	/// Slack.
	SLACK,
	
	/// Snmp.
	SNMP,
	
	/// Solidfire.
	SOLIDFIRE,
	
	/// Solr.
	SOLR,
	
	/// Spark.
	SPARK,
	
	/// Splunk.
	SPLUNK,
	
	/// Sql Server.
	SQLSERVER,
	
	/// Ssh.
	SSH,
	
	/// Statuspage.
	STATUSPAGE,
	
	/// Stride.
	STRIDE,
	
	/// Sumo Logic.
	SUMOLOGIC,
	
	/// Supervisord.
	SUPERVISORD,
	
	/// Teamcity.
	TEAMCITY,
	
	/// Tokumx.
	TOKUMX,
	
	/// Tomcat.
	TOMCAT,
	
	/// Travis Ci.
	TRAVISCI,
	
	/// Upstart.
	UPSTART,
	
	/// Varnish.
	VARNISH,
	
	/// Verisign.
	VERISIGN,
	
	/// Verisign Openhybrid.
	VERISIGNOPENHYBRID,
	
	/// Victorops.
	VICTOROPS,
	
	/// Vsphere.
	VSPHERE,
	
	/// Watchdog.
	WATCHDOG,
	
	/// Webhooks.
	WEBHOOKS,
	
	/// Webmetrics.
	WEBMETRICS,
	
	/// Windows Service.
	WINDOWSSERVICE,
	
	/// Wmi.
	WMI,
	
	/// Xmatters.
	XMATTERS,
	
	/// Yarn.
	YARN,
	
	/// Zendesk.
	ZENDESK,
	
	/// Zookeeper.
	ZOOKEEPER,
}

impl SourceTypeName
{
	#[inline(always)]
	fn to_bytes(self) -> &'static [u8]
	{
		use self::SourceTypeName::*;
		
		match self
		{
			ACTIVEMQ => b"ACTIVEMQ" as &[u8],
			AIRBRAKE => b"AIRBRAKE" as &[u8],
			AKAMAIDATASTREAM => b"AKAMAIDATASTREAM" as &[u8],
			AKAMAIMPULSE => b"AKAMAIMPULSE" as &[u8],
			ALIBABACLOUD => b"ALIBABACLOUD" as &[u8],
			APIGATEWAY => b"APIGATEWAY" as &[u8],
			AMAZONAPPMESH => b"AMAZONAPPMESH" as &[u8],
			APPSTREAM => b"APPSTREAM" as &[u8],
			APPSYNC => b"APPSYNC" as &[u8],
			ATHENA => b"ATHENA" as &[u8],
			AUTOSCALING => b"AUTOSCALING" as &[u8],
			BILLING => b"BILLING" as &[u8],
			CLOUDFRONT => b"CLOUDFRONT" as &[u8],
			CLOUDSEARCH => b"CLOUDSEARCH" as &[u8],
			CLOUDTRAIL => b"CLOUDTRAIL" as &[u8],
			CODEBUILD => b"CODEBUILD" as &[u8],
			CODEDEPLOY => b"CODEDEPLOY" as &[u8],
			COGNITO => b"COGNITO" as &[u8],
			AWSCONNECT => b"AWSCONNECT" as &[u8],
			DIRECTCONNECT => b"DIRECTCONNECT" as &[u8],
			AWSDMS => b"AWSDMS" as &[u8],
			DOCUMENTDB => b"DOCUMENTDB" as &[u8],
			DYNAMODB => b"DYNAMODB" as &[u8],
			EBS => b"EBS" as &[u8],
			EC2 => b"EC2" as &[u8],
			EC2SPOT => b"EC2SPOT" as &[u8],
			ECS => b"ECS" as &[u8],
			EFS => b"EFS" as &[u8],
			ELASTICTRANSCODER => b"ELASTICTRANSCODER" as &[u8],
			ELASTICACHE => b"ELASTICACHE" as &[u8],
			ELASTICBEANSTALK => b"ELASTICBEANSTALK" as &[u8],
			ELB => b"ELB" as &[u8],
			EMR => b"EMR" as &[u8],
			ES => b"ES" as &[u8],
			AWSEVENTHUB => b"AWSEVENTHUB" as &[u8],
			FIREHOSE => b"FIREHOSE" as &[u8],
			GAMELIFT => b"GAMELIFT" as &[u8],
			GLUE => b"GLUE" as &[u8],
			HEALTH => b"HEALTH" as &[u8],
			INSPECTOR => b"INSPECTOR" as &[u8],
			IOT => b"IOT" as &[u8],
			KINESIS => b"KINESIS" as &[u8],
			KMS => b"KMS" as &[u8],
			LAMBDA => b"LAMBDA" as &[u8],
			AWSLEX => b"AWSLEX" as &[u8],
			MACHINELEARNING => b"MACHINELEARNING" as &[u8],
			MEDIACONNECT => b"MEDIACONNECT" as &[u8],
			MEDIACONVERT => b"MEDIACONVERT" as &[u8],
			MEDIAPACKAGE => b"MEDIAPACKAGE" as &[u8],
			MEDIATAILOR => b"MEDIATAILOR" as &[u8],
			MQ => b"MQ" as &[u8],
			MSK => b"MSK" as &[u8],
			NATGATEWAY => b"NATGATEWAY" as &[u8],
			NEPTUNE => b"NEPTUNE" as &[u8],
			OPSWORKS => b"OPSWORKS" as &[u8],
			POLLY => b"POLLY" as &[u8],
			RDS => b"RDS" as &[u8],
			REDSHIFT => b"REDSHIFT" as &[u8],
			REKOGNITION => b"REKOGNITION" as &[u8],
			ROUTE53 => b"ROUTE53" as &[u8],
			S3 => b"S3" as &[u8],
			SAGEMAKER => b"SAGEMAKER" as &[u8],
			SES => b"SES" as &[u8],
			SHIELD => b"SHIELD" as &[u8],
			SNS => b"SNS" as &[u8],
			SQS => b"SQS" as &[u8],
			STEPFUNCTIONS => b"STEPFUNCTIONS" as &[u8],
			STORAGEGATEWAY => b"STORAGEGATEWAY" as &[u8],
			SWF => b"SWF" as &[u8],
			TRANSLATE => b"TRANSLATE" as &[u8],
			TRUSTEDADVISOR => b"TRUSTEDADVISOR" as &[u8],
			AWSVPN => b"AWSVPN" as &[u8],
			WAF => b"WAF" as &[u8],
			CLOUDWATCH => b"CLOUDWATCH" as &[u8],
			WORKSPACES => b"WORKSPACES" as &[u8],
			AWSXRAY => b"AWSXRAY" as &[u8],
			ANSIBLE => b"ANSIBLE" as &[u8],
			APACHE => b"APACHE" as &[u8],
			API => b"API" as &[u8],
			AZURE => b"AZURE" as &[u8],
			AZUREANALYSISSERVICES => b"AZUREANALYSISSERVICES" as &[u8],
			AZUREAPIMANAGEMENT => b"AZUREAPIMANAGEMENT" as &[u8],
			AZUREAPPSERVICEENVIRONMENT => b"AZUREAPPSERVICEENVIRONMENT" as &[u8],
			AZUREAPPSERVICEPLAN => b"AZUREAPPSERVICEPLAN" as &[u8],
			AZUREAPPSERVICES => b"AZUREAPPSERVICES" as &[u8],
			AZUREAPPLICATIONGATEWAY => b"AZUREAPPLICATIONGATEWAY" as &[u8],
			AZUREAUTOMATION => b"AZUREAUTOMATION" as &[u8],
			AZUREBATCH => b"AZUREBATCH" as &[u8],
			AZUREBLOBSTORAGE => b"AZUREBLOBSTORAGE" as &[u8],
			AZURECOGNITIVESERVICES => b"AZURECOGNITIVESERVICES" as &[u8],
			AZURECONTAINERINSTANCES => b"AZURECONTAINERINSTANCES" as &[u8],
			AZURECONTAINERSERVICE => b"AZURECONTAINERSERVICE" as &[u8],
			AZURECOSMOSDB => b"AZURECOSMOSDB" as &[u8],
			AZURECUSTOMERINSIGHTS => b"AZURECUSTOMERINSIGHTS" as &[u8],
			AZUREDATAFACTORY => b"AZUREDATAFACTORY" as &[u8],
			AZUREDATALAKEANALYTICS => b"AZUREDATALAKEANALYTICS" as &[u8],
			AZUREDATALAKESTORE => b"AZUREDATALAKESTORE" as &[u8],
			AZUREDBFORMARIADB => b"AZUREDBFORMARIADB" as &[u8],
			AZUREDBFORMYSQL => b"AZUREDBFORMYSQL" as &[u8],
			AZUREDBFORPOSTGRESQL => b"AZUREDBFORPOSTGRESQL" as &[u8],
			AZUREEVENTGRID => b"AZUREEVENTGRID" as &[u8],
			AZUREEVENTHUB => b"AZUREEVENTHUB" as &[u8],
			AZUREEXPRESSROUTE => b"AZUREEXPRESSROUTE" as &[u8],
			AZUREFILESTORAGE => b"AZUREFILESTORAGE" as &[u8],
			AZUREHDINSIGHT => b"AZUREHDINSIGHT" as &[u8],
			AZUREIOTHUB => b"AZUREIOTHUB" as &[u8],
			AZUREKEYVAULT => b"AZUREKEYVAULT" as &[u8],
			AZURELOADBALANCER => b"AZURELOADBALANCER" as &[u8],
			AZURELOGICAPP => b"AZURELOGICAPP" as &[u8],
			AZURENETWORKINTERFACE => b"AZURENETWORKINTERFACE" as &[u8],
			AZURENOTIFICATIONHUBS => b"AZURENOTIFICATIONHUBS" as &[u8],
			AZUREPUBLICIPADDRESS => b"AZUREPUBLICIPADDRESS" as &[u8],
			AZUREQUEUESTORAGE => b"AZUREQUEUESTORAGE" as &[u8],
			AZUREREDISCACHE => b"AZUREREDISCACHE" as &[u8],
			AZURERELAY => b"AZURERELAY" as &[u8],
			AZURESEARCH => b"AZURESEARCH" as &[u8],
			AZURESERVICEBUS => b"AZURESERVICEBUS" as &[u8],
			AZURESQLDATABASE => b"AZURESQLDATABASE" as &[u8],
			AZURESQLELASTICPOOL => b"AZURESQLELASTICPOOL" as &[u8],
			AZURESTORAGE => b"AZURESTORAGE" as &[u8],
			AZURESTREAMANALYTICS => b"AZURESTREAMANALYTICS" as &[u8],
			AZURETABLESTORAGE => b"AZURETABLESTORAGE" as &[u8],
			AZUREVM => b"AZUREVM" as &[u8],
			AZUREVMSCALESET => b"AZUREVMSCALESET" as &[u8],
			BITBUCKET => b"BITBUCKET" as &[u8],
			BTRFS => b"BTRFS" as &[u8],
			BUGSNAG => b"BUGSNAG" as &[u8],
			CACTI => b"CACTI" as &[u8],
			CAMPFIRE => b"CAMPFIRE" as &[u8],
			CAPISTRANO => b"CAPISTRANO" as &[u8],
			CASSANDRA => b"CASSANDRA" as &[u8],
			CATCHPOINT => b"CATCHPOINT" as &[u8],
			CEPH => b"CEPH" as &[u8],
			CHATWORK => b"CHATWORK" as &[u8],
			CHEF => b"CHEF" as &[u8],
			CIRCLECI => b"CIRCLECI" as &[u8],
			CISCOACI => b"CISCOACI" as &[u8],
			CLOUDFOUNDRY => b"CLOUDFOUNDRY" as &[u8],
			CLOUDFLARE => b"CLOUDFLARE" as &[u8],
			CLOUDHEALTH => b"CLOUDHEALTH" as &[u8],
			CLOUDNETWORKHEALTH => b"CLOUDNETWORKHEALTH" as &[u8],
			CONSUL => b"CONSUL" as &[u8],
			COUCHBASE => b"COUCHBASE" as &[u8],
			COUCHDB => b"COUCHDB" as &[u8],
			SYSTEM => b"SYSTEM" as &[u8],
			DESK => b"DESK" as &[u8],
			DINGTALK => b"DINGTALK" as &[u8],
			DOCKER => b"DOCKER" as &[u8],
			DYN => b"DYN" as &[u8],
			ELASTICSEARCH => b"ELASTICSEARCH" as &[u8],
			ETCD => b"ETCD" as &[u8],
			EVENTVIEWER => b"EVENTVIEWER" as &[u8],
			EXCEPTIONAL => b"EXCEPTIONAL" as &[u8],
			EXPRESS => b"EXPRESS" as &[u8],
			FABRIC => b"FABRIC" as &[u8],
			FASTLY => b"FASTLY" as &[u8],
			FEED => b"FEED" as &[u8],
			FLOWDOCK => b"FLOWDOCK" as &[u8],
			FLUENTD => b"FLUENTD" as &[u8],
			GANGLIA => b"GANGLIA" as &[u8],
			GEARMAN => b"GEARMAN" as &[u8],
			GIT => b"GIT" as &[u8],
			GITHUB => b"GITHUB" as &[u8],
			GOEXPVAR => b"GOEXPVAR" as &[u8],
			GAE => b"GAE" as &[u8],
			GCPAPIS => b"GCPAPIS" as &[u8],
			GCPBIGQUERY => b"GCPBIGQUERY" as &[u8],
			GCPBIGTABLE => b"GCPBIGTABLE" as &[u8],
			GCPCOMPOSER => b"GCPCOMPOSER" as &[u8],
			GCPDATAFLOW => b"GCPDATAFLOW" as &[u8],
			GCPDATAPROC => b"GCPDATAPROC" as &[u8],
			GCPDATASTORE => b"GCPDATASTORE" as &[u8],
			GCPFILESTORE => b"GCPFILESTORE" as &[u8],
			GCPFIREBASE => b"GCPFIREBASE" as &[u8],
			GCPFIRESTORE => b"GCPFIRESTORE" as &[u8],
			GCPCLOUDFUNCTIONS => b"GCPCLOUDFUNCTIONS" as &[u8],
			GCPINTERCONNECT => b"GCPINTERCONNECT" as &[u8],
			GCPIOT => b"GCPIOT" as &[u8],
			GCPLOADBALANCING => b"GCPLOADBALANCING" as &[u8],
			GCPML => b"GCPML" as &[u8],
			GCP => b"GCP" as &[u8],
			GCPPUBSUB => b"GCPPUBSUB" as &[u8],
			GCPREDIS => b"GCPREDIS" as &[u8],
			GCPROUTER => b"GCPROUTER" as &[u8],
			GCPCLOUDRUN => b"GCPCLOUDRUN" as &[u8],
			GCPSPANNER => b"GCPSPANNER" as &[u8],
			GCPSTORAGE => b"GCPSTORAGE" as &[u8],
			GCPTASKS => b"GCPTASKS" as &[u8],
			GCPTPU => b"GCPTPU" as &[u8],
			GCPVPN => b"GCPVPN" as &[u8],
			GCPCLOUDSQL => b"GCPCLOUDSQL" as &[u8],
			GCPCOMPUTE => b"GCPCOMPUTE" as &[u8],
			GCPCONTAINER => b"GCPCONTAINER" as &[u8],
			HANGOUTS => b"HANGOUTS" as &[u8],
			GCPLOGGING => b"GCPLOGGING" as &[u8],
			GUNICORN => b"GUNICORN" as &[u8],
			HAPROXY => b"HAPROXY" as &[u8],
			HDFS => b"HDFS" as &[u8],
			HIPCHAT => b"HIPCHAT" as &[u8],
			HONEYBADGER => b"HONEYBADGER" as &[u8],
			HPCLOUD => b"HPCLOUD" as &[u8],
			IIS => b"IIS" as &[u8],
			IMMUNIO => b"IMMUNIO" as &[u8],
			JAVA => b"JAVA" as &[u8],
			JENKINS => b"JENKINS" as &[u8],
			JIRA => b"JIRA" as &[u8],
			KAFKA => b"KAFKA" as &[u8],
			KONG => b"KONG" as &[u8],
			KUBERNETES => b"KUBERNETES" as &[u8],
			KYOTOTYCOON => b"KYOTOTYCOON" as &[u8],
			LIGHTBENDRP => b"LIGHTBENDRP" as &[u8],
			LIGHTTPD => b"LIGHTTPD" as &[u8],
			MAPREDUCE => b"MAPREDUCE" as &[u8],
			MARATHON => b"MARATHON" as &[u8],
			MARLO => b"MARLO" as &[u8],
			MEMCACHED => b"MEMCACHED" as &[u8],
			MESOS => b"MESOS" as &[u8],
			ALERT => b"ALERT" as &[u8],
			MICROSOFTTEAMS => b"MICROSOFTTEAMS" as &[u8],
			MONGODB => b"MONGODB" as &[u8],
			MONGODBATLAS => b"MONGODBATLAS" as &[u8],
			MOXTRA => b"MOXTRA" as &[u8],
			MPARTICLE => b"MPARTICLE" as &[u8],
			MYSQL => b"MYSQL" as &[u8],
			NAGIOS => b"NAGIOS" as &[u8],
			NEWRELIC => b"NEWRELIC" as &[u8],
			NGINX => b"NGINX" as &[u8],
			NODE => b"NODE" as &[u8],
			OFFICE365GROUPS => b"OFFICE365GROUPS" as &[u8],
			OKTA => b"OKTA" as &[u8],
			OPENSTACK => b"OPENSTACK" as &[u8],
			OPSGENIE => b"OPSGENIE" as &[u8],
			OPSMATIC => b"OPSMATIC" as &[u8],
			PAGERDUTY => b"PAGERDUTY" as &[u8],
			PAPERTRAIL => b"PAPERTRAIL" as &[u8],
			PGBOUNCER => b"PGBOUNCER" as &[u8],
			PHP => b"PHP" as &[u8],
			PHPFPM => b"PHPFPM" as &[u8],
			PINGDOM => b"PINGDOM" as &[u8],
			PIVOTAL => b"PIVOTAL" as &[u8],
			POSTFIX => b"POSTFIX" as &[u8],
			POSTGRES => b"POSTGRES" as &[u8],
			POWERDNSRECURSOR => b"POWERDNSRECURSOR" as &[u8],
			PUPPET => b"PUPPET" as &[u8],
			PUSHER => b"PUSHER" as &[u8],
			PYTHON => b"PYTHON" as &[u8],
			RABBITMQ => b"RABBITMQ" as &[u8],
			REDIS => b"REDIS" as &[u8],
			REDMINE => b"REDMINE" as &[u8],
			RIAK => b"RIAK" as &[u8],
			RIAKCS => b"RIAKCS" as &[u8],
			ROLLBAR => b"ROLLBAR" as &[u8],
			RUBY => b"RUBY" as &[u8],
			SEGMENT => b"SEGMENT" as &[u8],
			SENTRY => b"SENTRY" as &[u8],
			SERVICENOW => b"SERVICENOW" as &[u8],
			SKETCH => b"SKETCH" as &[u8],
			SLACK => b"SLACK" as &[u8],
			SNMP => b"SNMP" as &[u8],
			SOLIDFIRE => b"SOLIDFIRE" as &[u8],
			SOLR => b"SOLR" as &[u8],
			SPARK => b"SPARK" as &[u8],
			SPLUNK => b"SPLUNK" as &[u8],
			SQLSERVER => b"SQLSERVER" as &[u8],
			SSH => b"SSH" as &[u8],
			STATUSPAGE => b"STATUSPAGE" as &[u8],
			STRIDE => b"STRIDE" as &[u8],
			SUMOLOGIC => b"SUMOLOGIC" as &[u8],
			SUPERVISORD => b"SUPERVISORD" as &[u8],
			TEAMCITY => b"TEAMCITY" as &[u8],
			TOKUMX => b"TOKUMX" as &[u8],
			TOMCAT => b"TOMCAT" as &[u8],
			TRAVISCI => b"TRAVISCI" as &[u8],
			UPSTART => b"UPSTART" as &[u8],
			VARNISH => b"VARNISH" as &[u8],
			VERISIGN => b"VERISIGN" as &[u8],
			VERISIGNOPENHYBRID => b"VERISIGNOPENHYBRID" as &[u8],
			VICTOROPS => b"VICTOROPS" as &[u8],
			VSPHERE => b"VSPHERE" as &[u8],
			WATCHDOG => b"WATCHDOG" as &[u8],
			WEBHOOKS => b"WEBHOOKS" as &[u8],
			WEBMETRICS => b"WEBMETRICS" as &[u8],
			WINDOWSSERVICE => b"WINDOWSSERVICE" as &[u8],
			WMI => b"WMI" as &[u8],
			XMATTERS => b"XMATTERS" as &[u8],
			YARN => b"YARN" as &[u8],
			ZENDESK => b"ZENDESK" as &[u8],
			ZOOKEEPER => b"ZOOKEEPER" as &[u8],
		}
	}
}
