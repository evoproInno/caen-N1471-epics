# Copyright (c) 2024 evopro Innovation

Feature: Module level control
    The IOC can set module level parameters

    "module" refers to the Caen N1471 power supply
    "client" refers to channel access or pv access client

    Scenario: Set Interlock mode
        Given The module is accessible
        And The module is in remote control mode

        When The client set the interlock mode to <mode>

        Then The interlock mode should be <mode> within 10 seconds

        Examples:
        |  mode  |
        | CLOSED |
        |  OPEN  |

    Scenario: Clear alarm signal - no alarm condition
        Given The module is accessible
        And The module is in remote control mode
        And The alarm signal is active
        And The alarm condition is gone

        When The client clear the alarm signal

        Then The alarm signal should be inactive within <set interval>

    Scenario: Clear alarm signal - active alarm condition
        Given The module is accessible
        And The module is in remote control mode
        And The alarm signal is active
        And The alarm condition is active

        When The client clear the alarm signal

        Then The alarm signal should be active