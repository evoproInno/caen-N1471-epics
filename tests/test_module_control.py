# Copyright (c) 2024 evopro Innovation

from pytest_bdd import scenario, given, when, then, parsers
from epics import caget, caput
from time import sleep


@scenario('module_control.feature', 'Set Interlock mode')
def test_set_interlock():
    pass


@given("The module is accessible")
def module_accessible():
    pass


@given("The module is in remote control mode")
def module_is_in_remote():
    pass


@when(parsers.parse("The client set the interlock mode to {mode}"))
def set_interlock_mode(mode):
    caput("Module00:SetInterlockMode", mode)


@then(parsers.parse("The interlock mode should be {mode} within {timeout:d} seconds"))
def check_interlock_mode(mode, timeout):
    set_mode = None
    timer = 0
    while timer < timeout and set_mode != mode:
        timer += 1
        set_mode = caget("Module00:InterlockMode", as_string=True)
        sleep(1)

    assert set_mode == mode
