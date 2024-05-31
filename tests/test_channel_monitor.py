# Copyright (c) 2024 evopro Innovation

from pytest_bdd import scenarios, given, when, then, parsers
from epics import caget
import re

# Find and read all scenarios from specified file
# https://pytest-bdd.readthedocs.io/en/stable/#scenarios-shortcut
scenarios("channel_monitor.feature")


@given("The module is accessible", target_fixture="module")
def module_accessible():
    return {}


@when(parsers.parse("The client read the {param} value for {channel:d}"))
def read_channel_param(module, param, channel):
    pv, as_string = param2pv(param)
    module["answer"] = caget(f"Module00:Ch{channel}:{pv}", as_string=as_string)


def param2pv(param):
    match param:
        case "VSET": return "DrivenVoltage", False
        case "VMON": return "MonitoredVoltage", False
        case "ISET": return "DrivenCurrent", False
        case "IMON": return "MonitoredCurrent", False
        case "IMON RANGE": return "CurrentMonitoringRange", True
        case "MAX VSET": return "MaxVoltage", False
        case "RAMP UP": return "RampUp", False
        case "RAMP DOWN": return "RampDown", False
        case "TRIP": return "Trip", False
        case "POWER DOWN": return "PowerDownMode", True
        case "POL": return "Polarity", True
        case _: return param, True


@then(parsers.parse("The answer should be '{value}'"))
def check_imrange_value(module, value):
    assert module["answer"] == value


@then("The answer should contain one floating point value")
def check_float_value(module):
    assert isinstance(module["answer"], float)


@then("The answer should contain one integer value")
def check_integer_value(module):
    assert isinstance(module["answer"], int)


@then(parsers.parse("The answer should be '{value1}' or '{value2}'"))
def check_binary_answer(module, value1, value2):
    assert module["answer"] == value1 or module["answer"] == value2
