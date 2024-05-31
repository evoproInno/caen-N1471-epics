# Copyright (c) 2024 evopro Innovation

TOP=../..
include $(TOP)/configure/CONFIG
#----------------------------------------
#  ADD MACRO DEFINITIONS AFTER THIS LINE

#----------------------------------------------------
# Create and install (or just install) into <top>/db
# databases, templates, substitutions like this
DB += caen-N1471-timing.db
DB += caen-N1471-module-monitor.db
DB += caen-N1471-module-control.db
DB += caen-N1471-channel-monitor.db
DB += caen-N1471-channel-control.db
DB += caen-N1471-statusbit.template
DB += caen-N1471-module-alarm-bits.subs
DB += caen-N1471-channel-status-bits.subs
DB += caen-N1471.proto


#----------------------------------------------------
# If <anyname>.db template is not named <anyname>*.template add
# <anyname>_template = <templatename>

include $(TOP)/configure/RULES
#----------------------------------------
#  ADD RULES AFTER THIS LINE

