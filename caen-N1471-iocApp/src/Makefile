TOP=../..

include $(TOP)/configure/CONFIG
#----------------------------------------
#  ADD MACRO DEFINITIONS AFTER THIS LINE
#=============================

#=============================
# Build the IOC application

PROD_IOC = caen-N1471-ioc
# caen-N1471-ioc.dbd will be created and installed
DBD += caen-N1471-ioc.dbd

# Install xxxSupport.dbd into <top>/dbd
DBD += xxxSupport.dbd

# caen-N1471-ioc.dbd will be made up from these files:
caen-N1471-ioc_DBD += base.dbd

# Include dbd files from all support applications:
caen-N1471-ioc_DBD += xxxSupport.dbd

# Add PVAccess server
caen-N1471-ioc_DBD += PVAServerRegister.dbd
caen-N1471-ioc_DBD += qsrv.dbd

# Add all the support libraries needed by this IOC
caen-N1471-ioc_LIBS += stream
caen-N1471-ioc_LIBS += asyn
caen-N1471-ioc_LIBS += qsrv

# caen-N1471-ioc_registerRecordDeviceDriver.cpp derives from caen-N1471-ioc.dbd
caen-N1471-ioc_SRCS += caen-N1471-ioc_registerRecordDeviceDriver.cpp

# Build the main IOC entry point on workstation OSs.
caen-N1471-ioc_SRCS_DEFAULT += caen-N1471-iocMain.cpp
caen-N1471-ioc_SRCS_vxWorks += -nil-

# Add support from base/src/vxWorks if needed
#caen-N1471-ioc_OBJS_vxWorks += $(EPICS_BASE_BIN)/vxComLibrary

# Finally link to the EPICS Base libraries
caen-N1471-ioc_LIBS += $(EPICS_BASE_IOC_LIBS)
caen-N1471-ioc_LIBS += $(EPICS_BASE_PVA_CORE_LIBS)

#===========================

include $(TOP)/configure/RULES
#----------------------------------------
#  ADD RULES AFTER THIS LINE

