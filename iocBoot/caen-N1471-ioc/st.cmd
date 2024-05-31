#!../../bin/linux-x86_64/caen-N1471-ioc

#- You may have to change caen-N1471-ioc to something else
#- everywhere it appears in this file

< envPaths

# Configure these as necessary
epicsEnvSet ("CAEN_MODULE_ADDR","00")
epicsEnvSet ("CAEN_SERIAL_PORT","/dev/simulator_port")

# Internal variables
epicsEnvSet ("CAEN_PORT_NAME","PSU")
epicsEnvSet ("STREAM_PROTOCOL_PATH","$(TOP)/db")

cd "${TOP}"

## Register all support components
dbLoadDatabase "dbd/caen-N1471-ioc.dbd"
caen_N1471_ioc_registerRecordDeviceDriver pdbbase

drvAsynSerialPortConfigure("$(CAEN_PORT_NAME)", "$(CAEN_SERIAL_PORT)")

## Load record instances
dbLoadRecords("db/caen-N1471-timing.db" "PN=$(CAEN_PORT_NAME),ADDR=$(CAEN_MODULE_ADDR)")
dbLoadRecords("db/caen-N1471-module-monitor.db" "PN=$(CAEN_PORT_NAME),ADDR=$(CAEN_MODULE_ADDR)")
dbLoadRecords("db/caen-N1471-module-control.db" "PN=$(CAEN_PORT_NAME),ADDR=$(CAEN_MODULE_ADDR)")
dbLoadTemplate("db/caen-N1471-module-alarm-bits.subs" "ADDR=$(CAEN_MODULE_ADDR)")

# TODO: How to load these in a loop based on a single CAEN_CHANNEL_NUM=2 variable?
dbLoadRecords("db/caen-N1471-channel-control.db" "PN=$(CAEN_PORT_NAME),ADDR=$(CAEN_MODULE_ADDR),CH=0")
dbLoadRecords("db/caen-N1471-channel-control.db" "PN=$(CAEN_PORT_NAME),ADDR=$(CAEN_MODULE_ADDR),CH=1")
dbLoadRecords("db/caen-N1471-channel-monitor.db" "PN=$(CAEN_PORT_NAME),ADDR=$(CAEN_MODULE_ADDR),CH=0")
dbLoadRecords("db/caen-N1471-channel-monitor.db" "PN=$(CAEN_PORT_NAME),ADDR=$(CAEN_MODULE_ADDR),CH=1")
dbLoadTemplate("db/caen-N1471-channel-status-bits.subs" "ADDR=$(CAEN_MODULE_ADDR),CH=0")
dbLoadTemplate("db/caen-N1471-channel-status-bits.subs" "ADDR=$(CAEN_MODULE_ADDR),CH=1")

cd "${TOP}/iocBoot/${IOC}"
iocInit

## Start any sequence programs
#seq sncxxx,"user=ioc-user"
