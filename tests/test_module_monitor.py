# Copyright (c) 2024 evopro Innovation

from pytest_bdd import scenarios, given, when, then, parsers
from epics import caget
import re

# Find and read all scenarios from specified file
# https://pytest-bdd.readthedocs.io/en/stable/#scenarios-shortcut
scenarios("module_monitor.feature")


@given("The module is accessible", target_fixture="module")
def module_accessible():
    return {}


@when("The client read the module name")
def read_module_name(module):
    module["name"] = caget("Module00:Name")


@when("The client read the number of channels")
def read_module_name(module):
    module["channels"] = caget("Module00:NumberOfChannels")


@when("The client read the firmware release")
def read_module_name(module):
    module["fw"] = caget("Module00:FirmwareRelease")


@when("The client read the serial number")
def read_serial_number(module):
    module["serial"] = caget("Module00:SerialNumber")


@when("The client read the interlock status")
def read_interlock_status(module):
    module["answer"] = caget("Module00:InterlockStatus", as_string=True)


@when("The client read the interlock mode")
def read_interlock_mode(module):
    module["answer"] = caget("Module00:InterlockMode", as_string=True)


@when("The client read the control mode")
def read_control_mode(module):
    module["answer"] = caget("Module00:ControlMode", as_string=True)


@when("The client read the local bus termination")
def read_local_bus_termination(module):
    module["answer"] = caget("Module00:LocalBusTermination", as_string=True)


@when(parsers.parse("The client read the channel alarm status for channel {index:d}"))
def read_channel_alarm(module, index):
    module["answer"] = caget(f"Module00:Channel{index}Alarm", as_string=True)


@when("The client read the board in POWER FAIL alarm status")
def read_power_fail_alarm(module):
    module["answer"] = caget("Module00:PowerFailAlarm", as_string=True)


@when("The client read the board in OVER POWER alarm status")
def read_over_power_alarm(module):
    module["answer"] = caget("Module00:OverPowerAlarm", as_string=True)


@when("The client read the Internal HV Clock FAIL alarm status")
def read_hv_clock_fail_alarm(module):
    module["answer"] = caget("Module00:HvClockFailAlarm", as_string=True)


@then(parsers.parse("The answer should be equal to '{name}'"))
def check_module_name(module, name):
    assert module["name"] == name


@then(parsers.parse("The answer should be equal to {channels:d}"))
def check_number_of_channels(module, channels):
    assert module["channels"] == channels


@then("The answer should not be empty")
def check_firmware(module):
    assert module["fw"] is not None


@then("The answer should be formatted as 'XXXXX'")
def check_serial_format(module):
    assert re.match(r'\d{5}', module["serial"])


@then(parsers.parse("The answer should be '{value1}' or '{value2}'"))
def check_binary_answer(module, value1, value2):
    assert module["answer"] == value1 or module["answer"] == value2
